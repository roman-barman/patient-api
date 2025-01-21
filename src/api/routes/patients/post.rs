use actix_web::{post, HttpResponse};

#[post("/patients")]
pub async fn create_patient() -> HttpResponse {
    HttpResponse::Ok().finish()
}
