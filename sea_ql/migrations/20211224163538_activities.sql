-- Add migration script here

CREATE TABLE IF NOT EXISTS activities {
  "uuid" character varying(50) NOT NULL
  ,"name" text NOT NULL
  ,"category_id" text NOT NULL
  ,"created_at" timestamp with time zone NOT NULL
  ,"updated_at" timestamp with time zone NOT NULL
  ,"deleted_at" timestamp with time zone NOT NULL
  ,PRIMARY KEY ("uuid")
}
