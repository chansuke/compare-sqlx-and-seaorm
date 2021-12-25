-- Add migration script here

CREATE TABLE IF NOT EXISTS sub_categories (
  "uuid" character varying(50) NOT NULL
  ,"name" text NOT NULL
  ,"created_at" timestamp with time zone NOT NULL
  ,"updated_at" timestamp with time zone
  ,"deleted_at" timestamp with time zone
  ,PRIMARY KEY ("uuid")
)
