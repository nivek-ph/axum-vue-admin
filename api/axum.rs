use ava::{commands::serve, config::ServeConfig};
use tower::ServiceBuilder;
use vercel_runtime::{Error, axum::VercelLayer};

#[tokio::main]
async fn main() -> Result<(), Error> {
    ava::config::install_crypto_provider();
    let config = ServeConfig::from_env()?;
    let state = serve::serverless_state(&config).await?;
    let app = ServiceBuilder::new()
        .layer(VercelLayer::new())
        .service(api::router(state));
    vercel_runtime::run(app).await
}
