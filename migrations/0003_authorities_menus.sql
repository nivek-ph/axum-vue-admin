create table if not exists sys_authorities (
    authority_id bigint primary key,
    authority_name text not null,
    parent_id bigint not null default 0,
    default_router text not null default 'dashboard'
);

create table if not exists sys_menus (
    id bigserial primary key,
    parent_id bigint not null default 0,
    path text not null,
    name text not null unique,
    hidden boolean not null default false,
    component text not null,
    sort integer not null default 0,
    active_name text not null default '',
    keep_alive boolean not null default false,
    default_menu boolean not null default false,
    title text not null,
    icon text not null default '',
    close_tab boolean not null default false,
    transition_type text not null default '',
    parameters jsonb not null default '[]'::jsonb,
    menu_btn jsonb not null default '[]'::jsonb
);

create table if not exists sys_role_menus (
    authority_id bigint not null,
    menu_id bigint not null,
    primary key (authority_id, menu_id)
);
