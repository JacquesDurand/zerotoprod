use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{App, HttpServer};

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();

    Ok(server)
}
