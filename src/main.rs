use patient_api::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let application = Application::start("127.0.0.1", 8080).await?;
    let _ = tokio::spawn(application.run_until_stopped()).await?;

    Ok(())
}
