use tower_http::request_id::RequestId;

pub(crate) fn request_id_text(request_id: &RequestId) -> String {
    request_id
        .header_value()
        .to_str()
        .ok()
        .filter(|value| !value.is_empty())
        .unwrap_or("invalid-request-id")
        .to_string()
}

#[cfg(test)]
mod tests {
    use axum::http::HeaderValue;

    use super::*;

    #[test]
    fn converts_tower_request_id_for_audit_storage() {
        let request_id = RequestId::new(HeaderValue::from_static("req-123"));
        assert_eq!(request_id_text(&request_id), "req-123");
    }

    #[test]
    fn keeps_invalid_request_id_non_empty() {
        let request_id = RequestId::new(HeaderValue::from_bytes(b"\xff").unwrap());
        assert_eq!(request_id_text(&request_id), "invalid-request-id");
    }
}
