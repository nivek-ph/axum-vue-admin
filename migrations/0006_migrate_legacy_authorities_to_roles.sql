CREATE TEMP TABLE _legacy_authority_roles AS
SELECT
    a.authority_id,
    a.authority_name,
    trim(both '_' from regexp_replace(lower(a.authority_name), '[^a-z0-9]+', '_', 'g')) AS role_code
FROM sys_authorities a
WHERE a.authority_id <> 888;

INSERT INTO sys_roles (code, name, status, sort, data_scope, is_system)
SELECT
    COALESCE(NULLIF(role_code, ''), concat('role_', authority_id)),
    authority_name,
    'enabled',
    authority_id::integer,
    'all',
    false
FROM _legacy_authority_roles legacy
WHERE NOT EXISTS (
    SELECT 1
    FROM sys_roles r
    WHERE lower(r.name) = lower(legacy.authority_name)
       OR r.code = COALESCE(NULLIF(legacy.role_code, ''), concat('role_', legacy.authority_id))
)
ON CONFLICT (code) DO NOTHING;

INSERT INTO sys_user_roles (user_id, role_id)
SELECT
    u.id,
    CASE
        WHEN u.authority_id = 888 OR lower(u.authority_name) = 'super admin' THEN 1
        ELSE r.id
    END AS role_id
FROM sys_users u
LEFT JOIN sys_roles r
    ON lower(r.name) = lower(u.authority_name)
    OR r.code = trim(both '_' from regexp_replace(lower(u.authority_name), '[^a-z0-9]+', '_', 'g'))
WHERE (u.authority_id = 888 OR lower(u.authority_name) = 'super admin' OR r.id IS NOT NULL)
ON CONFLICT DO NOTHING;
