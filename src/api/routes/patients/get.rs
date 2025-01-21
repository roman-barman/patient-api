use actix_web::{get, web, HttpResponse};

#[get("/patients/{patient_id}")]
pub async fn get_patient(_path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
