use crate::application::{CommandHandler, Repository};
use crate::domain::{Gender, Patient};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;

pub struct CreatePatientHandler {
    repository: Arc<dyn Repository>,
}

impl CreatePatientHandler {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        Self { repository }
    }
}

pub struct CreatePatientCommand {
    family: String,
    given: Option<Vec<String>>,
    gender: Option<Gender>,
    birth_date: DateTime<Utc>,
    active: bool,
}

impl CreatePatientCommand {
    pub fn new(
        family: String,
        given: Option<Vec<String>>,
        gender: Option<Gender>,
        birth_date: DateTime<Utc>,
        active: bool,
    ) -> Self {
        Self {
            family,
            given,
            gender,
            birth_date,
            active,
        }
    }
}

#[async_trait]
impl CommandHandler<CreatePatientCommand, Patient> for CreatePatientHandler {
    async fn handle_command(
        &self,
        command: CreatePatientCommand,
    ) -> Result<Patient, anyhow::Error> {
        let patient = Patient::new(
            command.family,
            command.given,
            command.gender,
            command.birth_date,
            command.active,
        )?;

        self.repository.create(&patient).await?;

        Ok(patient)
    }
}
