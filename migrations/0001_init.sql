create table if not exists sys_users (
    id bigserial primary key,
    uuid text not null unique,
    username text not null unique,
    password_hash text not null,
    nick_name text not null,
    header_img text not null,
    authority_id bigint not null default 888,
    authority_name text not null default 'Super Admin',
    default_router text not null default 'dashboard',
    enable boolean not null default true,
    origin_setting jsonb,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index if not exists idx_sys_users_username on sys_users(username);
