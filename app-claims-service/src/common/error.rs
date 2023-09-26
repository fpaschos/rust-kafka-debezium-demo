use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum AppError {
    DbError(DbError),
    UnhandledDbError(Arc<sqlx::Error>),
    Unhandled(Arc<anyhow::Error>),
}

#[derive(Clone, Copy, Debug)]
pub enum DbError {
    NotFound,
    #[allow(unused)]
    Conflict,
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::UnhandledDbError(Arc::new(e))
    }
}

impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self {
        AppError::Unhandled(Arc::new(e))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Map the error into error message, log message and status code
        let (error_message, log_message, status_code) = match_error(&self);

        // Log message depending on status code
        if status_code == StatusCode::INTERNAL_SERVER_ERROR {
            // alternatively log "self" instead of the "log_message"
            tracing::error!("{:?}", log_message);
        } else if status_code == StatusCode::NOT_FOUND {
            tracing::warn!("{:?}", log_message);
        }

        // Build response body with error message
        let body = Json(json!({ "error": error_message }));

        (status_code, body).into_response()
    }
}

pub fn match_error(error: &AppError) -> (&str, String, StatusCode) {
    match error {
        AppError::DbError(e) => match e {
            DbError::Conflict => ("Conflict", "Outdated resource".into(), StatusCode::CONFLICT),
            DbError::NotFound => (
                "Not found",
                "DB Entry not found".into(),
                StatusCode::NOT_FOUND,
            ),
        },
        AppError::UnhandledDbError(e) => (
            "Internal Server Error",
            format!("{:?}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        AppError::Unhandled(e) => (
            "Internal Server Error",
            format!("{:?}", e),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (error_message, log_message, status_code) = match_error(self);
        // This is a workaround to log error details from graphql requests
        if status_code == StatusCode::INTERNAL_SERVER_ERROR {
            tracing::error!("{:?}", log_message);
        }
        write!(f, "{}", error_message)
    }
}

impl std::error::Error for AppError {}
