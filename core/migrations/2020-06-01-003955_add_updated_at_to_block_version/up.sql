ALTER TABLE block_versions ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();
