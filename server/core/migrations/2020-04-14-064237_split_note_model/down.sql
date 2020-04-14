DELETE FROM notes;
ALTER TABLE notes ADD COLUMN content JSONB NOT NULL;
DROP TABLE blocks;
DROP TABLE block_versions;
