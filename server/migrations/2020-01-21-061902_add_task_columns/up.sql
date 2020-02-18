-- TODO: add migration to create tags table w/ task_id fk
-- TODO: recursive CTE to traverse task list

ALTER TABLE tasks
    ADD COLUMN completed_at TIMESTAMPTZ,
    ADD COLUMN api_id VARCHAR UNIQUE,
    ADD COLUMN sibling_id INT UNIQUE DEFERRABLE INITIALLY DEFERRED,
    ADD COLUMN parent_id INT UNIQUE DEFERRABLE INITIALLY DEFERRED,
    ADD COLUMN is_collapsed BOOLEAN NOT NULL DEFAULT FALSE
