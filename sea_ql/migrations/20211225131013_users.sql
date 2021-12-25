-- Add migration script here

CREATE TABLE IF NOT EXISTS users (
  "uuid" character varying(50) NOT NULL
  ,"name" text NOT NULL
  ,"email" text NOT NULL
  ,"image" text
  ,"description" text
  ,"created_at" timestamp with time zone NOT NULL
  ,"updated_at" timestamp with time zone
  ,"deleted_at" timestamp with time zone
  ,PRIMARY KEY ("uuid")
)
