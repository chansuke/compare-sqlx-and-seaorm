-- Add migration script here

ALTER TABLE activities
RENAME COLUMN category_id TO category_uuid;
