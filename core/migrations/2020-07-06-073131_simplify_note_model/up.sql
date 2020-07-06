ALTER TABLE tasks DROP COLUMN block_id;
DROP TABLE block_versions;
DROP TABLE blocks;
DROP TABLE note_versions;
DROP TABLE notes;

CREATE TABLE notes (
    id            SERIAL PRIMARY KEY,
    api_id        VARCHAR UNIQUE NOT NULL,
    user_id       INT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    name          VARCHAR NOT NULL DEFAULT '',

    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE note_versions (
    id            SERIAL PRIMARY KEY,
    api_id        VARCHAR UNIQUE NOT NULL,
    note_id       INT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    content       JSONB NOT NULL,

    FOREIGN KEY (note_id) REFERENCES notes (id)
);
