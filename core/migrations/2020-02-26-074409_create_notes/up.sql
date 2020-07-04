CREATE TABLE notes
(
    id            SERIAL PRIMARY KEY,
    api_id        VARCHAR UNIQUE NOT NULL DEFAULT '',
    user_id       INT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    content       JSONB NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users (id)
);
