use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::http::StatusCode;
use actix_web::web::Form;
use actix_web::{get, post, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubscribeInput {
    email: String,
    name: String,
}

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}

#[post("/subscribe")]
async fn subscribe(_req: Form<SubscribeInput>) -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();

    Ok(server)
}
