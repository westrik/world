CREATE TABLE jobs
(
    id            SERIAL PRIMARY KEY,
    api_id        VARCHAR UNIQUE NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    status        VARCHAR NOT NULL DEFAULT 'pending',
    job_type      VARCHAR NOT NULL,
    payload       JSONB
);
