ALTER TABLE tasks
    DROP COLUMN completed_at,
    DROP COLUMN api_id,
    DROP COLUMN sibling_id,
    DROP COLUMN parent_id,
    DROP COLUMN is_collapsed
