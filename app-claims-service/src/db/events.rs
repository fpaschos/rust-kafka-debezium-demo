use crate::db::PostgresTx;
use crate::model::ClaimOutboxEventDb;

pub async fn send_event<E: Into<ClaimOutboxEventDb>>(
    tx: &mut PostgresTx<'_>,
    e: E,
) -> anyhow::Result<ClaimOutboxEventDb> {
    let inserted = create_event(tx, e.into()).await?;
    delete_event(tx, inserted).await
}

pub async fn create_event(
    tx: &mut PostgresTx<'_>,
    e: ClaimOutboxEventDb,
) -> anyhow::Result<ClaimOutboxEventDb> {
    let row: ClaimOutboxEventDb = sqlx::query_as(
        r#"INSERT INTO claim_outbox_event (aggregatetype, aggregateid, "type", payload) VALUES ($1, $2, $3, $4) RETURNING *"#,
    )
    .bind(e.aggregatetype)
    .bind(e.aggregateid)
    .bind(e.r#type)
    .bind(e.payload)

    // In 0.7, `Transaction` can no longer implement `Executor` directly,
    // so it must be de referenced to the internal connection type.
    .fetch_one(&mut **tx)
    .await?;
    Ok(row)
}

pub async fn delete_event(
    tx: &mut PostgresTx<'_>,
    e: ClaimOutboxEventDb,
) -> anyhow::Result<ClaimOutboxEventDb> {
    let row: ClaimOutboxEventDb =
        sqlx::query_as(r#"DELETE FROM claim_outbox_event WHERE id = $1 RETURNING *"#)
            .bind(e.id)
            // In 0.7, `Transaction` can no longer implement `Executor` directly,
            // so it must be de referenced to the internal connection type.
            .fetch_one(&mut **tx)
            .await?;
    Ok(row)
}
