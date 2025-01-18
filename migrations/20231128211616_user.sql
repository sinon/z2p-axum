-- Add migration script here
create table "subscriber"
(
    id       uuid primary key default gen_random_uuid(),
    email         text unique not null,
    name      text        not null
);