pub mod error;
pub mod response;

pub use error::{AppError, AppResult, ErrorKind, ErrorSpec, ErrorSpecExt, OptionAppExt};
pub use response::{ApiErrorResponse, ApiResponse};

#[cfg(test)]
mod tests {
    use axum::{body::to_bytes, response::IntoResponse};
    use serde_json::{Value, json};

    async fn response_json(response: axum::response::Response) -> Value {
        let body = response.into_body();
        let bytes = to_bytes(body, usize::MAX)
            .await
            .expect("response body should be readable");
        serde_json::from_slice(&bytes).expect("response body should be valid json")
    }

    #[test]
    fn ok_response_uses_frontend_compatible_shape() {
        let response = crate::response::ApiResponse::ok(serde_json::json!({ "alive": true }));
        let value = serde_json::to_value(response).expect("response should serialize");

        assert_eq!(value["code"], "OK");
        assert_eq!(value["message"], "ok");
        assert_eq!(value["data"]["alive"], true);
    }

    #[tokio::test]
    async fn bad_request_error_response_exposes_stable_semantics() {
        let response = crate::error::AppError::bad_request("id is required").into_response();
        let status = response.status();
        let value = response_json(response).await;

        assert_eq!(status, axum::http::StatusCode::BAD_REQUEST);
        assert_eq!(
            value,
            json!({
                "code": "BAD_REQUEST",
                "data": Value::Null,
                "message": "id is required",
            })
        );
    }

    #[tokio::test]
    async fn internal_error_response_masks_diagnostics_but_keeps_source() {
        let error = crate::error::AppError::internal("database connection failed")
            .with_source(std::io::Error::other("database offline"));
        assert!(std::error::Error::source(&error).is_some());
        let _ = error.span_trace();
        let response = error.into_response();
        let status = response.status();
        let value = response_json(response).await;

        assert_eq!(status, axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            value,
            json!({
                "code": "INTERNAL_SERVER_ERROR",
                "data": Value::Null,
                "message": "internal server error",
            })
        );
    }

    #[tokio::test]
    async fn conflict_error_can_override_stable_error_code() {
        let response = crate::error::AppError::conflict("role already exists")
            .with_code("ROLE_ALREADY_EXISTS")
            .into_response();
        let status = response.status();
        let value = response_json(response).await;

        assert_eq!(status, axum::http::StatusCode::CONFLICT);
        assert_eq!(value["code"], "ROLE_ALREADY_EXISTS");
        assert_eq!(value["message"], "role already exists");
    }

    #[test]
    fn error_spec_converts_into_app_error() {
        let error: crate::error::AppError =
            crate::error::ErrorSpec::not_found("USER_NOT_FOUND", "user not found").into();

        assert_eq!(error.status(), axum::http::StatusCode::NOT_FOUND);
        assert_eq!(error.code(), "USER_NOT_FOUND");
        assert_eq!(error.message(), "user not found");
    }

    #[test]
    fn error_spec_helpers_return_app_result() {
        use crate::{ErrorSpecExt, OptionAppExt};

        let spec = crate::error::ErrorSpec::conflict("EMAIL_ALREADY_USED", "email already used");

        let direct = spec.err::<()>().unwrap_err();
        let optional = Option::<u64>::None.ok_or_spec(spec).unwrap_err();

        assert_eq!(direct.code(), "EMAIL_ALREADY_USED");
        assert_eq!(optional.status(), axum::http::StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn error_spec_conflict_response_matches_api_envelope() {
        let error: crate::error::AppError =
            crate::error::ErrorSpec::conflict("USER_ALREADY_EXISTS", "user already exists").into();

        let response = error.into_response();
        let status = response.status();
        let value = response_json(response).await;

        assert_eq!(status, axum::http::StatusCode::CONFLICT);
        assert_eq!(
            value,
            json!({
                "code": "USER_ALREADY_EXISTS",
                "data": Value::Null,
                "message": "user already exists",
            })
        );
    }

    #[tokio::test]
    async fn internal_error_response_is_masked() {
        let error = crate::error::AppError::internal_error(anyhow::anyhow!("database detail"));
        assert_eq!(
            error.status(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        );
        let response = error.into_response();
        assert_eq!(
            response.status(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        );
        let value = response_json(response).await;
        assert_eq!(
            value,
            json!({
                "code": "INTERNAL_SERVER_ERROR",
                "data": Value::Null,
                "message": "internal server error",
            })
        );
    }

    #[tokio::test]
    async fn internal_with_source_then_with_code_keeps_source_and_masks_response() {
        let io_err = std::io::Error::other("root cause");
        let error = crate::error::AppError::internal("private diagnostic")
            .with_source(io_err)
            .with_code("CUSTOM");

        assert_eq!(error.code(), "CUSTOM");
        assert_eq!(
            error.status(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        );
        assert!(std::error::Error::source(&error).is_some());

        let response = error.into_response();
        let value = response_json(response).await;
        assert_eq!(value["code"], "CUSTOM");
        assert_eq!(value["message"], "internal server error");
        assert_eq!(value["data"], Value::Null);
    }

    #[tokio::test]
    async fn other_http_server_errors_are_masked() {
        let error = crate::error::AppError::new(
            axum::http::StatusCode::BAD_GATEWAY,
            "BAD_GATEWAY",
            "upstream exploded",
        );
        let response = error.into_response();
        let status = response.status();
        let value = response_json(response).await;
        assert_eq!(status, axum::http::StatusCode::BAD_GATEWAY);
        assert_eq!(value["code"], "BAD_GATEWAY");
        assert_eq!(value["message"], "internal server error");
    }
}
