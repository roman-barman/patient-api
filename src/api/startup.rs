use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use std::net::TcpListener;

pub struct Application {
    server: Server,
}

impl Application {
    pub async fn start() -> Result<Self, anyhow::Error> {
        let listener = TcpListener::bind("127.0.0.1:8080")?;
        let server = run(listener).await?;

        Ok(Self { server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(listener: TcpListener) -> Result<Server, anyhow::Error> {
    let server = HttpServer::new(App::new).listen(listener)?.run();

    Ok(server)
}
