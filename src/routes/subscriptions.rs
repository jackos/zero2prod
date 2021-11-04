use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_from: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
