use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct DictionaryDetailResponse {
    #[serde(rename = "id")]
    pub id: i64,
    pub label: String,
    pub value: String,
    pub extend: String,
    pub status: Option<bool>,
    pub sort: i32,
    #[serde(rename = "sysDictionaryId")]
    pub dictionary_id: i64,
    #[serde(rename = "parentId")]
    pub parent_id: Option<i64>,
    pub level: i32,
    pub path: String,
    pub children: Vec<DictionaryDetailResponse>,
}
impl From<metadata::dictionaries::SysDictionaryDetail> for DictionaryDetailResponse {
    fn from(v: metadata::dictionaries::SysDictionaryDetail) -> Self {
        Self {
            id: v.id,
            label: v.label,
            value: v.value,
            extend: v.extend,
            status: v.status,
            sort: v.sort,
            dictionary_id: v.sys_dictionary_id,
            parent_id: v.parent_id,
            level: v.level,
            path: v.path,
            children: v.children.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DictionaryDetailPayload {
    #[serde(rename = "id", default)]
    pub id: i64,
    pub label: String,
    pub value: String,
    pub extend: String,
    pub status: Option<bool>,
    pub sort: i32,
    #[serde(rename = "sysDictionaryId")]
    pub dictionary_id: i64,
    #[serde(rename = "parentId")]
    pub parent_id: Option<i64>,
}

impl From<DictionaryDetailPayload> for metadata::dictionaries::SysDictionaryDetail {
    fn from(value: DictionaryDetailPayload) -> Self {
        Self {
            id: value.id,
            label: value.label,
            value: value.value,
            extend: value.extend,
            status: value.status,
            sort: value.sort,
            sys_dictionary_id: value.dictionary_id,
            parent_id: value.parent_id,
            level: 0,
            path: String::new(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct IdRequest {
    #[serde(rename = "id")]
    pub id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct DictionaryTreeQuery {
    #[serde(rename = "sysDictionaryId")]
    pub dictionary_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct DictionaryTypeQuery {
    #[serde(rename = "type")]
    pub dictionary_type: String,
}

#[derive(Debug, Deserialize)]
pub struct DictionaryParentQuery {
    #[serde(rename = "parentId")]
    pub parent_id: i64,
}
