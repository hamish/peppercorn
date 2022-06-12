use peppercorn::configuration::get_configuration;
use peppercorn::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Filed to get config");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to Connect to the DB.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listner = TcpListener::bind(address)?;
    run(listner, connection_pool)?.await
}
