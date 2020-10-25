ALTER TABLE sites ADD COLUMN title VARCHAR;
UPDATE sites SET title = description;
ALTER TABLE sites ALTER COLUMN title SET NOT NULL;
ALTER TABLE sites DROP COLUMN description;
ALTER TABLE sites ADD COLUMN bucket_domain_name VARCHAR;
ALTER TABLE sites ADD COLUMN bucket_access_key_id VARCHAR;
ALTER TABLE sites ADD COLUMN bucket_access_key_secret VARCHAR;
