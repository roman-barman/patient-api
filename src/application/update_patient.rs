use crate::application::{CommandHandler, Repository};
use crate::domain::{
    BirthDate, BirthDateValidationError, Family, FamilyValidationError, Gender, Given,
    GivenValidationError, Name, NameId, Patient, Version, VersionValidationError,
};
use async_trait::async_trait;
use std::sync::Arc;
use thiserror::Error;
use uuid::Uuid;

pub struct UpdatePatientHandler {
    repository: Arc<dyn Repository>,
}

impl UpdatePatientHandler {
    pub fn new(repository: Arc<dyn Repository>) -> Self {
        Self { repository }
    }
}

#[derive(Debug)]
pub struct UpdatePatientCommand {
    id: Uuid,
    family: String,
    given: Option<Vec<String>>,
    gender: Option<Gender>,
    birth_date: String,
    active: bool,
    version: i64,
}

impl UpdatePatientCommand {
    pub fn new(
        id: Uuid,
        family: String,
        given: Option<Vec<String>>,
        gender: Option<Gender>,
        birth_date: String,
        active: bool,
        version: i64,
    ) -> Self {
        Self {
            id,
            family,
            given,
            gender,
            birth_date,
            active,
            version,
        }
    }
}

#[async_trait]
impl CommandHandler<UpdatePatientCommand, ()> for UpdatePatientHandler {
    #[tracing::instrument(name = "Handle update patient command", skip(self))]
    async fn handle_command(&self, command: UpdatePatientCommand) -> Result<(), anyhow::Error> {
        let patient: Patient = command
            .try_into()
            .map_err(UpdatePatientError::InvalidArguments)?;
        let is_updated = self.repository.update(&patient).await?;

        if is_updated {
            return Ok(());
        }

        let is_exist = self.repository.exist(patient.name.id.as_ref()).await?;

        if is_exist {
            Err(anyhow::Error::new(
                UpdatePatientError::PatientVersionWasChanged,
            ))
        } else {
            Err(anyhow::Error::new(UpdatePatientError::PatientDoesNotExist))
        }
    }
}

impl TryFrom<UpdatePatientCommand> for Patient {
    type Error = UpdatePatientValidationError;
    fn try_from(value: UpdatePatientCommand) -> Result<Self, Self::Error> {
        let name_id = NameId::new(value.id);
        let family = Family::try_from(value.family)?;
        let given = value.given.map(Given::try_from).transpose()?;
        let name = Name::new_with_id(name_id, family, given);

        let birth_date = BirthDate::try_from(value.birth_date)?;
        let version = Version::try_from(value.version)?;
        Ok(Patient::new_with_version(
            name,
            value.gender,
            birth_date,
            value.active,
            version,
        ))
    }
}

#[derive(Error, Debug)]
pub enum UpdatePatientError {
    #[error(transparent)]
    InvalidArguments(#[from] UpdatePatientValidationError),
    #[error("patient does not exists")]
    PatientDoesNotExist,
    #[error("patient version was changed")]
    PatientVersionWasChanged,
}

#[derive(Error, Debug)]
pub enum UpdatePatientValidationError {
    #[error(transparent)]
    Family(#[from] FamilyValidationError),
    #[error(transparent)]
    Given(#[from] GivenValidationError),
    #[error(transparent)]
    BirthDay(#[from] BirthDateValidationError),
    #[error(transparent)]
    Version(#[from] VersionValidationError),
}
