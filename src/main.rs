use patient_api::{Application, Settings};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::read_configuration()?;
    let application = Application::start(settings).await?;
    let _ = tokio::spawn(application.run_until_stopped()).await?;

    Ok(())
}
