-- Add up migration script here
ALTER TABLE image
ADD COLUMN created_at INTEGER
DEFAULT NULL /* replace me */;


UPDATE image SET created_at = unixepoch();


PRAGMA writable_schema = on;

UPDATE sqlite_master
SET sql = replace(sql, 'DEFAULT NULL /* replace me */',
                       'NOT NULL DEFAULT (unixepoch())')
WHERE type = 'table'
  AND name = 'image';

PRAGMA writable_schema = off;
