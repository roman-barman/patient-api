use patient_api::{initialize_tracing_subscribe, Application, Settings};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize_tracing_subscribe("info".into(), "patient-api".into());
    let settings = Settings::read_configuration()?;
    let application = Application::start(settings).await?;
    let _ = tokio::spawn(application.run_until_stopped()).await?;

    Ok(())
}
