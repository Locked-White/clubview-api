-- Your SQL goes here
CREATE TYPE club_info_component_type AS ENUM (
    'text',
    'header',
    'image',
    'video',
    'slider',
    'container',
    'chat',
    'sns',
    'news'
    );
CREATE TABLE club_info_component (
    id uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid()
)