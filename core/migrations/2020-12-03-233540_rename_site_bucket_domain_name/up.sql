ALTER TABLE sites ADD COLUMN bucket_name VARCHAR;
UPDATE sites SET bucket_name = bucket_domain_name;
ALTER TABLE sites DROP COLUMN bucket_domain_name;
