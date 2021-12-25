-- Add migration script here

CREATE TABLE IF NOT EXISTS categories (
  "uuid" character varying(50) NOT NULL
  ,"sub_category_uuid" character varying(50)
  ,"name" text NOT NULL
  ,"created_at" timestamp with time zone NOT NULL
  ,"updated_at" timestamp with time zone
  ,"deleted_at" timestamp with time zone
  ,PRIMARY KEY ("uuid")
  ,FOREIGN KEY ("sub_category_uuid") REFERENCES "sub_categories" ("uuid")
)
