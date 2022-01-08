-- Add migration script here

ALTER TABLE durations
ALTER COLUMN category_uuid TYPE character varying(50);
