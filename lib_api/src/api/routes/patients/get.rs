use crate::api::api_error::ApiError;
use crate::api::PatientResponse;
use crate::application::{CommandHandler, GetPatientByIdCommand};
use crate::domain::Patient;
use actix_web::{get, web, HttpResponse};
use uuid::Uuid;

#[utoipa::path(
    description = "Get patient by ID",
    responses(
        (status = 200, description = "Patient was found", body = PatientResponse),
        (status = 404, description = "Patient was not found", body = String, content_type = "text/plain; charset=utf-8")
    ),
    params(
        ("patient_id", description = "Patient ID"),
    )
)]
#[get("/patients/{patient_id}")]
#[tracing::instrument(name = "Get patient by ID", skip(handler))]
pub async fn get_patient(
    handler: web::Data<Box<dyn CommandHandler<GetPatientByIdCommand, Option<Patient>>>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let command = GetPatientByIdCommand::new(path.into_inner());
    let result = handler.handle_command(command).await;

    match result {
        Ok(patient) => match patient {
            Some(patient) => Ok(HttpResponse::Ok().json(PatientResponse::from(patient))),
            None => Err(ApiError::NotFound),
        },
        Err(_) => Err(ApiError::InternalServerError),
    }
}
