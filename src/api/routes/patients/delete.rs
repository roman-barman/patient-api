use crate::api::api_error::ApiError;
use crate::application::{CommandHandler, DeletePatientCommand};
use actix_web::{delete, web, HttpResponse};
use uuid::Uuid;

#[utoipa::path(
    description = "Delete patient by ID",
    responses(
        (status = 204, description = "Patient was deleted"),
        (status = 404, description = "Patient was not found", body = String, content_type = "text/plain; charset=utf-8")
    ),
    params(
        ("patient_id", description = "Patient ID"),
    )
)]
#[delete("/patients/{patient_id}")]
#[tracing::instrument(name = "Delete patient by ID", skip(handler))]
pub async fn delete_patient(
    handler: web::Data<Box<dyn CommandHandler<DeletePatientCommand, bool>>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, ApiError> {
    let command = DeletePatientCommand::new(path.into_inner());
    let result = handler.handle_command(command).await;

    match result {
        Ok(is_deleted) => {
            if is_deleted {
                Ok(HttpResponse::NoContent().finish())
            } else {
                Err(ApiError::NotFound)
            }
        }
        Err(_) => Err(ApiError::InternalServerError),
    }
}
