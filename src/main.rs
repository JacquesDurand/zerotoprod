use std::net::TcpListener;

use zerotoprod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind address");
    run(listener)?.await
}
