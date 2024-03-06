use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::{Pool, Postgres};

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener, pool: Pool<Postgres>) -> std::io::Result<Server> {
    let pg_pool = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(subscribe)
            .app_data(pg_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
