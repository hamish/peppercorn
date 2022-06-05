use std::net::TcpListener;
use peppercorn::startup::run;
//use peppercorn::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //let configuration = get_configuration().expect("Filed to get config");
    let address = format!("127.0.0.1:{}", 8080);
    let listner = TcpListener::bind(address)?;
    run(listner)?.await
}
