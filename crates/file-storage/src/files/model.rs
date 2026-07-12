use sqlx::FromRow;
#[derive(Debug, Clone, FromRow)]
pub struct StoredFile {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub tag: String,
    pub updated_at: String,
    pub class_id: i64,
}
