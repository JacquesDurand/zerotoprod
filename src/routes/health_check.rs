use actix_web::http::StatusCode;
use actix_web::{get, HttpResponse};

#[get("/health_check")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::new(StatusCode::OK)
}
