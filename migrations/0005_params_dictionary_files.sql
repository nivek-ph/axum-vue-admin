create table if not exists sys_params (
    id bigserial primary key,
    name text not null,
    "key" text not null unique,
    value text not null,
    "desc" text not null default ''
);

create table if not exists sys_dictionaries (
    id bigserial primary key,
    name text not null,
    type text not null unique,
    status boolean,
    "desc" text not null default '',
    parent_id bigint
);

create table if not exists sys_dictionary_details (
    id bigserial primary key,
    label text not null,
    value text not null,
    extend text not null default '',
    status boolean,
    sort integer not null default 0,
    sys_dictionary_id bigint not null,
    parent_id bigint,
    level integer not null default 0,
    path text not null default ''
);

create table if not exists attachment_categories (
    id bigserial primary key,
    name text not null,
    pid bigint not null default 0
);

create table if not exists uploaded_files (
    id bigserial primary key,
    name text not null,
    url text not null,
    tag text not null default '',
    class_id bigint not null default 0,
    updated_at timestamptz not null default now()
);
