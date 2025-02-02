use crate::api::configuration::Settings;
use crate::api::routes::{create_patient, get_all_patients, get_patient, update_patient};
use crate::application::{CommandHandler, CreatePatientCommand, CreatePatientHandler, Repository};
use crate::domain::Patient;
use crate::infrastructure::PostgresRepository;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

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
        let repository = Box::new(PostgresRepository::new(pg_pool)) as Box<dyn Repository>;
        let handler = Box::new(CreatePatientHandler::new(repository))
            as Box<dyn CommandHandler<CreatePatientCommand, Patient>>;
        let server = run(listener, handler).await?;

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
    handler: Box<dyn CommandHandler<CreatePatientCommand, Patient>>,
) -> Result<Server, anyhow::Error> {
    let handler = web::Data::new(handler);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(get_all_patients)
            .service(get_patient)
            .service(create_patient)
            .service(update_patient)
            .app_data(handler.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
