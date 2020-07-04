DELETE FROM notes;
ALTER TABLE notes DROP COLUMN content;
CREATE TABLE blocks (
    id            SERIAL PRIMARY KEY,
    api_id        VARCHAR UNIQUE NOT NULL DEFAULT '',
    user_id       INT NOT NULL,
    note_id       INT,
    position      INT NOT NULL DEFAULT 0,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (note_id) REFERENCES notes (id),

    UNIQUE (note_id, position)
);
CREATE TABLE block_versions
(
    id            SERIAL PRIMARY KEY,
    block_id      INT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    content       JSONB NOT NULL
);
