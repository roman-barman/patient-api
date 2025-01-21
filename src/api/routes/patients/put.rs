use actix_web::{put, web, HttpResponse};

#[put("/patients/{patient_id}")]
pub async fn update_patient(_path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
