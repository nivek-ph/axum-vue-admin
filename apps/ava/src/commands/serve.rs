use std::time::Duration;

use anyhow::{Context, Result};
use audit::{AuditAnalyzer, AuditService};
use auth::{captcha::CaptchaService, password::PasswordService, token::TokenService};
use file_storage::files::FileService;
use iam::{
    access::AccessService, departments::DepartmentService, menus::MenuService, roles::RoleService,
    users::UserService,
};
use metadata::{dictionaries::DictionaryService, parameters::ParameterService};
use tracing::info;

use crate::config::ServeConfig;

pub async fn run(config: ServeConfig) -> Result<()> {
    let server_config = config.api_server();
    let (pool, redis_connection) = connect_resources(&config).await?;

    info!("running database migrations");
    db::migrate(&pool)
        .await
        .context("database migrations should run")?;
    info!("database migrations complete");

    let state = build_state(&config, pool, redis_connection).await?;
    api::serve(server_config, state)
        .await
        .context("api server should run")
}

pub async fn serverless_state(config: &ServeConfig) -> Result<api::AppState> {
    let (pool, redis_connection) = connect_resources(config).await?;
    build_state(config, pool, redis_connection).await
}

async fn connect_resources(
    config: &ServeConfig,
) -> Result<(db::DbPool, redis::aio::MultiplexedConnection)> {
    let pool = db::connect(&config.database_url)
        .await
        .context("database pool should connect")?;
    info!("database connected");

    let redis_client =
        redis::Client::open(config.redis_url.clone()).context("redis client should construct")?;
    let redis_config = redis::AsyncConnectionConfig::new()
        .set_connection_timeout(Some(Duration::from_secs(10)))
        .set_response_timeout(Some(Duration::from_secs(5)));
    let redis_connection = redis_client
        .get_multiplexed_async_connection_with_config(&redis_config)
        .await
        .context("redis connection should connect")?;
    Ok((pool, redis_connection))
}

async fn build_state(
    config: &ServeConfig,
    pool: db::DbPool,
    redis_connection: redis::aio::MultiplexedConnection,
) -> Result<api::AppState> {
    let password_service = PasswordService::new();
    let tokens = TokenService::new(&config.jwt_secret, redis_connection.clone());
    let captcha = CaptchaService::new(redis_connection.clone());
    let access = AccessService::load(pool.clone(), redis_connection)
        .await
        .context("authorization catalog and cache should initialize")?;
    let audit = AuditService::new(pool.clone());
    let users = UserService::new(
        pool.clone(),
        access.clone(),
        audit.clone(),
        password_service,
    );
    Ok(api::AppState {
        public_base_url: config.public_base_url.clone(),
        tokens,
        captcha,
        users,
        roles: RoleService::new(pool.clone(), access.clone()),
        departments: DepartmentService::new(pool.clone(), access.clone()),
        access,
        dictionaries: DictionaryService::new(pool.clone()),
        parameters: ParameterService::new(pool.clone()),
        menus: MenuService::new(pool.clone()),
        audits: audit,
        audit_analyzer: AuditAnalyzer::new(
            config.ollama_base_url.clone(),
            config.ollama_model.clone(),
        ),
        files: FileService::new(pool, "./uploads"),
    })
}
