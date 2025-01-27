use patient_api::{Application, Environment, Settings};
use reqwest::Response;

pub struct TestApplication {
    address: String,
    api_client: reqwest::Client,
}

impl TestApplication {
    pub async fn run_app() -> TestApplication {
        let settings = Settings::read_configuration_with_env(Environment::Test)
            .expect("Failed to read configuration.");
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

    pub async fn get_all_patients(&self) -> Response {
        self.api_client
            .get(format!("{}/patients", &self.address))
            .send()
            .await
            .expect("Could not send request to server")
    }

    pub async fn get_patient_by_id(&self, id: &str) -> Response {
        self.api_client
            .get(format!("{}/patients/{}", &self.address, id))
            .send()
            .await
            .expect("Could not send request to server")
    }

    pub async fn update_patient(&self, id: &str) -> Response {
        self.api_client
            .put(format!("{}/patients/{}", &self.address, id))
            .send()
            .await
            .expect("Could not send request to server")
    }
}
