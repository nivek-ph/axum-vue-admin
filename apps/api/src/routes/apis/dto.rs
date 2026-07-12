use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ApiItemResponse {
    #[serde(rename = "id")]
    pub id: i64,
    pub path: String,
    pub description: String,
    #[serde(rename = "apiGroup")]
    pub api_group: String,
    pub method: String,
}
impl From<iam::apis::ApiRecord> for ApiItemResponse {
    fn from(v: iam::apis::ApiRecord) -> Self {
        Self {
            id: v.id,
            path: v.path,
            description: v.description,
            api_group: v.api_group,
            method: v.method,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiRoleSelectionResponse {
    #[serde(rename = "roleIds")]
    pub authority_ids: Vec<i64>,
}
impl From<iam::apis::ApiRoleSelection> for ApiRoleSelectionResponse {
    fn from(v: iam::apis::ApiRoleSelection) -> Self {
        Self {
            authority_ids: v.authority_ids,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiRoleMatrixResponse {
    pub path: String,
    pub method: String,
    #[serde(rename = "roleIds")]
    pub authority_ids: Vec<i64>,
}
impl From<iam::apis::ApiRoleMatrixItem> for ApiRoleMatrixResponse {
    fn from(v: iam::apis::ApiRoleMatrixItem) -> Self {
        Self {
            path: v.path,
            method: v.method,
            authority_ids: v.authority_ids,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchApiRequest {
    pub page: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    pub path: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "apiGroup")]
    pub api_group: Option<String>,
    pub method: Option<String>,
    #[serde(rename = "orderKey")]
    pub order_key: Option<String>,
    pub desc: Option<bool>,
}
impl From<SearchApiRequest> for iam::apis::SearchApiRequest {
    fn from(v: SearchApiRequest) -> Self {
        Self {
            page: v.page,
            page_size: v.page_size,
            path: v.path,
            description: v.description,
            api_group: v.api_group,
            method: v.method,
            order_key: v.order_key,
            desc: v.desc,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ApiIdRequest {
    #[serde(rename = "id")]
    pub id: i64,
}
#[derive(Debug, Deserialize)]
pub struct DeleteApisByIdsRequest {
    pub ids: Vec<i64>,
}
#[derive(Debug, Deserialize)]
pub struct ApiPayload {
    #[serde(rename = "id", default)]
    pub id: i64,
    pub path: String,
    pub description: String,
    #[serde(rename = "apiGroup")]
    pub api_group: String,
    pub method: String,
}
impl From<ApiPayload> for iam::apis::ApiPayload {
    fn from(v: ApiPayload) -> Self {
        Self {
            id: v.id,
            path: v.path,
            description: v.description,
            api_group: v.api_group,
            method: v.method,
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct ApiRoleQuery {
    pub path: String,
    pub method: String,
}
#[derive(Debug, Deserialize)]
pub struct AuthorityApiQuery {
    #[serde(rename = "authorityId")]
    pub authority_id: i64,
}
#[derive(Debug, Deserialize)]
pub struct SetApiRolesRequest {
    pub path: String,
    pub method: String,
    #[serde(rename = "roleIds")]
    pub authority_ids: Vec<i64>,
}
