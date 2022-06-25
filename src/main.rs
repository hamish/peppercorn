use peppercorn::configuration::get_configuration;
use peppercorn::startup::run;
use peppercorn::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("peppercorn".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Filed to get config");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&configuration.database.connection_string().expose_secret())
        .expect("Failed to Connect to the DB.");
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listner = TcpListener::bind(address)?;
    run(listner, connection_pool)?.await
}
