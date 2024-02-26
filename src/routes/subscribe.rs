use actix_web::{HttpResponse, post};
use actix_web::http::StatusCode;
use actix_web::web::Form;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubscribeInput {
    email: String,
    name: String,
}

#[post("/subscribe")]
pub async fn subscribe(_req: Form<SubscribeInput>) -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}
