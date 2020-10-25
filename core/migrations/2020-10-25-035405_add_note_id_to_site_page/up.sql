DROP TABLE site_pages;
CREATE TABLE site_pages
(
    id                SERIAL PRIMARY KEY,
    api_id            VARCHAR UNIQUE NOT NULL DEFAULT '',
    user_id           INT NOT NULL,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    site_id           INT NOT NULL,
    note_id           INT NOT NULL,
    note_version_id   INT NOT NULL,
    path              TEXT NOT NULL,
    published         BOOLEAN NOT NULL DEFAULT FALSE,

    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (site_id) REFERENCES sites (id),
    FOREIGN KEY (note_id) REFERENCES notes (id),
    FOREIGN KEY (note_version_id) REFERENCES note_versions (id)
);
