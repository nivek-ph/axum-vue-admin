create table if not exists sys_apis (
    id bigserial primary key,
    path text not null,
    description text not null,
    api_group text not null,
    method text not null,
    unique (path, method)
);

create table if not exists sys_role_apis (
    authority_id bigint not null,
    api_id bigint not null,
    primary key (authority_id, api_id)
);

create table if not exists sys_login_logs (
    id bigserial primary key,
    username text not null,
    ip text not null default '',
    status boolean not null,
    error_message text not null default '',
    agent text not null default '',
    user_id bigint,
    created_at timestamptz not null default now()
);

create table if not exists sys_operation_records (
    id bigserial primary key,
    ip text not null default '',
    method text not null,
    path text not null,
    status integer not null default 200,
    agent text not null default '',
    error_message text not null default '',
    body text not null default '',
    resp text not null default '',
    user_id bigint not null,
    created_at timestamptz not null default now()
);
