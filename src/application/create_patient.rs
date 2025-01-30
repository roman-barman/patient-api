use crate::application::{CommandHandler, Repository};
use crate::domain::{Gender, Patient};
use async_trait::async_trait;
use chrono::{DateTime, Utc};

pub struct CreatePatientHandler {
    repository: Box<dyn Repository>,
}

impl CreatePatientHandler {
    pub fn new(repository: Box<dyn Repository>) -> Self {
        Self { repository }
    }
}

#[derive(Debug)]
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
    #[tracing::instrument(name = "Handle create patient command", skip(self))]
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
