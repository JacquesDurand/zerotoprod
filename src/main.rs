use std::net::TcpListener;
use zerotoprod::configuration::get_configuration;

use zerotoprod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read config file");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))
        .expect("Failed to bind address");
    run(listener)?.await
}
