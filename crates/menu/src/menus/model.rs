use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone)]
pub struct MenuMeta {
    pub active_name: String,
    pub keep_alive: bool,
    pub default_menu: bool,
    pub title: String,
    pub icon: String,
    pub close_tab: bool,
    pub transition_type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuParameter {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "sysBaseMenuId")]
    pub sys_base_menu_id: i64,
    #[serde(rename = "type")]
    pub parameter_type: String,
    pub key: String,
    pub value: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuButton {
    #[serde(rename = "id")]
    pub id: i64,
    pub name: String,
    pub desc: String,
}
#[derive(Debug, Clone)]
pub struct MenuView {
    pub id: i64,
    pub parent_id: i64,
    pub path: String,
    pub name: String,
    pub hidden: bool,
    pub component: String,
    pub sort: i32,
    pub meta: MenuMeta,
    pub parameters: Vec<MenuParameter>,
    pub menu_btn: Vec<MenuButton>,
    pub menu_type: String,
    pub permission: Option<String>,
    pub permission_id: Option<i64>,
    pub method: Option<String>,
    pub api_path: Option<String>,
    pub children: Vec<MenuView>,
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
    pub permission_id: Option<i64>,
    pub method: Option<String>,
    pub api_path: Option<String>,
}
#[derive(Debug, Clone)]
pub struct MenuRoleSelection {
    pub authority_ids: Vec<i64>,
}
