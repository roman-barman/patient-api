use crate::telemetry::initialize_tracing_subscribe;
use lib_api::{Application, Settings};

mod telemetry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize_tracing_subscribe("info".into(), "patient-api".into());
    let settings = Settings::read_configuration()?;
    let application = Application::start(settings).await?;
    let _ = tokio::spawn(application.run_until_stopped()).await?;

    Ok(())
}
