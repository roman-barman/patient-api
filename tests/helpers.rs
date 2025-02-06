use patient_api::{Application, Environment, Settings};
use reqwest::Response;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApplication {
    address: String,
    api_client: reqwest::Client,
}

impl TestApplication {
    pub async fn run_app() -> TestApplication {
        let settings = {
            let mut settings = Settings::read_configuration_with_env(Environment::Test)
                .expect("Failed to read configuration.");
            settings.database.database_name = Uuid::new_v4().to_string();
            settings
        };
        Self::configure_database(&settings).await;
        let application = Application::start(settings)
            .await
            .expect("Failed to build application.");
        let address = format!("http:/{}", application.get_address());
        tokio::spawn(application.run_until_stopped());

        TestApplication {
            address,
            api_client: reqwest::Client::new(),
        }
    }

    pub async fn create_patient<Body>(&self, body: &Body) -> Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(format!("{}/patients", &self.address))
            .json(body)
            .send()
            .await
            .expect("Could not send request to server")
    }

    pub async fn get_all_patients(&self) -> Response {
        self.api_client
            .get(format!("{}/patients", &self.address))
            .send()
            .await
            .expect("Could not send request to server")
    }

    pub async fn get_patient_by_id(&self, id: &Uuid) -> Response {
        self.api_client
            .get(format!("{}/patients/{}", &self.address, id))
            .send()
            .await
            .expect("Could not send request to server")
    }

    pub async fn update_patient<Body>(&self, id: &Uuid, body: &Body) -> Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .put(format!("{}/patients/{}", &self.address, id))
            .json(body)
            .send()
            .await
            .expect("Could not send request to server")
    }

    pub async fn delete_patient(&self, id: &Uuid) -> Response {
        self.api_client
            .delete(format!("{}/patients/{}", &self.address, id))
            .send()
            .await
            .expect("Could not send request to server")
    }

    async fn configure_database(setting: &Settings) {
        let maintenance_settings = {
            let mut maintenance_settings = setting.clone();
            maintenance_settings.database.database_name = "postgres".to_string();
            maintenance_settings
        };

        let mut connection =
            PgConnection::connect_with(&maintenance_settings.database.get_connection_options())
                .await
                .expect("Failed to connect to Postgres");

        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, setting.database.database_name).as_str())
            .await
            .expect("Failed to create database.");

        let connection_pool = PgPool::connect_with(setting.database.get_connection_options())
            .await
            .expect("Failed to connect to Postgres.");

        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database");
    }
}
