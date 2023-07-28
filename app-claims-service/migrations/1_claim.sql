CREATE TABLE IF NOT EXISTS claim
(
    id          BIGSERIAL PRIMARY KEY,
    involved    JSONB NOT NULL
);