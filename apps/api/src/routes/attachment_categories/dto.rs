use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    #[serde(rename = "ID")]
    pub id: i64,
    pub name: String,
    pub pid: i64,
    pub children: Vec<CategoryResponse>,
}
impl From<file_storage::categories::AttachmentCategory> for CategoryResponse {
    fn from(v: file_storage::categories::AttachmentCategory) -> Self {
        Self {
            id: v.id,
            name: v.name,
            pid: v.pid,
            children: v.children.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CategoryPayload {
    #[serde(rename = "ID", default)]
    pub id: i64,
    pub name: String,
    pub pid: i64,
}
impl From<CategoryPayload> for file_storage::categories::CategoryPayload {
    fn from(v: CategoryPayload) -> Self {
        Self {
            id: v.id,
            name: v.name,
            pid: v.pid,
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct DeleteCategoryPayload {
    pub id: i64,
}
