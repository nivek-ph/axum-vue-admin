CREATE TABLE IF NOT EXISTS sys_depts (
    id BIGSERIAL PRIMARY KEY,
    parent_id BIGINT REFERENCES sys_depts(id) ON DELETE SET NULL,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    sort INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'enabled',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS sys_roles (
    id BIGSERIAL PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'enabled',
    sort INTEGER NOT NULL DEFAULT 0,
    data_scope TEXT NOT NULL DEFAULT 'all',
    is_system BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT sys_roles_data_scope_check CHECK (
        data_scope IN ('all', 'dept', 'dept_and_children', 'self', 'custom_depts')
    )
);

CREATE TABLE IF NOT EXISTS sys_user_roles (
    user_id BIGINT NOT NULL REFERENCES sys_users(id) ON DELETE CASCADE,
    role_id BIGINT NOT NULL REFERENCES sys_roles(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);

CREATE TABLE IF NOT EXISTS sys_permissions (
    id BIGSERIAL PRIMARY KEY,
    module_key TEXT NOT NULL,
    resource TEXT NOT NULL,
    action TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    type TEXT NOT NULL DEFAULT 'action',
    status TEXT NOT NULL DEFAULT 'enabled',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT sys_permissions_type_check CHECK (type IN ('page', 'action', 'api', 'data'))
);

CREATE TABLE IF NOT EXISTS sys_permission_apis (
    permission_id BIGINT NOT NULL REFERENCES sys_permissions(id) ON DELETE CASCADE,
    method TEXT NOT NULL,
    path_pattern TEXT NOT NULL,
    PRIMARY KEY (permission_id, method, path_pattern)
);

CREATE TABLE IF NOT EXISTS sys_role_permissions (
    role_id BIGINT NOT NULL REFERENCES sys_roles(id) ON DELETE CASCADE,
    permission_id BIGINT NOT NULL REFERENCES sys_permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

CREATE TABLE IF NOT EXISTS sys_role_depts (
    role_id BIGINT NOT NULL REFERENCES sys_roles(id) ON DELETE CASCADE,
    dept_id BIGINT NOT NULL REFERENCES sys_depts(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, dept_id)
);

ALTER TABLE sys_users ADD COLUMN IF NOT EXISTS dept_id BIGINT REFERENCES sys_depts(id) ON DELETE SET NULL;
ALTER TABLE sys_users ADD COLUMN IF NOT EXISTS is_system BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE sys_menus ADD COLUMN IF NOT EXISTS module_key TEXT NOT NULL DEFAULT 'system';
ALTER TABLE sys_menus ADD COLUMN IF NOT EXISTS permission_id BIGINT REFERENCES sys_permissions(id) ON DELETE SET NULL;

INSERT INTO sys_depts (id, parent_id, name, code, sort, status)
VALUES (1, NULL, 'Head Office', 'head_office', 0, 'enabled')
ON CONFLICT (id) DO NOTHING;

SELECT setval(pg_get_serial_sequence('sys_depts', 'id'), (SELECT max(id) FROM sys_depts));

INSERT INTO sys_roles (id, code, name, status, sort, data_scope, is_system)
VALUES (1, 'super_admin', 'Super Admin', 'enabled', 0, 'all', true)
ON CONFLICT (id) DO NOTHING;

SELECT setval(pg_get_serial_sequence('sys_roles', 'id'), (SELECT max(id) FROM sys_roles));

UPDATE sys_users
SET dept_id = COALESCE(dept_id, 1),
    is_system = true
WHERE username = 'admin';

INSERT INTO sys_user_roles (user_id, role_id)
SELECT id, 1 FROM sys_users WHERE username = 'admin'
ON CONFLICT DO NOTHING;

CREATE OR REPLACE FUNCTION ensure_super_admin_role_assignment()
RETURNS trigger AS $$
BEGIN
    IF NEW.username = 'admin' THEN
        UPDATE sys_users
        SET dept_id = COALESCE(dept_id, 1),
            is_system = true
        WHERE id = NEW.id;

        INSERT INTO sys_user_roles (user_id, role_id)
        VALUES (NEW.id, 1)
        ON CONFLICT DO NOTHING;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_ensure_super_admin_role_assignment ON sys_users;
CREATE TRIGGER trg_ensure_super_admin_role_assignment
AFTER INSERT ON sys_users
FOR EACH ROW
EXECUTE FUNCTION ensure_super_admin_role_assignment();

INSERT INTO sys_permissions (module_key, resource, action, code, name, type, status)
VALUES
('system', 'user', 'list', 'system:user:list', 'List Users', 'action', 'enabled'),
('system', 'user', 'create', 'system:user:create', 'Create User', 'action', 'enabled'),
('system', 'user', 'update', 'system:user:update', 'Update User', 'action', 'enabled'),
('system', 'user', 'delete', 'system:user:delete', 'Delete User', 'action', 'enabled'),
('system', 'role', 'list', 'system:role:list', 'List Roles', 'action', 'enabled'),
('system', 'role', 'create', 'system:role:create', 'Create Role', 'action', 'enabled'),
('system', 'role', 'update', 'system:role:update', 'Update Role', 'action', 'enabled'),
('system', 'role', 'delete', 'system:role:delete', 'Delete Role', 'action', 'enabled'),
('system', 'menu', 'list', 'system:menu:list', 'List Menus', 'action', 'enabled'),
('system', 'permission', 'list', 'system:permission:list', 'List Permissions', 'action', 'enabled')
ON CONFLICT (code) DO NOTHING;
