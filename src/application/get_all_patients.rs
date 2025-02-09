use crate::application::{CommandHandler, Repository};
use crate::domain::Patient;
use async_trait::async_trait;
use std::sync::Arc;

pub struct GetPatientsHandler {
    repository: Arc<dyn Repository>,
}

impl GetPatientsHandler {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        Self { repository }
    }
}

#[derive(Debug)]
pub struct GetPatientsCommand();

#[async_trait]
impl CommandHandler<GetPatientsCommand, Vec<Patient>> for GetPatientsHandler {
    #[tracing::instrument(name = "Handle get all patients command", skip(self, _command))]
    async fn handle_command(
        &self,
        _command: GetPatientsCommand,
    ) -> Result<Vec<Patient>, anyhow::Error> {
        self.repository.get().await
    }
}
