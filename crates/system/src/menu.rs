use std::collections::{BTreeMap, HashMap, HashSet};

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use admin_httpz::AppError;

use crate::{authority::SUPER_ADMIN_AUTHORITY_ID, errors};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuMeta {
    #[serde(rename = "activeName")]
    pub active_name: String,
    #[serde(rename = "keepAlive")]
    pub keep_alive: bool,
    #[serde(rename = "defaultMenu")]
    pub default_menu: bool,
    pub title: String,
    pub icon: String,
    #[serde(rename = "closeTab")]
    pub close_tab: bool,
    #[serde(rename = "transitionType")]
    pub transition_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuParameter {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "sysBaseMenuID")]
    pub sys_base_menu_id: i64,
    #[serde(rename = "type")]
    pub parameter_type: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuButton {
    #[serde(rename = "ID")]
    pub id: i64,
    pub name: String,
    pub desc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuView {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "parentId")]
    pub parent_id: i64,
    pub path: String,
    pub name: String,
    pub hidden: bool,
    pub component: String,
    pub sort: i32,
    pub meta: MenuMeta,
    pub parameters: Vec<MenuParameter>,
    #[serde(rename = "menuBtn")]
    pub menu_btn: Vec<MenuButton>,
    #[serde(rename = "menuType", default = "default_menu_type")]
    pub menu_type: String,
    #[serde(default)]
    pub permission: Option<String>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(rename = "apiPath", default)]
    pub api_path: Option<String>,
    pub children: Vec<MenuView>,
}

fn default_menu_type() -> String {
    "page".to_string()
}

#[derive(Debug, Clone, Copy)]
struct DefaultAction {
    name: &'static str,
    title: &'static str,
    permission: &'static str,
    method: &'static str,
    api_path: &'static str,
    sort: i32,
}

