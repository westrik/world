ALTER TABLE sites ADD COLUMN bucket_domain_name VARCHAR;
UPDATE sites SET bucket_domain_name = bucket_name;
ALTER TABLE sites DROP COLUMN bucket_name;
