use crate::api::api_doc::ApiDoc;
use crate::api::configuration::Settings;
use crate::api::{
    create_patient, delete_patient, get_all_patients, get_patient, health_check, update_patient,
};
use crate::application::{
    CommandHandler, CreatePatientCommand, CreatePatientHandler, DeletePatientCommand,
    DeletePatientHandler, GetPatientByIdCommand, GetPatientByIdHandler, GetPatientsCommand,
    GetPatientsHandler, Repository, UpdatePatientCommand, UpdatePatientHandler,
};
use crate::domain::Patient;
use crate::infrastructure::PostgresRepository;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct Application {
    server: Server,
    address: String,
}

impl Application {
    pub async fn start(settings: Settings) -> Result<Self, anyhow::Error> {
        let listener = TcpListener::bind(settings.application.get_application_address())?;
        let address = listener.local_addr()?.to_string();
        let pg_pool =
            PgPoolOptions::new().connect_lazy_with(settings.database.get_connection_options());
        let repository = Arc::new(PostgresRepository::new(pg_pool)) as Arc<dyn Repository>;
        let create_patient_handler = Box::new(CreatePatientHandler::new(repository.clone()))
            as Box<dyn CommandHandler<CreatePatientCommand, Patient>>;
        let get_patient_by_id_handler = Box::new(GetPatientByIdHandler::new(repository.clone()))
            as Box<dyn CommandHandler<GetPatientByIdCommand, Option<Patient>>>;
        let update_patient_handler = Box::new(UpdatePatientHandler::new(repository.clone()))
            as Box<dyn CommandHandler<UpdatePatientCommand, ()>>;
        let delete_patient_handler = Box::new(DeletePatientHandler::new(repository.clone()))
            as Box<dyn CommandHandler<DeletePatientCommand, bool>>;
        let get_all_patients_handler = Box::new(GetPatientsHandler::new(repository.clone()))
            as Box<dyn CommandHandler<GetPatientsCommand, Vec<Patient>>>;
        let server = run(
            listener,
            create_patient_handler,
            get_patient_by_id_handler,
            update_patient_handler,
            delete_patient_handler,
            get_all_patients_handler,
        )
        .await?;

        Ok(Self { server, address })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }
}

async fn run(
    listener: TcpListener,
    create_patient_handler: Box<dyn CommandHandler<CreatePatientCommand, Patient>>,
    get_patient_by_id_handler: Box<dyn CommandHandler<GetPatientByIdCommand, Option<Patient>>>,
    update_patient_handler: Box<dyn CommandHandler<UpdatePatientCommand, ()>>,
    delete_patient_handler: Box<dyn CommandHandler<DeletePatientCommand, bool>>,
    get_all_patients_handler: Box<dyn CommandHandler<GetPatientsCommand, Vec<Patient>>>,
) -> Result<Server, anyhow::Error> {
    let create_patient_handler = web::Data::new(create_patient_handler);
    let get_patient_by_id_handler = web::Data::new(get_patient_by_id_handler);
    let update_patient_handler = web::Data::new(update_patient_handler);
    let delete_patient_handler = web::Data::new(delete_patient_handler);
    let get_all_patients_handler = web::Data::new(get_all_patients_handler);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(get_all_patients)
            .service(get_patient)
            .service(create_patient)
            .service(update_patient)
            .service(delete_patient)
            .service(health_check)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .app_data(create_patient_handler.clone())
            .app_data(get_patient_by_id_handler.clone())
            .app_data(update_patient_handler.clone())
            .app_data(delete_patient_handler.clone())
            .app_data(get_all_patients_handler.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
