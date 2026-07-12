use sqlx::FromRow;
#[derive(Debug, Clone)]
pub struct AttachmentCategory {
    pub id: i64,
    pub name: String,
    pub pid: i64,
    pub children: Vec<AttachmentCategory>,
}
#[derive(Debug, Clone, FromRow)]
pub(super) struct AttachmentCategoryRow {
    pub id: i64,
    pub name: String,
    pub pid: i64,
}
