use std::sync::Arc;

use api::{auth::session::AuthSessionService, state::AppConfig};
use auth::password::PasswordService;

use anyhow::Result;
use tracing::info;
use tracing_otel_extra::Logger;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let logger = Logger::from_env(Some("LOG"))?.with_ansi(true);
    let _guard = logger.init()?;

    let config = AppConfig::from_env().expect("config should load from env");
    let pool = db::connect(&config.database_url)
        .await
        .expect("database pool should connect");
    info!("database connected");
    let redis_client =
        redis::Client::open(config.redis_url.clone()).expect("redis client should construct");
    let redis_connection = redis_client
        .get_multiplexed_async_connection()
        .await
        .expect("redis connection should connect");
    info!("running database migrations");
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("database migrations should run");
    info!("database migrations complete");

    let password_service = PasswordService::new();
    let auth_session_service = AuthSessionService::new(&config.jwt_secret, redis_connection);

    let app_state = api::state::AppState {
        config: Arc::new(config.clone()),
        pool,
        auth_session_service,
        password_service,
    };

    let app = api::router::build_router(app_state);
    let listener = tokio::net::TcpListener::bind(config.get_http_addr())
        .await
        .expect("listener should bind");
    let listen_addr = listener
        .local_addr()
        .expect("listener should expose local addr");
    info!(%listen_addr, swagger_url = %format!("http://127.0.0.1:{}/swagger-ui/", config.http_port), "api server listening");

    axum::serve(listener, app).await?;

    Ok(())
}
