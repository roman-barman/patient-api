use crate::telemetry::initialize_tracing_subscribe;
use lib_api::{Application, Settings};
use std::env;

mod telemetry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let variable = "APP_TRACING_ADDRESS";
    let env = env::var(variable);

    let tracing_address = match env {
        Ok(val) => Some(val),
        Err(_e) => None,
    };

    initialize_tracing_subscribe("info".into(), "patient-api".into(), tracing_address);
    let settings = Settings::read_configuration()?;
    let application = Application::start(settings).await?;
    let _ = tokio::spawn(application.run_until_stopped()).await?;

    Ok(())
}
