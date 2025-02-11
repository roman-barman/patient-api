use crate::api::api_error::ApiError;
use crate::application::{CommandHandler, UpdatePatientCommand, UpdatePatientError};
use crate::domain::Gender;
use actix_web::{put, web, HttpResponse};
use uuid::Uuid;

#[derive(serde::Deserialize, Debug, utoipa::ToSchema)]
pub struct UpdateRequestName {
    family: String,
    given: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Debug, utoipa::ToSchema)]
pub struct UpdateRequestPatient {
    name: UpdateRequestName,
    gender: Option<Gender>,
    birth_date: String,
    active: bool,
    version: i64,
}

#[utoipa::path(
    description = "Update a patient",
    responses(
        (status = 204, description = "Patient was updated"),
        (status = 400, description = "Invalid request body", body = String, content_type = "text/plain; charset=utf-8"),
        (status = 404, description = "Patient was not found", body = String, content_type = "text/plain; charset=utf-8"),
        (status = 409, description = "Patient has been already updated", body = String, content_type = "text/plain; charset=utf-8")
    ),
    request_body(content = UpdateRequestPatient, description = "Patient to update in the API", content_type = "application/json"),
    params(
        ("patient_id", description = "Patient ID"),
    )
)]
#[put("/patients/{patient_id}")]
#[tracing::instrument(name = "Update a patient", skip(handler))]
pub async fn update_patient(
    handler: web::Data<Box<dyn CommandHandler<UpdatePatientCommand, ()>>>,
    path: web::Path<Uuid>,
    request: web::Json<UpdateRequestPatient>,
) -> Result<HttpResponse, ApiError> {
    let command = create_command(request.into_inner(), path.into_inner());
    let result = handler.handle_command(command).await;

    match result {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(err) => {
            if let Some(expected_error) = err.root_cause().downcast_ref::<UpdatePatientError>() {
                match expected_error {
                    UpdatePatientError::InvalidArguments(e) => {
                        Err(ApiError::BadRequest(e.to_string()))
                    }
                    UpdatePatientError::PatientDoesNotExist => Err(ApiError::NotFound),
                    UpdatePatientError::PatientVersionWasChanged => Err(ApiError::Conflict),
                }
            } else {
                Err(ApiError::InternalServerError)
            }
        }
    }
}

fn create_command(request: UpdateRequestPatient, id: Uuid) -> UpdatePatientCommand {
    UpdatePatientCommand::new(
        id,
        request.name.family,
        request.name.given,
        request.gender,
        request.birth_date,
        request.active,
        request.version,
    )
}
