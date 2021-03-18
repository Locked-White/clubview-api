-- Your SQL goes here
CREATE EXTENSION if not exists pgcrypto;
CREATE TYPE message_type AS ENUM (
    'normal',
    'audio',
    'video',
    'file'
    );

CREATE TABLE messages (
    id uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid() ,
    author_id TEXT NOT NULL ,
    content_type message_type NOT NULL ,
    message TEXT NOT NULL ,
    --channel uuid NOT NULL ,
    meta jsonb NOT NULL DEFAULT '{}', -- in Rust, NonNull is better (not nullable)
    posted_at TIMESTAMP
);