ALTER TABLE block_versions DROP COLUMN position;
ALTER TABLE blocks ADD COLUMN position INT NOT NULL DEFAULT 0;
