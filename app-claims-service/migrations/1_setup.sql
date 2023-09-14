CREATE SEQUENCE IF NOT EXISTS claim_id_seq;

CREATE TABLE IF NOT EXISTS claim
(
    id              INTEGER PRIMARY KEY DEFAULT nextval('claim_id_seq'),
    claim_no        VARCHAR UNIQUE NOT NULL,
    incident_type   VARCHAR NOT NULL,
    status          VARCHAR NOT NULL
);
-- Needed for debezium previous state capture
-- see: https://stackoverflow.com/questions/59799503/postgres-debezium-does-not-publish-the-previous-state-of-a-record
ALTER TABLE claim REPLICA IDENTITY FULL;

CREATE SEQUENCE IF NOT EXISTS party_id_seq;
CREATE TABLE IF NOT EXISTS party
(
    id              INTEGER PRIMARY KEY DEFAULT nextval('party_id_seq'),
    claim_id        INTEGER NOT NULL,
    "type"          VARCHAR NOT NULL,
    subtype         VARCHAR NOT NULL,
    "data"          JSONB NOT NULL,
    CONSTRAINT fk_claim_id
        FOREIGN KEY(claim_id)
        REFERENCES claim(id)
);
-- Needed for debezium previous state capture see above
ALTER TABLE party REPLICA IDENTITY FULL;

