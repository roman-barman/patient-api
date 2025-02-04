use crate::application::{CommandHandler, Repository};
use crate::domain::Patient;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct GetPatientByIdHandler {
    repository: Arc<dyn Repository>,
}

impl GetPatientByIdHandler {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        Self { repository }
    }
}

#[derive(Debug)]
pub struct GetPatientByIdCommand {
    id: Uuid,
}

impl GetPatientByIdCommand {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

#[async_trait]
impl CommandHandler<GetPatientByIdCommand, Option<Patient>> for GetPatientByIdHandler {
    #[tracing::instrument(name = "Handle get patient by ID command", skip(self))]
    async fn handle_command(
        &self,
        command: GetPatientByIdCommand,
    ) -> Result<Option<Patient>, anyhow::Error> {
        let patient = self.repository.get_by_id(&command.id).await?;
        Ok(patient)
    }
}
