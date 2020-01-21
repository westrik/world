-- TODO: add migration to create tags table w/ task_id fk
-- TODO: recursive CTE to traverse task list

ALTER TABLE tasks
    ADD COLUMN completed_at TIMESTAMPTZ,
    ADD COLUMN api_id VARCHAR UNIQUE,
    ADD COLUMN next_api_id VARCHAR UNIQUE,
    ADD COLUMN parent_api_id VARCHAR UNIQUE,
    ADD COLUMN is_collapsed_child BOOLEAN DEFAULT FALSE
