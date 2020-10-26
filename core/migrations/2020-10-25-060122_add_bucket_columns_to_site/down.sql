ALTER TABLE sites ADD COLUMN description VARCHAR;
UPDATE sites SET description = title;
ALTER TABLE sites ALTER COLUMN description SET NOT NULL;
ALTER TABLE sites DROP COLUMN title;
ALTER TABLE sites DROP COLUMN bucket_domain_name;
ALTER TABLE sites DROP COLUMN bucket_access_key_id;
ALTER TABLE sites DROP COLUMN bucket_access_key_secret;
