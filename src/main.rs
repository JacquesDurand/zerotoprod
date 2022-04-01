use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zerotoprod::configuration::get_configuration;
use zerotoprod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let configuration = get_configuration().expect("Failed to read configuration file");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind address");

    let connection_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to the database");
    run(listener, connection_pool)?.await
}
