DROP TABLE media_item_versions;
DROP TABLE media_items;

CREATE TABLE library_items (
    id            SERIAL PRIMARY KEY,
    api_id        VARCHAR UNIQUE NOT NULL,
    user_id       INT NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    name          VARCHAR NOT NULL DEFAULT '',
    presigned_upload_url VARCHAR,
    uploaded_file_name VARCHAR,
    uploaded_file_size_bytes BIGINT,

    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE library_item_versions (
    id                    SERIAL PRIMARY KEY,
    api_id                VARCHAR UNIQUE NOT NULL,
    user_id               INT NOT NULL,
    library_item_id       INT NOT NULL,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    version_type          VARCHAR NOT NULL,
    asset_url             VARCHAR,
    asset_file_size_bytes BIGINT,
    asset_data            JSONB,

    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (library_item_id) REFERENCES library_items (id)
);

