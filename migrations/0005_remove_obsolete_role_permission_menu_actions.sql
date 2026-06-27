DELETE FROM sys_menus
WHERE name IN ('roles:permission-tree', 'roles:permission-matrix')
   OR api_path IN ('/api/roles/permissions/tree', '/api/roles/permissions/role-matrix');
