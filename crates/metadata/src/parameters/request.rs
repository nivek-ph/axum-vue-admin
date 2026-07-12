#[derive(Debug, Clone)]
pub struct ParamListQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub name: Option<String>,
    pub key: Option<String>,
}
