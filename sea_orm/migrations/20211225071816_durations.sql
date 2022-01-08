-- Add migration script here

CREATE TABLE IF NOT EXISTS durations (
  "uuid" character varying(50) NOT NULL
  ,"activity_uuid" character varying(50) NOT NULL
  ,"category_uuid" text
  ,"start_date" timestamp with time zone NOT NULL
  ,"end_date" timestamp with time zone
  ,"created_at" timestamp with time zone NOT NULL
  ,"updated_at" timestamp with time zone
  ,"deleted_at" timestamp with time zone
  ,PRIMARY KEY ("uuid")
  ,FOREIGN KEY ("activity_uuid") REFERENCES "durations" ("uuid")
)
