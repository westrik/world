DROP TABLE library_item_versions;
CREATE TABLE library_item_versions (
    id                SERIAL PRIMARY KEY,
    api_id            VARCHAR UNIQUE NOT NULL,
    user_id           INT NOT NULL,
    library_item_id   INT NOT NULL,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    asset_url         VARCHAR UNIQUE NOT NULL DEFAULT '',

    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (library_item_id) REFERENCES library_items (id)
);

