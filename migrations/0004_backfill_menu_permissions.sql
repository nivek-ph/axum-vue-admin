CREATE TEMP TABLE _page_permission_seed (
    module_key TEXT NOT NULL,
    resource TEXT NOT NULL,
    action TEXT NOT NULL,
    code TEXT NOT NULL,
    name TEXT NOT NULL
) ON COMMIT DROP;

INSERT INTO _page_permission_seed (module_key, resource, action, code, name)
VALUES
('system', 'dashboard', 'page', 'system:dashboard:page', 'Dashboard page'),
('system', 'users', 'page', 'system:users:page', 'Users page'),
('system', 'roles', 'page', 'system:roles:page', 'Roles page'),
('system', 'menus', 'page', 'system:menus:page', 'Menus page'),
('system', 'apis', 'page', 'system:apis:page', 'API directory page'),
('system', 'params', 'page', 'system:params:page', 'Params page'),
('system', 'dictionaries', 'page', 'system:dictionaries:page', 'Dictionaries page'),
('system', 'files', 'page', 'system:files:page', 'Files page'),
('system', 'login_logs', 'page', 'system:login_logs:page', 'Login logs page'),
('system', 'operation_logs', 'page', 'system:operation_logs:page', 'Operation logs page'),
('system', 'profile', 'page', 'system:profile:page', 'Profile page'),
('system', 'system_config', 'page', 'system:system_config:page', 'System config page'),
('system', 'system_state', 'page', 'system:system_state:page', 'System state page'),
('system', 'departments', 'page', 'system:departments:page', 'Departments page'),
('system', 'permissions', 'page', 'system:permissions:page', 'Permissions page'),
('system', 'api_permissions', 'page', 'system:api_permissions:page', 'API permission bindings page');

INSERT INTO sys_permissions (module_key, resource, action, code, name, type, status)
SELECT module_key, resource, action, code, name, 'page', 'enabled'
FROM _page_permission_seed
ON CONFLICT (code) DO UPDATE
SET module_key = EXCLUDED.module_key,
    resource = EXCLUDED.resource,
    action = EXCLUDED.action,
    name = EXCLUDED.name,
    type = 'page',
    status = 'enabled',
    updated_at = now();

INSERT INTO sys_permissions (module_key, resource, action, code, name, type, status)
SELECT DISTINCT
    m.module_key,
    replace(m.name, '-', '_'),
    'page',
    concat(m.module_key, ':', replace(m.name, '-', '_'), ':page'),
    concat(m.title, ' page'),
    'page',
    'enabled'
FROM sys_menus m
WHERE m.menu_type <> 'action'
  AND m.permission IS NULL
ON CONFLICT (code) DO UPDATE
SET module_key = EXCLUDED.module_key,
    resource = EXCLUDED.resource,
    action = EXCLUDED.action,
    name = EXCLUDED.name,
    type = 'page',
    status = 'enabled',
    updated_at = now();

UPDATE sys_menus m
SET permission = COALESCE(m.permission, concat(m.module_key, ':', replace(m.name, '-', '_'), ':page')),
    permission_id = p.id
FROM sys_permissions p
WHERE m.menu_type <> 'action'
  AND p.code = COALESCE(m.permission, concat(m.module_key, ':', replace(m.name, '-', '_'), ':page'));

UPDATE sys_menus m
SET permission_id = p.id
FROM sys_permissions p
WHERE m.menu_type = 'action'
  AND m.permission IS NOT NULL
  AND p.code = m.permission;

INSERT INTO sys_role_permissions (role_id, permission_id)
SELECT 1, id
FROM sys_permissions
ON CONFLICT DO NOTHING;
