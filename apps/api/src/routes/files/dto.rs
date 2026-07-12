use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct FileResponse {
    #[serde(rename = "id")]
    pub id: i64,
    pub name: String,
    pub url: String,
    pub tag: String,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,
    #[serde(rename = "classId")]
    pub class_id: i64,
}
impl From<file_storage::files::StoredFile> for FileResponse {
    fn from(v: file_storage::files::StoredFile) -> Self {
        Self {
            id: v.id,
            name: v.name,
            url: v.url,
            tag: v.tag,
            updated_at: v.updated_at,
            class_id: v.class_id,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FileListQuery {
    pub page: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    pub keyword: Option<String>,
    #[serde(rename = "classId")]
    pub class_id: Option<i64>,
}
impl From<FileListQuery> for file_storage::files::FileListQuery {
    fn from(v: FileListQuery) -> Self {
        Self {
            page: v.page,
            page_size: v.page_size,
            keyword: v.keyword,
            class_id: v.class_id,
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct FileEditPayload {
    #[serde(rename = "id")]
    pub id: i64,
    pub name: String,
}
impl From<FileEditPayload> for file_storage::files::FileEditPayload {
    fn from(v: FileEditPayload) -> Self {
        Self {
            id: v.id,
            name: v.name,
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct FileDeletePayload {
    #[serde(rename = "id")]
    pub id: i64,
}
#[derive(Debug, Deserialize)]
pub struct ImportUrlPayload {
    pub name: String,
    pub url: String,
    #[serde(rename = "classId")]
    pub class_id: Option<i64>,
}
impl From<ImportUrlPayload> for file_storage::files::ImportUrlPayload {
    fn from(v: ImportUrlPayload) -> Self {
        Self {
            name: v.name,
            url: v.url,
            class_id: v.class_id,
        }
    }
}
