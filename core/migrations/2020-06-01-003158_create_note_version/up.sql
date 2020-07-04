CREATE TABLE note_versions (
    id            SERIAL PRIMARY KEY,
    api_id        VARCHAR UNIQUE NOT NULL,
    note_id       INT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    FOREIGN KEY (note_id) REFERENCES notes (id)
);
