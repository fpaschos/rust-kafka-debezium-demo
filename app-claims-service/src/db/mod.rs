use sqlx::{Executor, Postgres, Transaction};
use sqlx::types::Json;
use crate::model::ClaimDb;

// For implementation details of these functions
// see examples:
//  - https://github.com/govinda-attal/app-a/blob/main/src/db/functions.rs
//  - https://github.com/launchbadge/sqlx/blob/main/examples/postgres/transaction/src/main.rs#L3-

pub async fn fetch_claims(con: impl Executor<'_, Database=Postgres>) -> anyhow::Result<Vec<ClaimDb>> {
    let rows: Vec<ClaimDb> = sqlx::query_as(r#"SELECT id, involved FROM claim"#)
        .fetch_all(con)
        .await?;

    Ok(rows)
}

pub async fn fetch_claim(con: impl Executor<'_, Database=Postgres>, id: i64) -> anyhow::Result<Option<ClaimDb>> {
    let row: Option<ClaimDb> = sqlx::query_as(r#"SELECT * FROM claim WHERE id = $1"#)
        .bind(id)
        .fetch_optional(con)
        .await?;

    Ok(row)
}

pub async fn create_claim(tx: &mut Transaction<'_, Postgres>, c: ClaimDb) -> anyhow::Result<ClaimDb> {
    let row = sqlx::query_as::<_, ClaimDb>(
        r#"INSERT INTO claim (involved) VALUES ($1) RETURNING *"#,
    )
        .bind(Json(c.involved))

        // In 0.7, `Transaction` can no longer implement `Executor` directly,
        // so it must be dereferenced to the internal connection type.
        .fetch_one(&mut **tx)
        .await?;
    Ok(row)
}

pub async fn update_claim(tx: &mut Transaction<'_, Postgres>, c: ClaimDb) -> anyhow::Result<ClaimDb> {
    let row = sqlx::query_as::<_, ClaimDb>(
        r#"UPDATE claim SET involved = $1 WHERE id = $2 RETURNING *"#,
    )
        .bind(Json(c.involved))
        .bind(c.id)
        .fetch_one(&mut **tx)
        .await?;
    Ok(row)
}