pub fn default_menus() -> Vec<MenuView> {
    [
        (
            1,
            "dashboard",
            "Dashboard",
            "view/dashboard/index.vue",
            "odometer",
            Vec::new(),
        ),
        (
            2,
            "users",
            "Users",
            "view/users/index.vue",
            "user",
            vec![
                DefaultAction {
                    name: "users:list",
                    title: "List",
                    permission: "system:user:list",
                    method: "GET",
                    api_path: "/api/users",
                    sort: 10,
                },
                DefaultAction {
                    name: "users:create",
                    title: "Create",
                    permission: "system:user:create",
                    method: "POST",
                    api_path: "/api/users",
                    sort: 20,
                },
                DefaultAction {
                    name: "users:update",
                    title: "Update",
                    permission: "system:user:update",
                    method: "PUT",
                    api_path: "/api/users/{id}",
                    sort: 30,
                },
                DefaultAction {
                    name: "users:delete",
                    title: "Delete",
                    permission: "system:user:delete",
                    method: "DELETE",
                    api_path: "/api/users/{id}",
                    sort: 40,
                },
                DefaultAction {
                    name: "users:reset-password",
                    title: "Reset password",
                    permission: "system:user:reset-password",
                    method: "POST",
                    api_path: "/api/users/{id}/password/reset",
                    sort: 50,
                },
                DefaultAction {
                    name: "users:assign-roles",
                    title: "Assign roles",
                    permission: "system:user:assign-roles",
                    method: "PUT",
                    api_path: "/api/users/{id}/authorities",
                    sort: 60,
                },
            ],
        ),
        (
            3,
            "roles",
            "Roles",
            "view/roles/index.vue",
            "shield",
            vec![
                DefaultAction {
                    name: "roles:list",
                    title: "List",
                    permission: "system:role:list",
                    method: "GET",
                    api_path: "/api/roles",
                    sort: 10,
                },
                DefaultAction {
                    name: "roles:create",
                    title: "Create",
                    permission: "system:role:create",
                    method: "POST",
                    api_path: "/api/roles",
                    sort: 20,
                },
                DefaultAction {
                    name: "roles:update",
                    title: "Update",
                    permission: "system:role:update",
                    method: "PUT",
                    api_path: "/api/roles/{authority_id}",
                    sort: 30,
                },
                DefaultAction {
                    name: "roles:delete",
                    title: "Delete",
                    permission: "system:role:delete",
                    method: "DELETE",
                    api_path: "/api/roles/{authority_id}",
                    sort: 40,
                },
                DefaultAction {
                    name: "roles:list-users",
                    title: "View members",
                    permission: "system:role:list-users",
                    method: "GET",
                    api_path: "/api/roles/{authority_id}/users",
                    sort: 50,
                },
                DefaultAction {
                    name: "roles:assign-users",
                    title: "Assign users",
                    permission: "system:role:assign-users",
                    method: "PUT",
                    api_path: "/api/roles/{authority_id}/users",
                    sort: 60,
                },
                DefaultAction {
                    name: "roles:permission-tree",
                    title: "Permission tree",
                    permission: "system:role:permission-tree",
                    method: "GET",
                    api_path: "/api/roles/permissions/tree",
                    sort: 70,
                },
                DefaultAction {
                    name: "roles:permission-matrix",
                    title: "Permission matrix",
                    permission: "system:role:permission-matrix",
                    method: "GET",
                    api_path: "/api/roles/permissions/role-matrix",
                    sort: 80,
                },
                DefaultAction {
                    name: "roles:update-permission",
                    title: "Save permissions",
                    permission: "system:role:update-permission",
                    method: "PUT",
                    api_path: "/api/menus/authority",
                    sort: 90,
                },
            ],
        ),
        (
            4,
            "menus",
            "Menus",
            "view/menus/index.vue",
            "menu",
            vec![
                DefaultAction {
                    name: "menus:list",
                    title: "List",
                    permission: "system:menu:list",
                    method: "GET",
                    api_path: "/api/menus",
                    sort: 10,
                },
                DefaultAction {
                    name: "menus:create",
                    title: "Create",
                    permission: "system:menu:create",
                    method: "POST",
                    api_path: "/api/menus",
                    sort: 20,
                },
                DefaultAction {
                    name: "menus:update",
                    title: "Update",
                    permission: "system:menu:update",
                    method: "PUT",
                    api_path: "/api/menus/{id}",
                    sort: 30,
                },
                DefaultAction {
                    name: "menus:delete",
                    title: "Delete",
                    permission: "system:menu:delete",
                    method: "DELETE",
                    api_path: "/api/menus/{id}",
                    sort: 40,
                },
                DefaultAction {
                    name: "menus:tree",
                    title: "Tree",
                    permission: "system:menu:tree",
                    method: "GET",
                    api_path: "/api/menus/tree",
                    sort: 50,
                },
                DefaultAction {
                    name: "menus:get-authority",
                    title: "View role menus",
                    permission: "system:menu:get-authority",
                    method: "GET",
                    api_path: "/api/menus/authority",
                    sort: 60,
                },
                DefaultAction {
                    name: "menus:set-authority",
                    title: "Assign role menus",
                    permission: "system:menu:set-authority",
                    method: "PUT",
                    api_path: "/api/menus/authority",
                    sort: 70,
                },
            ],
        ),
        (
            5,
            "apis",
            "API directory",
            "view/apis/index.vue",
            "route",
            vec![
                DefaultAction {
                    name: "apis:list",
                    title: "List",
                    permission: "system:api:list",
                    method: "GET",
                    api_path: "/api/routes",
                    sort: 10,
                },
                DefaultAction {
                    name: "apis:create",
                    title: "Create",
                    permission: "system:api:create",
                    method: "POST",
                    api_path: "/api/routes",
                    sort: 20,
                },
                DefaultAction {
                    name: "apis:update",
                    title: "Update",
                    permission: "system:api:update",
                    method: "PUT",
                    api_path: "/api/routes/{id}",
                    sort: 30,
                },
                DefaultAction {
                    name: "apis:delete",
                    title: "Delete",
                    permission: "system:api:delete",
                    method: "DELETE",
                    api_path: "/api/routes/{id}",
                    sort: 40,
                },
                DefaultAction {
                    name: "apis:list-roles",
                    title: "View roles",
                    permission: "system:api:list-roles",
                    method: "GET",
                    api_path: "/api/routes/roles",
                    sort: 50,
                },
                DefaultAction {
                    name: "apis:assign-roles",
                    title: "Assign roles",
                    permission: "system:api:assign-roles",
                    method: "PUT",
                    api_path: "/api/routes/roles",
                    sort: 60,
                },
                DefaultAction {
                    name: "apis:batch-delete",
                    title: "Batch delete",
                    permission: "system:api:batch-delete",
                    method: "DELETE",
                    api_path: "/api/routes/batch",
                    sort: 70,
                },
                DefaultAction {
                    name: "apis:list-all",
                    title: "List all",
                    permission: "system:api:list-all",
                    method: "GET",
                    api_path: "/api/routes/all",
                    sort: 80,
                },
                DefaultAction {
                    name: "apis:groups",
                    title: "Groups",
                    permission: "system:api:groups",
                    method: "GET",
                    api_path: "/api/routes/groups",
                    sort: 90,
                },
            ],
        ),
        (
            6,
            "params",
            "Params",
            "view/params/index.vue",
            "sliders",
            vec![
                DefaultAction {
                    name: "params:list",
                    title: "List",
                    permission: "system:param:list",
                    method: "GET",
                    api_path: "/api/params",
                    sort: 10,
                },
                DefaultAction {
                    name: "params:create",
                    title: "Create",
                    permission: "system:param:create",
                    method: "POST",
                    api_path: "/api/params",
                    sort: 20,
                },
                DefaultAction {
                    name: "params:get-by-key",
                    title: "Get by key",
                    permission: "system:param:get-by-key",
                    method: "GET",
                    api_path: "/api/params/by-key",
                    sort: 30,
                },
                DefaultAction {
                    name: "params:batch-delete",
                    title: "Batch delete",
                    permission: "system:param:batch-delete",
                    method: "DELETE",
                    api_path: "/api/params/batch",
                    sort: 40,
                },
                DefaultAction {
                    name: "params:get",
                    title: "Get",
                    permission: "system:param:get",
                    method: "GET",
                    api_path: "/api/params/{id}",
                    sort: 50,
                },
                DefaultAction {
                    name: "params:update",
                    title: "Update",
                    permission: "system:param:update",
                    method: "PUT",
                    api_path: "/api/params/{id}",
                    sort: 60,
                },
                DefaultAction {
                    name: "params:delete",
                    title: "Delete",
                    permission: "system:param:delete",
                    method: "DELETE",
                    api_path: "/api/params/{id}",
                    sort: 70,
                },
            ],
        ),
        (
            7,
            "dictionaries",
            "Dictionaries",
            "view/dictionaries/index.vue",
            "book",
            vec![
                DefaultAction {
                    name: "dictionaries:list",
                    title: "List",
                    permission: "system:dictionary:list",
                    method: "GET",
                    api_path: "/api/dictionaries",
                    sort: 10,
                },
                DefaultAction {
                    name: "dictionaries:create",
                    title: "Create",
                    permission: "system:dictionary:create",
                    method: "POST",
                    api_path: "/api/dictionaries",
                    sort: 20,
                },
                DefaultAction {
                    name: "dictionaries:import",
                    title: "Import",
                    permission: "system:dictionary:import",
                    method: "POST",
                    api_path: "/api/dictionaries/import",
                    sort: 30,
                },
                DefaultAction {
                    name: "dictionaries:get",
                    title: "Get",
                    permission: "system:dictionary:get",
                    method: "GET",
                    api_path: "/api/dictionaries/{id}",
                    sort: 40,
                },
                DefaultAction {
                    name: "dictionaries:update",
                    title: "Update",
                    permission: "system:dictionary:update",
                    method: "PUT",
                    api_path: "/api/dictionaries/{id}",
                    sort: 50,
                },
                DefaultAction {
                    name: "dictionaries:delete",
                    title: "Delete",
                    permission: "system:dictionary:delete",
                    method: "DELETE",
                    api_path: "/api/dictionaries/{id}",
                    sort: 60,
                },
                DefaultAction {
                    name: "dictionaries:export",
                    title: "Export",
                    permission: "system:dictionary:export",
                    method: "GET",
                    api_path: "/api/dictionaries/{id}/export",
                    sort: 70,
                },
                DefaultAction {
                    name: "dictionaries:details-tree",
                    title: "Details tree",
                    permission: "system:dictionary:details-tree",
                    method: "GET",
                    api_path: "/api/dictionaries/{id}/details/tree",
                    sort: 80,
                },
                DefaultAction {
                    name: "dictionary-details:create",
                    title: "Create detail",
                    permission: "system:dictionary-detail:create",
                    method: "POST",
                    api_path: "/api/dictionary-details",
                    sort: 90,
                },
                DefaultAction {
                    name: "dictionary-details:tree-by-type",
                    title: "Tree by type",
                    permission: "system:dictionary-detail:tree-by-type",
                    method: "GET",
                    api_path: "/api/dictionary-details/tree-by-type",
                    sort: 100,
                },
                DefaultAction {
                    name: "dictionary-details:by-parent",
                    title: "List by parent",
                    permission: "system:dictionary-detail:by-parent",
                    method: "GET",
                    api_path: "/api/dictionary-details/by-parent",
                    sort: 110,
                },
                DefaultAction {
                    name: "dictionary-details:get",
                    title: "Get detail",
                    permission: "system:dictionary-detail:get",
                    method: "GET",
                    api_path: "/api/dictionary-details/{id}",
                    sort: 120,
                },
                DefaultAction {
                    name: "dictionary-details:update",
                    title: "Update detail",
                    permission: "system:dictionary-detail:update",
                    method: "PUT",
                    api_path: "/api/dictionary-details/{id}",
                    sort: 130,
                },
                DefaultAction {
                    name: "dictionary-details:delete",
                    title: "Delete detail",
                    permission: "system:dictionary-detail:delete",
                    method: "DELETE",
                    api_path: "/api/dictionary-details/{id}",
                    sort: 140,
                },
                DefaultAction {
                    name: "dictionary-details:path",
                    title: "Detail path",
                    permission: "system:dictionary-detail:path",
                    method: "GET",
                    api_path: "/api/dictionary-details/{id}/path",
                    sort: 150,
                },
            ],
        ),
        (
            8,
            "files",
            "Files",
            "view/files/index.vue",
            "file",
            vec![
                DefaultAction {
                    name: "files:list",
                    title: "List",
                    permission: "system:file:list",
                    method: "GET",
                    api_path: "/api/files",
                    sort: 10,
                },
                DefaultAction {
                    name: "files:import-url",
                    title: "Import URL",
                    permission: "system:file:import-url",
                    method: "POST",
                    api_path: "/api/files/import-url",
                    sort: 20,
                },
                DefaultAction {
                    name: "files:upload",
                    title: "Upload",
                    permission: "system:file:upload",
                    method: "POST",
                    api_path: "/api/files/upload",
                    sort: 30,
                },
                DefaultAction {
                    name: "files:delete",
                    title: "Delete",
                    permission: "system:file:delete",
                    method: "DELETE",
                    api_path: "/api/files/{id}",
                    sort: 40,
                },
                DefaultAction {
                    name: "files:rename",
                    title: "Rename",
                    permission: "system:file:rename",
                    method: "PATCH",
                    api_path: "/api/files/{id}/name",
                    sort: 50,
                },
                DefaultAction {
                    name: "files:categories-list",
                    title: "List categories",
                    permission: "system:file:categories-list",
                    method: "GET",
                    api_path: "/api/attachment-categories",
                    sort: 60,
                },
                DefaultAction {
                    name: "files:categories-create",
                    title: "Create category",
                    permission: "system:file:categories-create",
                    method: "POST",
                    api_path: "/api/attachment-categories",
                    sort: 70,
                },
                DefaultAction {
                    name: "files:categories-delete",
                    title: "Delete category",
                    permission: "system:file:categories-delete",
                    method: "DELETE",
                    api_path: "/api/attachment-categories/{id}",
                    sort: 80,
                },
            ],
        ),
        (
            9,
            "login-logs",
            "Login logs",
            "view/login-logs/index.vue",
            "history",
            vec![
                DefaultAction {
                    name: "login-logs:list",
                    title: "List",
                    permission: "system:login-log:list",
                    method: "GET",
                    api_path: "/api/login-logs",
                    sort: 10,
                },
                DefaultAction {
                    name: "login-logs:batch-delete",
                    title: "Batch delete",
                    permission: "system:login-log:batch-delete",
                    method: "DELETE",
                    api_path: "/api/login-logs",
                    sort: 20,
                },
                DefaultAction {
                    name: "login-logs:get",
                    title: "Get",
                    permission: "system:login-log:get",
                    method: "GET",
                    api_path: "/api/login-logs/{id}",
                    sort: 30,
                },
                DefaultAction {
                    name: "login-logs:delete",
                    title: "Delete",
                    permission: "system:login-log:delete",
                    method: "DELETE",
                    api_path: "/api/login-logs/{id}",
                    sort: 40,
                },
            ],
        ),
        (
            10,
            "operation-logs",
            "Operation logs",
            "view/operation-logs/index.vue",
            "list",
            vec![
                DefaultAction {
                    name: "operation-logs:list",
                    title: "List",
                    permission: "system:operation-log:list",
                    method: "GET",
                    api_path: "/api/operation-logs",
                    sort: 10,
                },
                DefaultAction {
                    name: "operation-logs:batch-delete",
                    title: "Batch delete",
                    permission: "system:operation-log:batch-delete",
                    method: "DELETE",
                    api_path: "/api/operation-logs",
                    sort: 20,
                },
                DefaultAction {
                    name: "operation-logs:delete",
                    title: "Delete",
                    permission: "system:operation-log:delete",
                    method: "DELETE",
                    api_path: "/api/operation-logs/{id}",
                    sort: 30,
                },
            ],
        ),
        (
            11,
            "profile",
            "Profile",
            "view/profile/index.vue",
            "user",
            Vec::new(),
        ),
        (
            12,
            "system-config",
            "System config",
            "view/system-config/index.vue",
            "settings",
            vec![
                DefaultAction {
                    name: "system-config:get",
                    title: "View config",
                    permission: "system:config:get",
                    method: "GET",
                    api_path: "/api/system/config",
                    sort: 10,
                },
                DefaultAction {
                    name: "system-config:update",
                    title: "Update config",
                    permission: "system:config:update",
                    method: "PUT",
                    api_path: "/api/system/config",
                    sort: 20,
                },
            ],
        ),
        (
            13,
            "system-state",
            "System status",
            "view/system-state/index.vue",
            "activity",
            vec![DefaultAction {
                name: "system-state:get",
                title: "View status",
                permission: "system:state:get",
                method: "GET",
                api_path: "/api/system/server-info",
                sort: 10,
            }],
        ),
    ]
    .into_iter()
    .map(|(id, name, title, component, icon, actions)| {
        default_menu(id, name, title, component, icon, actions)
    })
    .collect()
}

