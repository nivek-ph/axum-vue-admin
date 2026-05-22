alter table sys_users
    add column if not exists phone text,
    add column if not exists email text;
