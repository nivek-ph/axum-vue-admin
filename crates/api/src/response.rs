use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub code: String,
    pub message: String,
    pub data: Option<T>,
}

/// Schema marker for stable envelopes whose contract has no data payload.
#[derive(Debug, Serialize, ToSchema)]
pub struct NoData {}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            code: "OK".to_string(),
            message: "ok".to_string(),
            data: Some(data),
        }
    }

    pub fn new(code: impl Into<String>, message: impl Into<String>, data: Option<T>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            data,
        }
    }
}

pub type ApiErrorResponse = ApiResponse<NoData>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutation_response_has_the_shared_null_data_contract() {
        let value = serde_json::to_value(ApiResponse::<NoData>::new("OK", "saved", None))
            .expect("mutation response should serialize");

        assert_eq!(value["code"], "OK");
        assert_eq!(value["message"], "saved");
        assert!(value["data"].is_null());
    }
}
