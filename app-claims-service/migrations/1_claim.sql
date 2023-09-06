CREATE TABLE IF NOT EXISTS claim
(
    id          BIGSERIAL PRIMARY KEY,
    status      VARCHAR NOT NULL,
    involved    JSONB NOT NULL
);