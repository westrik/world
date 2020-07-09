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
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    FOREIGN KEY (note_id) REFERENCES notes (id)
);

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

ALTER TABLE tasks ADD COLUMN block_id INT NOT NULL, ADD FOREIGN KEY (block_id) REFERENCES blocks (id);
