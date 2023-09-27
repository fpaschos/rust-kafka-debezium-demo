CREATE TABLE IF NOT EXISTS claim_outbox_event
(
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    aggregatetype   VARCHAR(255) NOT NULL,
    aggregateid     VARCHAR(255) NOT NULL,
    "type"          VARCHAR(255) NOT NULL,
    payload         BYTEA  NOT NULL
);

-- Needed for debezium previous state capture see above
ALTER TABLE claim_outbox_event REPLICA IDENTITY FULL;