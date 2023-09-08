use sqlx::{Executor, Postgres, Transaction};
use crate::model::PartyDb;


pub async fn fetch_one(con: impl Executor<'_, Database=Postgres>, id: i32) -> anyhow::Result<Option<PartyDb>> {
    let row: Option<PartyDb> = sqlx::query_as(r#"SELECT id, claim_id, "type", subtype, data FROM party WHERE id = $1"#)
        .bind(id)
        .fetch_optional(con)
        .await?;

    Ok(row)
}

pub async fn delete(tx: &mut Transaction<'_, Postgres>, p: PartyDb) -> anyhow::Result<PartyDb> {
    let row: PartyDb = sqlx::query_as(
        r#"DELETE FROM party WHERE id = $1 AND claim_id = $2 RETURNING *"#,
    )
        .bind(p.id)
        .bind(p.claim_id)

        // In 0.7, `Transaction` can no longer implement `Executor` directly,
        // so it must be de referenced to the internal connection type.
        .fetch_one(&mut **tx)
        .await?;
    Ok(row)
}

pub async fn create(tx: &mut Transaction<'_, Postgres>, p: PartyDb) -> anyhow::Result<PartyDb> {
    let row: PartyDb = sqlx::query_as(
        r#"INSERT INTO party (claim_id, "type", subtype, data) VALUES ($1, $2, $3, $4) RETURNING *"#,
    )
        .bind(p.claim_id)
        .bind(p.r#type)
        .bind(p.subtype)
        .bind(p.data)

        // In 0.7, `Transaction` can no longer implement `Executor` directly,
        // so it must be de referenced to the internal connection type.
        .fetch_one(&mut **tx)
        .await?;
    Ok(row)
}


pub async fn update(tx: &mut Transaction<'_, Postgres>, p: PartyDb) -> anyhow::Result<PartyDb> {
    let row: PartyDb = sqlx::query_as(
        r#"UPDATE party
            SET "type" = $1, subtype = $2, data = $3
            WHERE id = $4 AND claim_id = $5
            RETURNING *"#
    )
        .bind(p.r#type)
        .bind(p.subtype)
        .bind(p.data)
        .bind(p.id)
        .bind(p.claim_id)
        .fetch_one(&mut **tx)
        .await?;
    Ok(row)
}
