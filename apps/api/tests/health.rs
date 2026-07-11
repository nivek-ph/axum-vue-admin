use axum::{
    body::Body,
    http::{Method, Request, header},
};
use std::sync::Arc;
use tower::ServiceExt;

fn test_state() -> api::state::AppState {
    let database_url = "postgres://postgres:postgres@localhost/axum_vue_admin";
    api::state::AppState {
        config: Arc::new(api::state::AppConfig {
            http_port: 3000,
            database_url: database_url.to_string(),
            redis_url: "redis://127.0.0.1:6379/".to_string(),
            jwt_secret: "test-secret".to_string(),
        }),
        pool: db::DbPool::connect_lazy(database_url).expect("lazy pool should construct"),
        auth_session_service: api::auth::session::AuthSessionService::without_revocation_store(
            "test-secret",
        ),
        password_service: auth::password::PasswordService::new(),
    }
}

#[tokio::test]
async fn health_route_returns_ok_response_body() {
    let app = api::router::build_router(test_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("router should produce a response");

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn swagger_ui_route_is_available() {
    let app = api::router::build_router(test_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/swagger-ui/")
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("router should produce a response");

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn removed_non_core_routes_return_not_found() {
    let app = api::router::build_router(test_state());
    let requests = [
        ("/api/autoCode/getDB", "GET"),
        ("/api/customer/customerList?page=1&pageSize=10", "GET"),
        (
            "/api/sysVersion/getSysVersionList?page=1&pageSize=10",
            "GET",
        ),
        ("/api/sysError/getSysErrorList?page=1&pageSize=10", "GET"),
        (
            "/api/sysExportTemplate/getSysExportTemplateList?page=1&pageSize=10",
            "GET",
        ),
        ("/api/sysApiToken/getApiTokenList", "POST"),
    ];

    for (uri, method) in requests {
        let request = Request::builder()
            .method(method)
            .uri(uri)
            .body(Body::empty())
            .expect("request should build");
        let response = app
            .clone()
            .oneshot(request)
            .await
            .expect("router should produce a response");

        assert_eq!(
            response.status(),
            404,
            "route {method} {uri} should be removed"
        );
    }
}

#[tokio::test]
async fn cors_preflight_allows_desktop_dev_origin() {
    let app = api::router::build_router(test_state());
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/api/auth/login")
                .header(header::ORIGIN, "http://localhost:5173")
                .header(header::ACCESS_CONTROL_REQUEST_METHOD, "POST")
                .header(header::ACCESS_CONTROL_REQUEST_HEADERS, "content-type")
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("router should produce a response");

    assert_eq!(response.status(), 200);
    assert_eq!(
        response.headers().get(header::ACCESS_CONTROL_ALLOW_ORIGIN),
        Some(&header::HeaderValue::from_static("*"))
    );
}
