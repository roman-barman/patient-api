use crate::api::configuration::Settings;
use crate::api::routes::{create_patient, get_all_patients, get_patient, update_patient};
use crate::application::{CommandHandler, CreatePatientCommand, CreatePatientHandler, Repository};
use crate::domain::Patient;
use crate::infrastructure::PostgresRepository;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub struct Application {
    server: Server,
}

impl Application {
    pub async fn start(address: &str, port: u16) -> Result<Self, anyhow::Error> {
        let settings = Settings::read_configuration()?;
        let pg_pool =
            PgPoolOptions::new().connect_lazy_with(settings.database.get_connection_string());
        let repository = Arc::new(PostgresRepository::new(pg_pool)) as Arc<dyn Repository + Sync>;
        let handler = Arc::new(CreatePatientHandler::new(repository.clone()))
            as Arc<dyn CommandHandler<CreatePatientCommand, Patient>>;
        let server = run(address, port, handler).await?;

        Ok(Self { server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(
    address: &str,
    port: u16,
    handler: Arc<dyn CommandHandler<CreatePatientCommand, Patient>>,
) -> Result<Server, anyhow::Error> {
    let handler = web::Data::new(handler);
    let server = HttpServer::new(move || {
        App::new()
            .service(get_all_patients)
            .service(get_patient)
            .service(create_patient)
            .service(update_patient)
            .app_data(handler.clone())
    })
    .bind((address, port))?
    .run();

    Ok(server)
}
