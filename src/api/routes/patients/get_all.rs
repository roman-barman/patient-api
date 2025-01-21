use actix_web::{get, HttpResponse};

#[get("/patients")]
pub async fn get_all_patients() -> HttpResponse {
    HttpResponse::Ok().finish()
}
