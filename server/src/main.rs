use capstone::configs::ApiConfig;
use capstone::errors::ApiErrors;
use capstone::startup::startup;
use config::Config;
use sqlx::PgPool;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

#[tokio::main]
async fn main() -> Result<(), ApiErrors> {
    let builder = Config::builder().add_source(config::Environment::with_prefix("API_"));

    let config = builder.build().map_err(ApiErrors::ConfigError)?;
    let config = config
        .try_deserialize::<ApiConfig>()
        .map_err(ApiErrors::ConfigError)?;

    LogTracer::init().expect("Failed to set logger");

    let pool = PgPool::connect(&config.db).await.map_err(ApiErrors::Sqlx)?;

    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");

    let server = startup(pool, config).await?;

    server.await.map_err(ApiErrors::IoError)
}
