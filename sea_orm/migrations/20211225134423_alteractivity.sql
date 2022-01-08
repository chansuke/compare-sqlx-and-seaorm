-- Add migration script here

ALTER TABLE activities
ALTER COLUMN category_id TYPE character varying(50);
