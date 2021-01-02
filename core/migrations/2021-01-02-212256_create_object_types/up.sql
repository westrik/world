CREATE TABLE object_types
(
    id               SERIAL PRIMARY KEY,
    api_id           VARCHAR UNIQUE NOT NULL DEFAULT '',
    user_id          INT NOT NULL,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    object_name      VARCHAR NOT NULL,
    description      VARCHAR,
    barcode_prefix   VARCHAR(10) UNIQUE,
    barcode_length   INT,

    FOREIGN KEY (user_id) REFERENCES users (id),
    CHECK (barcode_length IS NULL OR barcode_length > 0),
    CHECK (
        (barcode_length IS NULL AND barcode_prefix IS NULL) OR
        (barcode_length IS NOT NULL AND barcode_prefix IS NOT NULL)
    )
);
