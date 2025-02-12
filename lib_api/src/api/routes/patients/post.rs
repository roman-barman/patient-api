use crate::api::api_error::ApiError;
use crate::api::PatientResponse;
use crate::application::{CommandHandler, CreatePatientCommand, CreatePatientValidationError};
use crate::domain::{Gender, Patient};
use actix_web::{post, web, HttpResponse};

#[derive(serde::Deserialize, Debug, utoipa::ToSchema)]
pub struct CreateRequestName {
    family: String,
    given: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Debug, utoipa::ToSchema)]
pub struct CreateRequestPatient {
    name: CreateRequestName,
    gender: Option<Gender>,
    birth_date: String,
    active: bool,
}

#[utoipa::path(
    description = "Create a new patient",
    responses(
        (status = 201, description = "Patient was created", body = PatientResponse),
        (status = 400, description = "Invalid request body", body = String, content_type = "text/plain; charset=utf-8")
    ),
    request_body(content = CreateRequestPatient, description = "Patient to store the API", content_type = "application/json"),
)]
#[post("/patients")]
#[tracing::instrument(name = "Adding a new patient", skip(handler))]
pub async fn create_patient(
    handler: web::Data<Box<dyn CommandHandler<CreatePatientCommand, Patient>>>,
    request: web::Json<CreateRequestPatient>,
) -> Result<HttpResponse, ApiError> {
    let patient = request.into_inner();
    let command = CreatePatientCommand::from(patient);
    let result = handler.handle_command(command).await;

    match result {
        Ok(patient) => Ok(HttpResponse::Created()
            .append_header(("Location", format!("/patients/{}", patient.name.id)))
            .json(PatientResponse::from(patient))),
        Err(e) => {
            if let Some(validation_error) = e
                .root_cause()
                .downcast_ref::<CreatePatientValidationError>()
            {
                Err(ApiError::BadRequest(validation_error.to_string()))
            } else {
                Err(ApiError::InternalServerError)
            }
        }
    }
}

impl From<CreateRequestPatient> for CreatePatientCommand {
    fn from(request_patient: CreateRequestPatient) -> Self {
        CreatePatientCommand::new(
            request_patient.name.family,
            request_patient.name.given,
            request_patient.gender,
            request_patient.birth_date,
            request_patient.active,
        )
    }
}
