use peppercorn::configuration::get_configuration;
use peppercorn::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    LogTracer::init().expect("Failed to set Logger");
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let formating_layer = BunyanFormattingLayer::new(
        "peppercorn".into(),
        std::io::stdout
    );
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formating_layer);
    set_global_default(subscriber).expect("Failed to tet Subscriber");

    let configuration = get_configuration().expect("Filed to get config");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to Connect to the DB.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listner = TcpListener::bind(address)?;
    run(listner, connection_pool)?.await
}
