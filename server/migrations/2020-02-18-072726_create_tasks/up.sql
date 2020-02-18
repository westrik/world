DROP TABLE items;
CREATE TABLE tasks
(
    id            SERIAL PRIMARY KEY,
    api_id        VARCHAR UNIQUE NOT NULL DEFAULT '',
    user_id       INT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at  TIMESTAMPTZ,
    content       TEXT NOT NULL,
    sibling_id    INT UNIQUE DEFERRABLE INITIALLY DEFERRED,
    parent_id     INT UNIQUE DEFERRABLE INITIALLY DEFERRED,
    is_collapsed  BOOLEAN NOT NULL DEFAULT FALSE,

    FOREIGN KEY (user_id) REFERENCES users (id)
);
