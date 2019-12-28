CREATE TABLE "user"
(
    id            SERIAL PRIMARY KEY,
    email_address TEXT        NOT NULL,
    full_name     TEXT,
    password_hash TEXT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
)
