use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(
    crate::api::routes::create_patient,
    crate::api::routes::get_patient,
    crate::api::routes::update_patient,
    crate::api::routes::delete_patient,
    crate::api::routes::get_all_patients,
))]
pub struct ApiDoc;
