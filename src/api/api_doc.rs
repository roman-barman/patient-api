use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(crate::api::routes::create_patient))]
pub struct ApiDoc;
