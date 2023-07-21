-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    id uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4 ()),
    email varchar NOT NULL UNIQUE,
    first_name varchar(255),
    last_name varchar(255),
    phone varchar(255),
    created_at timestamp NOT NULL DEFAULT now(),
    updated_at timestamp NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS services (
    id uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4 ()),
    name varchar(255) NOT NULL UNIQUE,
    created_at timestamp NOT NULL DEFAULT now(),
);

CREATE TABLE IF NOT EXISTS emails (
    id uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4 ()),
    user_id uuid NOT NULL REFERENCES users (id),
    title varchar(255),
    body varchar,
);
