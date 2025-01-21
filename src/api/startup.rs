use crate::api::routes::{create_patient, get_all_patients, get_patient, update_patient};
use actix_web::dev::Server;
use actix_web::{App, HttpServer};

pub struct Application {
    server: Server,
}

impl Application {
    pub async fn start() -> Result<Self, anyhow::Error> {
        let server = run().await?;

        Ok(Self { server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run() -> Result<Server, anyhow::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .service(get_all_patients)
            .service(get_patient)
            .service(create_patient)
            .service(update_patient)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    Ok(server)
}
