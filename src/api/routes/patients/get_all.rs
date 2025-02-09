use crate::api::api_error::ApiError;
use crate::application::{CommandHandler, GetPatientsCommand};
use crate::domain::Patient;
use crate::PatientResponse;
use actix_web::{get, web, HttpResponse};

#[utoipa::path(
    description = "Get all patients",
    responses(
        (status = 200, description = "All patients", body = Vec<PatientResponse>)
    )
)]
#[get("/patients")]
#[tracing::instrument(name = "Get all patients", skip(handler))]
pub async fn get_all_patients(
    handler: web::Data<Box<dyn CommandHandler<GetPatientsCommand, Vec<Patient>>>>,
) -> Result<HttpResponse, ApiError> {
    let result = handler.handle_command(GetPatientsCommand()).await;

    match result {
        Ok(patients) => Ok(HttpResponse::Ok().json(
            patients
                .into_iter()
                .map(PatientResponse::from)
                .collect::<Vec<PatientResponse>>(),
        )),
        Err(_) => Err(ApiError::InternalServerError),
    }
}
