use actix_web::http::StatusCode;
use actix_web::web::Form;
use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{query, PgPool};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct SubscribeInput {
    email: String,
    name: String,
}

#[post("/subscribe")]
pub async fn subscribe(req: Form<SubscribeInput>, connection: web::Data<PgPool>) -> HttpResponse {
    match query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        req.email,
        req.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(..) => HttpResponse::new(StatusCode::OK),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
