use crate::domain::Patient;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn create(&self, patient: &Patient) -> Result<(), anyhow::Error>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Patient>, anyhow::Error>;
    async fn exist(&self, id: &Uuid) -> Result<bool, anyhow::Error>;
    async fn update(&self, patient: &Patient) -> Result<bool, anyhow::Error>;
    async fn delete(&self, id: &Uuid) -> Result<bool, anyhow::Error>;
    async fn get(&self) -> Result<Vec<Patient>, anyhow::Error>;
}

#[async_trait]
pub trait CommandHandler<TCommand, TResult>: Send + Sync {
    async fn handle_command(&self, command: TCommand) -> Result<TResult, anyhow::Error>;
}
