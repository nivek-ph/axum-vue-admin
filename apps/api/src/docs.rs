use serde::{Deserialize, Serialize};
use utoipa::{
    Modify, OpenApi, ToSchema,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthData {
    pub alive: bool,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub code: String,
    pub data: HealthData,
    pub message: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CaptchaData {
    #[schema(value_type = i32)]
    #[serde(rename = "captchaLength")]
    pub captcha_length: i32,
    #[serde(rename = "picPath")]
    pub pic_path: String,
    #[serde(rename = "captchaId")]
    pub captcha_id: String,
    #[serde(rename = "openCaptcha")]
    pub open_captcha: bool,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CaptchaResponse {
    pub code: String,
    pub data: CaptchaData,
    pub message: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AuthorityDoc {
    #[serde(rename = "authorityId")]
    pub authority_id: i64,
    #[serde(rename = "authorityName")]
    pub authority_name: String,
    #[serde(rename = "parentId")]
    pub parent_id: i64,
    #[serde(rename = "defaultRouter")]
    pub default_router: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserDoc {
    #[serde(rename = "ID")]
    pub id: i64,
    pub uuid: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "nickName")]
    pub nick_name: String,
    #[serde(rename = "headerImg")]
    pub header_img: String,
    pub authority: AuthorityDoc,
    pub authorities: Vec<AuthorityDoc>,
    pub enable: i32,
    pub phone: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginData {
    pub token: String,
    pub user: UserDoc,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub code: String,
    pub data: LoginData,
    pub message: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserInfoData {
    #[serde(rename = "userInfo")]
    pub user_info: UserDoc,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserInfoResponse {
    pub code: String,
    pub data: UserInfoData,
    pub message: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MenuMetaDoc {
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

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MenuDoc {
    #[serde(rename = "ID")]
    pub id: i64,
    #[serde(rename = "parentId")]
    pub parent_id: i64,
    pub path: String,
    pub name: String,
    pub hidden: bool,
    pub component: String,
    pub sort: i32,
    pub meta: MenuMetaDoc,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MenuData {
    pub menus: Vec<MenuDoc>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MenuResponse {
    pub code: String,
    pub data: MenuData,
    pub message: String,
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    paths(
        crate::routes::public::health::health,
        crate::routes::public::captcha::captcha,
        crate::routes::public::auth::login,
        crate::routes::protected::user::get_user_info,
        crate::routes::protected::menu::get_menu,
    ),
    components(
        schemas(
            HealthData,
            HealthResponse,
            CaptchaData,
            CaptchaResponse,
            system::users::LoginRequest,
            AuthorityDoc,
            UserDoc,
            LoginData,
            LoginResponse,
            UserInfoData,
            UserInfoResponse,
            MenuMetaDoc,
            MenuDoc,
            MenuData,
            MenuResponse
        )
    ),
    tags(
        (name = "auth", description = "Auth endpoints"),
        (name = "user", description = "User endpoints"),
        (name = "menu", description = "Menu endpoints"),
        (name = "system", description = "System endpoints")
    )
)]
pub struct ApiDoc;