fn default_menu(
    id: i64,
    name: &str,
    title: &str,
    component: &str,
    icon: &str,
    actions: Vec<DefaultAction>,
) -> MenuView {
    let mut menu = MenuView {
        id,
        parent_id: 0,
        path: name.to_string(),
        name: name.to_string(),
        hidden: false,
        component: component.to_string(),
        sort: id as i32,
        meta: MenuMeta {
            active_name: String::new(),
            keep_alive: false,
            default_menu: false,
            title: title.to_string(),
            icon: icon.to_string(),
            close_tab: false,
            transition_type: String::new(),
        },
        parameters: Vec::new(),
        menu_btn: Vec::new(),
        menu_type: "page".to_string(),
        permission: None,
        method: None,
        api_path: None,
        children: Vec::new(),
    };
    menu.children = actions
        .into_iter()
        .map(|action| default_action(id, action))
        .collect();
    menu
}

fn default_action(parent_id: i64, action: DefaultAction) -> MenuView {
    MenuView {
        id: 0,
        parent_id,
        path: String::new(),
        name: action.name.to_string(),
        hidden: true,
        component: String::new(),
        sort: action.sort,
        meta: MenuMeta {
            active_name: String::new(),
            keep_alive: false,
            default_menu: false,
            title: action.title.to_string(),
            icon: String::new(),
            close_tab: false,
            transition_type: String::new(),
        },
        parameters: Vec::new(),
        menu_btn: Vec::new(),
        menu_type: "action".to_string(),
        permission: Some(action.permission.to_string()),
        method: Some(action.method.to_string()),
        api_path: Some(action.api_path.to_string()),
        children: Vec::new(),
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct MenuRecord {
    pub id: i64,
    pub parent_id: i64,
    pub path: String,
    pub name: String,
    pub hidden: bool,
    pub component: String,
    pub sort: i32,
    pub active_name: String,
    pub keep_alive: bool,
    pub default_menu: bool,
    pub title: String,
    pub icon: String,
    pub close_tab: bool,
    pub transition_type: String,
    pub parameters: Option<serde_json::Value>,
    pub menu_btn: Option<serde_json::Value>,
    pub menu_type: String,
    pub permission: Option<String>,
    pub method: Option<String>,
    pub api_path: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
struct MenuRoleMatrixRow {
    pub menu_id: i64,
    pub authority_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MenuIdRequest {
    #[serde(rename = "ID", alias = "id", alias = "menuId")]
    pub id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MenuAuthorityRequest {
    #[serde(rename = "authorityId")]
    pub authority_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddMenuAuthorityRequest {
    pub menus: Vec<MenuView>,
    #[serde(rename = "authorityId")]
    pub authority_id: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetAuthorityMenusRequest {
    #[serde(rename = "authorityId")]
    pub authority_id: i64,
    #[serde(rename = "menuIds")]
    pub menu_ids: Vec<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SetMenuRolesRequest {
    #[serde(rename = "menuId")]
    pub menu_id: i64,
    #[serde(rename = "authorityIds")]
    pub authority_ids: Vec<i64>,
}

#[derive(Debug, thiserror::Error)]
pub enum MenuError {
    #[error("menu not found")]
    NotFound,
    #[error("{0}")]
    Database(#[from] sqlx::Error),
    #[error("invalid menu payload")]
    InvalidPayload,
    #[error("default role permissions cannot be changed")]
    RootAuthorityImmutable,
}

impl From<MenuError> for AppError {
    fn from(error: MenuError) -> Self {
        match error {
            MenuError::NotFound => errors::menu::MENU_NOT_FOUND.into(),
            MenuError::Database(error) => {
                errors::menu::MENU_DB_FAILED.into_error().with_source(error)
            }
            MenuError::InvalidPayload => errors::menu::MENU_INVALID_PAYLOAD.into(),
            MenuError::RootAuthorityImmutable => errors::menu::ROOT_AUTHORITY_IMMUTABLE.into(),
        }
    }
}

pub async fn ensure_default_menu(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    for mut menu in default_menus() {
        let children = std::mem::take(&mut menu.children);
        let menu_id = insert_menu_node(pool, menu, 0).await?;
        for child in children {
            insert_menu_node(pool, child, menu_id).await?;
        }
    }

    sqlx::query(
        r#"
        select setval(
            pg_get_serial_sequence('sys_menus', 'id'),
            greatest((select coalesce(max(id), 1) from sys_menus), 1),
            true
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn insert_menu_node(
    pool: &sqlx::PgPool,
    menu: MenuView,
    parent_id: i64,
) -> Result<i64, sqlx::Error> {
    let menu_id: i64 = sqlx::query_scalar(
        r#"
        insert into sys_menus (
            parent_id, path, name, hidden, component, sort,
            active_name, keep_alive, default_menu, title, icon, close_tab, transition_type,
            parameters, menu_btn, menu_type, permission, method, api_path
        ) values (
            $1, $2, $3, $4, $5, $6,
            $7, $8, $9, $10, $11, $12, $13,
            $14, $15, $16, $17, $18, $19
        )
        on conflict (name) do update
        set parent_id = excluded.parent_id,
            path = excluded.path,
            hidden = excluded.hidden,
            component = excluded.component,
            sort = excluded.sort,
            active_name = excluded.active_name,
            keep_alive = excluded.keep_alive,
            default_menu = excluded.default_menu,
            title = excluded.title,
            icon = excluded.icon,
            close_tab = excluded.close_tab,
            transition_type = excluded.transition_type,
            parameters = excluded.parameters,
            menu_btn = excluded.menu_btn,
            menu_type = excluded.menu_type,
            permission = excluded.permission,
            method = excluded.method,
            api_path = excluded.api_path
        returning id
        "#,
    )
    .bind(parent_id)
    .bind(menu.path)
    .bind(menu.name)
    .bind(menu.hidden)
    .bind(menu.component)
    .bind(menu.sort)
    .bind(menu.meta.active_name)
    .bind(menu.meta.keep_alive)
    .bind(menu.meta.default_menu)
    .bind(menu.meta.title)
    .bind(menu.meta.icon)
    .bind(menu.meta.close_tab)
    .bind(menu.meta.transition_type)
    .bind(serde_json::to_value(menu.parameters).unwrap_or_else(|_| serde_json::json!([])))
    .bind(serde_json::to_value(menu.menu_btn).unwrap_or_else(|_| serde_json::json!([])))
    .bind(menu.menu_type)
    .bind(menu.permission)
    .bind(menu.method)
    .bind(menu.api_path)
    .fetch_one(pool)
    .await?;

    sqlx::query(
        r#"
        insert into sys_role_menus (authority_id, menu_id)
        values (888, $1)
        on conflict do nothing
        "#,
    )
    .bind(menu_id)
    .execute(pool)
    .await?;

    Ok(menu_id)
}

pub async fn get_menu_tree_for_authority(
    pool: &sqlx::PgPool,
    authority_id: i64,
) -> Result<Vec<MenuView>, MenuError> {
    let authorized_menu_ids: Vec<i64> = sqlx::query_scalar(
        "select menu_id from sys_role_menus where authority_id = $1 order by menu_id",
    )
    .bind(authority_id)
    .fetch_all(pool)
    .await?;

    if authorized_menu_ids.is_empty() {
        return Ok(Vec::new());
    }

    let rows = sqlx::query_as::<_, MenuRecord>(
        r#"
        select
            m.id,
            m.parent_id,
            m.path,
            m.name,
            m.hidden,
            m.component,
            m.sort,
            m.active_name,
            m.keep_alive,
            m.default_menu,
            m.title,
            m.icon,
            m.close_tab,
            m.transition_type,
            m.parameters,
            m.menu_btn,
            m.menu_type,
            m.permission,
            m.method,
            m.api_path
        from sys_menus m
        order by m.sort asc, m.id asc
        "#,
    )
    .fetch_all(pool)
    .await?;
    let authorized_rows = filter_authorized_with_ancestors(&rows, &authorized_menu_ids);
    let rows = filter_navigation_rows(&authorized_rows, &rows, &authorized_menu_ids);

    Ok(build_tree(&rows, 0))
}

pub async fn get_menu_list(pool: &sqlx::PgPool) -> Result<Vec<MenuView>, MenuError> {
    let rows = sqlx::query_as::<_, MenuRecord>(
        r#"
        select
            id, parent_id, path, name, hidden, component, sort, active_name, keep_alive,
            default_menu, title, icon, close_tab, transition_type, parameters, menu_btn,
            menu_type, permission, method, api_path
        from sys_menus
        order by sort asc, id asc
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(build_tree(&rows, 0))
}

pub async fn get_base_menu_tree(pool: &sqlx::PgPool) -> Result<Vec<MenuView>, MenuError> {
    get_menu_list(pool).await
}

pub async fn add_base_menu(pool: &sqlx::PgPool, payload: MenuView) -> Result<(), MenuError> {
    sqlx::query(
        r#"
        insert into sys_menus (
            parent_id, path, name, hidden, component, sort,
            active_name, keep_alive, default_menu, title, icon, close_tab, transition_type,
            parameters, menu_btn, menu_type, permission, method, api_path
        ) values (
            $1, $2, $3, $4, $5, $6,
            $7, $8, $9, $10, $11, $12, $13,
            $14, $15, $16, $17, $18, $19
        )
        "#,
    )
    .bind(payload.parent_id)
    .bind(payload.path)
    .bind(payload.name)
    .bind(payload.hidden)
    .bind(payload.component)
    .bind(payload.sort)
    .bind(payload.meta.active_name)
    .bind(payload.meta.keep_alive)
    .bind(payload.meta.default_menu)
    .bind(payload.meta.title)
    .bind(payload.meta.icon)
    .bind(payload.meta.close_tab)
    .bind(payload.meta.transition_type)
    .bind(serde_json::to_value(payload.parameters).map_err(|_| MenuError::InvalidPayload)?)
    .bind(serde_json::to_value(payload.menu_btn).map_err(|_| MenuError::InvalidPayload)?)
    .bind(payload.menu_type)
    .bind(payload.permission)
    .bind(payload.method)
    .bind(payload.api_path)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_base_menu(pool: &sqlx::PgPool, payload: MenuView) -> Result<(), MenuError> {
    sqlx::query(
        r#"
        update sys_menus
        set parent_id = $1,
            path = $2,
            name = $3,
            hidden = $4,
            component = $5,
            sort = $6,
            active_name = $7,
            keep_alive = $8,
            default_menu = $9,
            title = $10,
            icon = $11,
            close_tab = $12,
            transition_type = $13,
            parameters = $14,
            menu_btn = $15,
            menu_type = $16,
            permission = $17,
            method = $18,
            api_path = $19
        where id = $20
        "#,
    )
    .bind(payload.parent_id)
    .bind(payload.path)
    .bind(payload.name)
    .bind(payload.hidden)
    .bind(payload.component)
    .bind(payload.sort)
    .bind(payload.meta.active_name)
    .bind(payload.meta.keep_alive)
    .bind(payload.meta.default_menu)
    .bind(payload.meta.title)
    .bind(payload.meta.icon)
    .bind(payload.meta.close_tab)
    .bind(payload.meta.transition_type)
    .bind(serde_json::to_value(payload.parameters).map_err(|_| MenuError::InvalidPayload)?)
    .bind(serde_json::to_value(payload.menu_btn).map_err(|_| MenuError::InvalidPayload)?)
    .bind(payload.menu_type)
    .bind(payload.permission)
    .bind(payload.method)
    .bind(payload.api_path)
    .bind(payload.id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_base_menu(pool: &sqlx::PgPool, menu_id: i64) -> Result<(), MenuError> {
    sqlx::query("delete from sys_role_menus where menu_id = $1")
        .bind(menu_id)
        .execute(pool)
        .await?;
    sqlx::query("delete from sys_menus where id = $1 or parent_id = $1")
        .bind(menu_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_base_menu_by_id(pool: &sqlx::PgPool, menu_id: i64) -> Result<MenuView, MenuError> {
    let row = sqlx::query_as::<_, MenuRecord>(
        r#"
        select
            id, parent_id, path, name, hidden, component, sort, active_name, keep_alive,
            default_menu, title, icon, close_tab, transition_type, parameters, menu_btn,
            menu_type, permission, method, api_path
        from sys_menus
        where id = $1
        "#,
    )
    .bind(menu_id)
    .fetch_optional(pool)
    .await?
    .ok_or(MenuError::NotFound)?;

    build_menu_view(&row)
}

pub async fn get_menu_authority(
    pool: &sqlx::PgPool,
    authority_id: i64,
) -> Result<Vec<AssignedMenu>, MenuError> {
    let menu_ids: Vec<i64> = sqlx::query_scalar(
        "select menu_id from sys_role_menus where authority_id = $1 order by menu_id",
    )
    .bind(authority_id)
    .fetch_all(pool)
    .await?;
    let rows = sqlx::query_as::<_, MenuRecord>(
        r#"
        select
            id, parent_id, path, name, hidden, component, sort, active_name, keep_alive,
            default_menu, title, icon, close_tab, transition_type, parameters, menu_btn,
            menu_type, permission, method, api_path
        from sys_menus
        where id = any($1)
        order by sort asc, id asc
        "#,
    )
    .bind(&menu_ids)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| AssignedMenu {
            menu_id: row.id,
            parent_id: row.parent_id,
        })
        .collect())
}

pub async fn add_menu_authority(
    pool: &sqlx::PgPool,
    payload: AddMenuAuthorityRequest,
) -> Result<(), MenuError> {
    let menu_ids: Vec<i64> = payload.menus.into_iter().map(|menu| menu.id).collect();
    replace_authority_menus(pool, payload.authority_id, &menu_ids).await
}

pub async fn set_authority_menus(
    pool: &sqlx::PgPool,
    payload: SetAuthorityMenusRequest,
) -> Result<(), MenuError> {
    replace_authority_menus(pool, payload.authority_id, &payload.menu_ids).await
}

pub async fn get_menu_roles(
    pool: &sqlx::PgPool,
    menu_id: i64,
) -> Result<MenuRoleSelection, MenuError> {
    let authority_ids: Vec<i64> = sqlx::query_scalar(
        "select authority_id from sys_role_menus where menu_id = $1 order by authority_id",
    )
    .bind(menu_id)
    .fetch_all(pool)
    .await?;
    let default_router_authority_ids: Vec<i64> = sqlx::query_scalar(
        "select authority_id from sys_authorities where default_router = (select name from sys_menus where id = $1)",
    )
    .bind(menu_id)
    .fetch_all(pool)
    .await?;

    Ok(MenuRoleSelection {
        authority_ids,
        default_router_authority_ids,
    })
}

pub async fn get_menu_role_matrix(
    pool: &sqlx::PgPool,
) -> Result<Vec<MenuRoleMatrixItem>, MenuError> {
    let rows = sqlx::query_as::<_, MenuRoleMatrixRow>(
        r#"
        select menu_id, authority_id
        from sys_role_menus
        order by menu_id, authority_id
        "#,
    )
    .fetch_all(pool)
    .await?;

    let mut grouped = BTreeMap::<i64, Vec<i64>>::new();
    for row in rows {
        grouped
            .entry(row.menu_id)
            .or_default()
            .push(row.authority_id);
    }

    Ok(grouped
        .into_iter()
        .map(|(menu_id, authority_ids)| MenuRoleMatrixItem {
            menu_id,
            authority_ids,
        })
        .collect())
}

pub async fn set_menu_roles(
    pool: &sqlx::PgPool,
    payload: SetMenuRolesRequest,
) -> Result<(), MenuError> {
    sqlx::query("delete from sys_role_menus where menu_id = $1")
        .bind(payload.menu_id)
        .execute(pool)
        .await?;

    for authority_id in payload.authority_ids {
        sqlx::query(
            r#"
            insert into sys_role_menus (authority_id, menu_id)
            values ($1, $2)
            on conflict do nothing
            "#,
        )
        .bind(authority_id)
        .bind(payload.menu_id)
        .execute(pool)
        .await?;
    }
    Ok(())
}

async fn replace_authority_menus(
    pool: &sqlx::PgPool,
    authority_id: i64,
    menu_ids: &[i64],
) -> Result<(), MenuError> {
    sqlx::query("delete from sys_role_menus where authority_id = $1")
        .bind(authority_id)
        .execute(pool)
        .await?;

    for menu_id in menu_ids {
        sqlx::query(
            r#"
            insert into sys_role_menus (authority_id, menu_id)
            values ($1, $2)
            on conflict do nothing
            "#,
        )
        .bind(authority_id)
        .bind(menu_id)
        .execute(pool)
        .await?;
    }

    Ok(())
}

fn filter_authorized_with_ancestors(
    rows: &[MenuRecord],
    authorized_menu_ids: &[i64],
) -> Vec<MenuRecord> {
    let rows_by_id = rows
        .iter()
        .map(|row| (row.id, row))
        .collect::<HashMap<_, _>>();
    let mut included_ids = HashSet::new();

    for menu_id in authorized_menu_ids {
        let mut current_id = *menu_id;
        while current_id != 0 {
            let Some(row) = rows_by_id.get(&current_id) else {
                break;
            };
            if !included_ids.insert(current_id) {
                break;
            }
            current_id = row.parent_id;
        }
    }

    rows.iter()
        .filter(|row| included_ids.contains(&row.id))
        .cloned()
        .collect()
}

fn filter_navigation_rows(
    authorized_rows: &[MenuRecord],
    all_rows: &[MenuRecord],
    authorized_menu_ids: &[i64],
) -> Vec<MenuRecord> {
    let authorized_ids = authorized_menu_ids.iter().copied().collect::<HashSet<_>>();
    let mut page_ids_with_actions = HashSet::new();
    let mut page_ids_with_authorized_actions = HashSet::new();

    for row in all_rows {
        if row.menu_type != "action" {
            continue;
        }
        page_ids_with_actions.insert(row.parent_id);
        if authorized_ids.contains(&row.id) {
            page_ids_with_authorized_actions.insert(row.parent_id);
        }
    }

    authorized_rows
        .iter()
        .filter(|row| row.menu_type != "action")
        .filter(|row| {
            !page_ids_with_actions.contains(&row.id)
                || page_ids_with_authorized_actions.contains(&row.id)
        })
        .cloned()
        .collect()
}

fn build_tree(rows: &[MenuRecord], parent_id: i64) -> Vec<MenuView> {
    let mut menus = rows
        .iter()
        .filter(|row| row.parent_id == parent_id)
        .filter_map(|row| {
            let mut view = build_menu_view(row).ok()?;
            view.children = build_tree(rows, row.id);
            Some(view)
        })
        .collect::<Vec<_>>();

    menus.sort_by_key(|item| (item.sort, item.id));
    menus
}

fn build_menu_view(row: &MenuRecord) -> Result<MenuView, MenuError> {
    Ok(MenuView {
        id: row.id,
        parent_id: row.parent_id,
        path: row.path.clone(),
        name: row.name.clone(),
        hidden: row.hidden,
        component: row.component.clone(),
        sort: row.sort,
        meta: MenuMeta {
            active_name: row.active_name.clone(),
            keep_alive: row.keep_alive,
            default_menu: row.default_menu,
            title: row.title.clone(),
            icon: row.icon.clone(),
            close_tab: row.close_tab,
            transition_type: row.transition_type.clone(),
        },
        parameters: serde_json::from_value(
            row.parameters
                .clone()
                .unwrap_or_else(|| serde_json::json!([])),
        )
        .map_err(|_| MenuError::InvalidPayload)?,
        menu_btn: serde_json::from_value(
            row.menu_btn
                .clone()
                .unwrap_or_else(|| serde_json::json!([])),
        )
        .map_err(|_| MenuError::InvalidPayload)?,
        menu_type: row.menu_type.clone(),
        permission: row.permission.clone(),
        method: row.method.clone(),
        api_path: row.api_path.clone(),
        children: Vec::new(),
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionAccessDecision {
    Allowed,
    Denied,
    Unregistered,
}

#[derive(Debug, Clone, FromRow)]
struct RegisteredPermissionPath {
    pub id: i64,
    pub api_path: String,
}

pub fn route_pattern_matches(pattern: &str, path: &str) -> bool {
    let pattern_segments = pattern.trim_matches('/').split('/').collect::<Vec<_>>();
    let path_segments = path.trim_matches('/').split('/').collect::<Vec<_>>();

    pattern_segments.len() == path_segments.len()
        && pattern_segments.iter().zip(path_segments.iter()).all(
            |(pattern_segment, path_segment)| {
                is_dynamic_segment(pattern_segment) || pattern_segment == path_segment
            },
        )
}

pub fn is_dynamic_path_pattern(pattern: &str) -> bool {
    pattern.trim_matches('/').split('/').any(is_dynamic_segment)
}

fn matching_permission_ids(candidates: &[RegisteredPermissionPath], path: &str) -> Vec<i64> {
    let exact_ids = candidates
        .iter()
        .filter(|permission| permission.api_path == path)
        .map(|permission| permission.id)
        .collect::<Vec<_>>();
    if !exact_ids.is_empty() {
        return exact_ids;
    }

    candidates
        .iter()
        .filter(|permission| {
            is_dynamic_path_pattern(&permission.api_path)
                && route_pattern_matches(&permission.api_path, path)
        })
        .map(|permission| permission.id)
        .collect()
}

fn is_dynamic_segment(segment: &str) -> bool {
    (segment.starts_with('{') && segment.ends_with('}')) || segment.starts_with(':')
}

pub async fn check_permission_access(
    pool: &sqlx::PgPool,
    authority_id: i64,
    path: &str,
    method: &str,
) -> Result<PermissionAccessDecision, MenuError> {
    if authority_id == SUPER_ADMIN_AUTHORITY_ID {
        return Ok(PermissionAccessDecision::Allowed);
    }

    let method = method.to_ascii_uppercase();
    let candidates = sqlx::query_as::<_, RegisteredPermissionPath>(
        r#"
        select id, api_path
        from sys_menus
        where menu_type = 'action'
          and method = $1
          and api_path is not null
        order by api_path
        "#,
    )
    .bind(method)
    .fetch_all(pool)
    .await?;

    let matched_menu_ids = matching_permission_ids(&candidates, path);

    if matched_menu_ids.is_empty() {
        return Ok(PermissionAccessDecision::Unregistered);
    }

    let allowed: Option<i64> = sqlx::query_scalar(
        r#"
        select menu_id
        from sys_role_menus
        where authority_id = $1 and menu_id = any($2)
        limit 1
        "#,
    )
    .bind(authority_id)
    .bind(&matched_menu_ids)
    .fetch_optional(pool)
    .await?;

    Ok(if allowed.is_some() {
        PermissionAccessDecision::Allowed
    } else {
        PermissionAccessDecision::Denied
    })
}

pub async fn get_permissions_by_authority_id(
    pool: &sqlx::PgPool,
    authority_id: i64,
) -> Result<Vec<String>, MenuError> {
    let permissions = sqlx::query_scalar(
        r#"
        select m.permission
        from sys_role_menus rm
        inner join sys_menus m on m.id = rm.menu_id
        where rm.authority_id = $1
          and m.menu_type = 'action'
          and m.permission is not null
        order by m.permission
        "#,
    )
    .bind(authority_id)
    .fetch_all(pool)
    .await?;

    Ok(permissions)
}

#[derive(Debug, Clone, Serialize)]
pub struct AssignedMenu {
    #[serde(rename = "menuId")]
    pub menu_id: i64,
    #[serde(rename = "parentId")]
    pub parent_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct MenuRoleSelection {
    #[serde(rename = "authorityIds")]
    pub authority_ids: Vec<i64>,
    #[serde(rename = "defaultRouterAuthorityIds")]
    pub default_router_authority_ids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MenuRoleMatrixItem {
    #[serde(rename = "menuId")]
    pub menu_id: i64,
    #[serde(rename = "authorityIds")]
    pub authority_ids: Vec<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn menu_record(id: i64, parent_id: i64, name: &str) -> MenuRecord {
        MenuRecord {
            id,
            parent_id,
            path: name.to_string(),
            name: name.to_string(),
            hidden: false,
            component: format!("view/{name}.vue"),
            sort: id as i32,
            active_name: String::new(),
            keep_alive: false,
            default_menu: false,
            title: name.to_string(),
            icon: String::new(),
            close_tab: false,
            transition_type: String::new(),
            parameters: Some(serde_json::json!([])),
            menu_btn: Some(serde_json::json!([])),
            menu_type: "page".to_string(),
            permission: None,
            method: None,
            api_path: None,
        }
    }

    #[test]
    fn keeps_ancestors_for_authorized_child_menus() {
        let rows = vec![menu_record(1, 0, "system"), menu_record(2, 1, "users")];

        let filtered = filter_authorized_with_ancestors(&rows, &[2]);
        let tree = build_tree(&filtered, 0);

        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].name, "system");
        assert_eq!(tree[0].children.len(), 1);
        assert_eq!(tree[0].children[0].name, "users");
    }

    #[test]
    fn navigation_hides_pages_without_authorized_actions() {
        let mut users = menu_record(1, 0, "users");
        let mut list = menu_record(2, 1, "users:list");
        list.menu_type = "action".to_string();
        list.permission = Some("system:user:list".to_string());

        let visible =
            filter_navigation_rows(&[users.clone()], &[users.clone(), list.clone()], &[1]);
        assert!(visible.is_empty());

        users.menu_type = "page".to_string();
        let visible =
            filter_navigation_rows(&[users.clone(), list.clone()], &[users, list], &[1, 2]);
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].name, "users");
    }

    #[test]
    fn route_pattern_matches_dynamic_permission_paths() {
        assert!(route_pattern_matches("/api/users/{id}", "/api/users/2"));
        assert!(route_pattern_matches(
            "/api/roles/{authority_id}/permission",
            "/api/roles/1/permission",
        ));
        assert!(!route_pattern_matches(
            "/api/users/{id}",
            "/api/users/2/password/reset",
        ));
    }

    #[test]
    fn static_paths_do_not_match_dynamic_patterns_without_dynamic_segments() {
        assert!(!is_dynamic_path_pattern("/api/routes/batch"));
        assert!(is_dynamic_path_pattern("/api/routes/{id}"));
    }

    #[test]
    fn matching_permission_ids_prefers_exact_paths_over_dynamic_patterns() {
        let candidates = vec![
            RegisteredPermissionPath {
                id: 1,
                api_path: "/api/routes/{id}".to_string(),
            },
            RegisteredPermissionPath {
                id: 2,
                api_path: "/api/routes/batch".to_string(),
            },
        ];

        assert_eq!(
            matching_permission_ids(&candidates, "/api/routes/batch"),
            vec![2]
        );
        assert_eq!(
            matching_permission_ids(&candidates, "/api/routes/9"),
            vec![1]
        );
    }
}
