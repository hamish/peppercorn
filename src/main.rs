use peppercorn::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listner = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind random port");
    run(listner)?.await
}
