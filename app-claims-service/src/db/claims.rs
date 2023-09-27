use crate::db::PostgresTx;
use crate::model::ClaimDb;
use sqlx::{Executor, Postgres};

// For implementation details of these functions
// see examples:
//  - https://github.com/govinda-attal/app-a/blob/main/src/db/functions.rs
//  - https://github.com/launchbadge/sqlx/blob/main/examples/postgres/transaction/src/main.rs#L3-

pub async fn fetch_all(
    con: impl Executor<'_, Database = Postgres>,
) -> anyhow::Result<Vec<ClaimDb>> {
    let rows: Vec<ClaimDb> =
        sqlx::query_as(r#"SELECT id, claim_no, incident_type, status FROM claim"#)
            .fetch_all(con)
            .await?;

    Ok(rows)
}

pub async fn fetch_one(
    con: impl Executor<'_, Database = Postgres>,
    id: i32,
) -> anyhow::Result<Option<ClaimDb>> {
    let row: Option<ClaimDb> =
        sqlx::query_as(r#"SELECT id, claim_no, incident_type, status FROM claim WHERE id = $1"#)
            .bind(id)
            .fetch_optional(con)
            .await?;

    Ok(row)
}

pub async fn create(tx: &mut PostgresTx<'_>, c: ClaimDb) -> anyhow::Result<ClaimDb> {
    let row: ClaimDb = sqlx::query_as(
        r#"INSERT INTO claim (claim_no, incident_type, status) VALUES ($1, $2, $3) RETURNING *"#,
    )
    .bind(c.claim_no)
    .bind(c.incident_type)
    .bind(c.status)
    // In 0.7, `Transaction` can no longer implement `Executor` directly,
    // so it must be de referenced to the internal connection type.
    .fetch_one(&mut **tx)
    .await?;
    Ok(row)
}

pub async fn update(tx: &mut PostgresTx<'_>, c: ClaimDb) -> anyhow::Result<ClaimDb> {
    let row: ClaimDb = sqlx::query_as(
        r#"UPDATE claim SET incident_type = $1, status = $2 WHERE id = $3 RETURNING *"#,
    )
    .bind(c.incident_type)
    .bind(c.status)
    .bind(c.id)
    .fetch_one(&mut **tx)
    .await?;
    Ok(row)
}
