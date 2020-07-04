ALTER TABLE block_versions ADD COLUMN note_version_id INT NOT NULL;
ALTER TABLE block_versions ADD FOREIGN KEY (note_version_id) REFERENCES note_versions (id);
ALTER TABLE block_versions ADD CONSTRAINT note_version_position_uniq UNIQUE (note_version_id, position) DEFERRABLE INITIALLY DEFERRED;
