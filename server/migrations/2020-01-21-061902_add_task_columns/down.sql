ALTER TABLE tasks
    DROP COLUMN completed_at,
    DROP COLUMN api_id,
    DROP COLUMN next_api_id,
    DROP COLUMN parent_api_id,
    DROP COLUMN is_collapsed_child
