CREATE TABLE links
(
    id            SERIAL PRIMARY KEY,
    user_id       INT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    note_id       INT NOT NULL,
    media_item_id INT,
    external_url  VARCHAR,

    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (note_id) REFERENCES notes (id),
    FOREIGN KEY (media_item_id) REFERENCES media_items (id)
);
