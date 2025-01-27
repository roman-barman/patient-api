use crate::domain::Patient;
use async_trait::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create(&self, patient: &Patient) -> Result<(), anyhow::Error>;
}

#[async_trait]
pub trait CommandHandler<TCommand, TResult>: Send + Sync {
    async fn handle_command(&self, command: TCommand) -> Result<TResult, anyhow::Error>;
}
