CREATE TABLE sessions
(
    user_id       INT NOT NULL,
    token         TEXT NOT NULL PRIMARY KEY,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at    TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '14 DAY',
    FOREIGN KEY (user_id) REFERENCES users (id)
)
