use patient_api::Application;

pub async fn run_app(port: u16) {
    let application = Application::start("127.0.0.1", port)
        .await
        .expect("Failed to build application.");
    tokio::spawn(application.run_until_stopped());
}
