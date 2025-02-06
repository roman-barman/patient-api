use crate::application::{CommandHandler, Repository};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct DeletePatientHandler {
    repository: Arc<dyn Repository>,
}

impl DeletePatientHandler {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        Self { repository }
    }
}

#[derive(Debug)]
pub struct DeletePatientCommand {
    id: Uuid,
}

impl DeletePatientCommand {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

#[async_trait]
impl CommandHandler<DeletePatientCommand, bool> for DeletePatientHandler {
    #[tracing::instrument(name = "Handle delete patient command", skip(self))]
    async fn handle_command(&self, command: DeletePatientCommand) -> Result<bool, anyhow::Error> {
        let result = self.repository.delete(&command.id).await?;
        Ok(result)
    }
}
