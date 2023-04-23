-- Add down migration script here
ALTER TABLE image
DROP COLUMN created_at;
