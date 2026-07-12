use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct MenuParameter {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "sysBaseMenuID")]
    pub menu_id: i64,
    #[serde(rename = "type")]
    pub parameter_type: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MenuButton {
    #[serde(rename = "ID")]
    pub id: i64,
    pub name: String,
    pub desc: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MenuPayload {
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
    #[serde(rename = "permissionId", default)]
    pub permission_id: Option<i64>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(rename = "apiPath", default)]
    pub api_path: Option<String>,
    #[serde(default)]
    pub children: Vec<MenuPayload>,
}

fn default_menu_type() -> String {
    "page".to_string()
}

impl From<MenuPayload> for menu::MenuView {
    fn from(v: MenuPayload) -> Self {
        Self {
            id: v.id,
            parent_id: v.parent_id,
            path: v.path,
            name: v.name,
            hidden: v.hidden,
            component: v.component,
            sort: v.sort,
            meta: menu::MenuMeta {
                active_name: v.meta.active_name,
                keep_alive: v.meta.keep_alive,
                default_menu: v.meta.default_menu,
                title: v.meta.title,
                icon: v.meta.icon,
                close_tab: v.meta.close_tab,
                transition_type: v.meta.transition_type,
            },
            parameters: v
                .parameters
                .into_iter()
                .map(|p| menu::MenuParameter {
                    id: p.id,
                    sys_base_menu_id: p.menu_id,
                    parameter_type: p.parameter_type,
                    key: p.key,
                    value: p.value,
                })
                .collect(),
            menu_btn: v
                .menu_btn
                .into_iter()
                .map(|b| menu::MenuButton {
                    id: b.id,
                    name: b.name,
                    desc: b.desc,
                })
                .collect(),
            menu_type: v.menu_type,
            permission: v.permission,
            permission_id: v.permission_id,
            method: v.method,
            api_path: v.api_path,
            children: v.children.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<menu::MenuView> for MenuPayload {
    fn from(v: menu::MenuView) -> Self {
        Self {
            id: v.id,
            parent_id: v.parent_id,
            path: v.path,
            name: v.name,
            hidden: v.hidden,
            component: v.component,
            sort: v.sort,
            meta: MenuMeta {
                active_name: v.meta.active_name,
                keep_alive: v.meta.keep_alive,
                default_menu: v.meta.default_menu,
                title: v.meta.title,
                icon: v.meta.icon,
                close_tab: v.meta.close_tab,
                transition_type: v.meta.transition_type,
            },
            parameters: v
                .parameters
                .into_iter()
                .map(|p| MenuParameter {
                    id: p.id,
                    menu_id: p.sys_base_menu_id,
                    parameter_type: p.parameter_type,
                    key: p.key,
                    value: p.value,
                })
                .collect(),
            menu_btn: v
                .menu_btn
                .into_iter()
                .map(|b| MenuButton {
                    id: b.id,
                    name: b.name,
                    desc: b.desc,
                })
                .collect(),
            menu_type: v.menu_type,
            permission: v.permission,
            permission_id: v.permission_id,
            method: v.method,
            api_path: v.api_path,
            children: v.children.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MenuRoleSelectionResponse {
    #[serde(rename = "roleIds")]
    pub authority_ids: Vec<i64>,
}
impl From<menu::MenuRoleSelection> for MenuRoleSelectionResponse {
    fn from(v: menu::MenuRoleSelection) -> Self {
        Self {
            authority_ids: v.authority_ids,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MenuIdRequest {
    #[serde(rename = "ID", alias = "id", alias = "menuId")]
    pub id: i64,
}
#[derive(Debug, Deserialize)]
pub struct SetMenuRolesRequest {
    #[serde(rename = "menuId")]
    pub menu_id: i64,
    #[serde(rename = "roleIds")]
    pub authority_ids: Vec<i64>,
}
