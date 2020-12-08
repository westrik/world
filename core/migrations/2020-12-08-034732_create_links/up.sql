CREATE TABLE links
(
    id               SERIAL PRIMARY KEY,
    user_id          INT NOT NULL,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    note_id          INT NOT NULL,
    note_version_id  INT NOT NULL,
    media_item_id    INT,
    target_note_id   INT,
    external_url     VARCHAR,

    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (note_id) REFERENCES notes (id),
    FOREIGN KEY (note_version_id) REFERENCES note_versions (id),
    FOREIGN KEY (target_note_id) REFERENCES notes (id),
    FOREIGN KEY (media_item_id) REFERENCES media_items (id),
    CHECK (
        (media_item_id IS NOT NULL AND target_note_id IS NULL AND external_url IS NULL) OR
        (media_item_id IS NULL AND target_note_id IS NOT NULL AND external_url IS NULL) OR
        (media_item_id IS NULL AND target_note_id IS NULL AND external_url IS NOT NULL)
    )
);
