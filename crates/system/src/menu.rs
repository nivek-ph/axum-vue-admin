use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use admin_httpz::AppError;

use crate::errors;

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
    pub children: Vec<MenuView>,
}

pub fn default_menus() -> Vec<MenuView> {
    vec![MenuView {
        id: 1,
        parent_id: 0,
        path: "dashboard".to_string(),
        name: "dashboard".to_string(),
        hidden: false,
        component: "view/dashboard/index.vue".to_string(),
        sort: 1,
        meta: MenuMeta {
            active_name: String::new(),
            keep_alive: false,
            default_menu: false,
            title: "Dashboard".to_string(),
            icon: "odometer".to_string(),
            close_tab: false,
            transition_type: String::new(),
        },
        parameters: Vec::new(),
        menu_btn: Vec::new(),
        children: Vec::new(),
    }]
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
}

impl From<MenuError> for AppError {
    fn from(error: MenuError) -> Self {
        match error {
            MenuError::NotFound => errors::menu::MENU_NOT_FOUND.into(),
            MenuError::Database(error) => {
                errors::menu::MENU_DB_FAILED.into_error().with_source(error)
            }
            MenuError::InvalidPayload => errors::menu::MENU_INVALID_PAYLOAD.into(),
        }
    }
}

pub async fn ensure_default_menu(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        insert into sys_menus (
            id, parent_id, path, name, hidden, component, sort,
            active_name, keep_alive, default_menu, title, icon, close_tab, transition_type,
            parameters, menu_btn
        ) values (
            1, 0, 'dashboard', 'dashboard', false, 'view/dashboard/index.vue', 1,
            '', false, false, 'Dashboard', 'odometer', false, '',
            '[]'::jsonb, '[]'::jsonb
        )
        on conflict (id) do update
        set parent_id = excluded.parent_id,
            path = excluded.path,
            name = excluded.name,
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
            menu_btn = excluded.menu_btn
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        insert into sys_role_menus (authority_id, menu_id)
        values (888, 1)
        on conflict do nothing
        "#,
    )
    .execute(pool)
    .await?;

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
            m.menu_btn
        from sys_menus m
        order by m.sort asc, m.id asc
        "#,
    )
    .fetch_all(pool)
    .await?;
    let rows = filter_authorized_with_ancestors(&rows, &authorized_menu_ids);

    Ok(build_tree(&rows, 0))
}

pub async fn get_menu_list(pool: &sqlx::PgPool) -> Result<Vec<MenuView>, MenuError> {
    let rows = sqlx::query_as::<_, MenuRecord>(
        r#"
        select
            id, parent_id, path, name, hidden, component, sort, active_name, keep_alive,
            default_menu, title, icon, close_tab, transition_type, parameters, menu_btn
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
            parameters, menu_btn
        ) values (
            $1, $2, $3, $4, $5, $6,
            $7, $8, $9, $10, $11, $12, $13,
            $14, $15
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
            menu_btn = $15
        where id = $16
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
            default_menu, title, icon, close_tab, transition_type, parameters, menu_btn
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
            default_menu, title, icon, close_tab, transition_type, parameters, menu_btn
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
        children: Vec::new(),
    })
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
}
