use crate::application::{CommandHandler, Repository};
use crate::domain::{
    BirthDate, BirthDateValidationError, Family, FamilyValidationError, Gender, Given,
    GivenValidationError, Name, Patient,
};
use async_trait::async_trait;
use std::sync::Arc;
use thiserror::Error;

pub struct CreatePatientHandler {
    repository: Arc<dyn Repository>,
}

impl CreatePatientHandler {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        Self { repository }
    }
}

#[derive(Debug)]
pub struct CreatePatientCommand {
    family: String,
    given: Option<Vec<String>>,
    gender: Option<Gender>,
    birth_date: String,
    active: bool,
}

impl CreatePatientCommand {
    pub fn new(
        family: String,
        given: Option<Vec<String>>,
        gender: Option<Gender>,
        birth_date: String,
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
        let patient: Patient = command.try_into()?;

        self.repository.create(&patient).await?;

        Ok(patient)
    }
}

impl TryFrom<CreatePatientCommand> for Patient {
    type Error = CreatePatientValidationError;
    fn try_from(value: CreatePatientCommand) -> Result<Self, Self::Error> {
        let family = Family::try_from(value.family)?;
        let given = value.given.map(Given::try_from).transpose()?;
        let name = Name::new(family, given);

        let birth_date = BirthDate::try_from(value.birth_date)?;
        Ok(Patient::new(name, value.gender, birth_date, value.active))
    }
}

#[derive(Error, Debug)]
pub enum CreatePatientValidationError {
    #[error(transparent)]
    Family(#[from] FamilyValidationError),
    #[error(transparent)]
    Given(#[from] GivenValidationError),
    #[error(transparent)]
    BirthDay(#[from] BirthDateValidationError),
}
