use std::sync::Arc;

use auth::{jwt::JwtService, password::PasswordService};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = api::state::AppConfig::from_env().expect("config should load from environment");
    info!(bind_addr = %config.bind_addr, "starting api bootstrap");
    info!("connecting to database");
    let pool = db::connect(&config.database_url)
        .await
        .expect("database pool should connect");
    info!("database connected");
    info!("running database migrations");
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("database migrations should run");
    info!("database migrations complete");
    system::authority::ensure_default_authority(&pool)
        .await
        .expect("default authority should be bootstrapped");
    system::menu::ensure_default_menu(&pool)
        .await
        .expect("default menu should be bootstrapped");
    system::api_registry::ensure_default_apis(&pool)
        .await
        .expect("default apis should be bootstrapped");
    let password_service = PasswordService::default();
    system::users::ensure_admin_user(
        &pool,
        &password_service,
        &config.admin_username,
        &config.admin_password,
        &config.admin_nickname,
    )
    .await
    .expect("admin user should be bootstrapped");

    let app = api::router::build_router(api::state::AppState {
        config: Arc::new(config.clone()),
        pool,
        jwt_service: JwtService::new(&config.jwt_secret),
        password_service,
    });
    info!(bind_addr = %config.bind_addr, "binding api listener");
    let listener = tokio::net::TcpListener::bind(&config.bind_addr)
        .await
        .expect("listener should bind");
    info!(listen_addr = %listener.local_addr().expect("listener should expose local addr"), "api server listening");

    axum::serve(listener, app).await.expect("server should run");
}
