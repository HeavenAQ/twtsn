-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    id uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4 ()),
    email varchar NOT NULL UNIQUE,
    name varchar(255),
    company varchar(255),
    phone varchar(255),
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);

CREATE TABLE IF NOT EXISTS emails (
    id uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4 ()),
    user_id uuid NOT NULL REFERENCES users (id),
    title varchar(255) NOT NULL,
    service_type varchar(255) NOT NULL,
    body varchar NOT NULL,
    created_at timestamp with time zone DEFAULT now(),
    updated_at timestamp with time zone DEFAULT now()
);
