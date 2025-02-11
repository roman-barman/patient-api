use crate::domain::{BirthDate, Family, Gender, Given, Name, NameId, Patient, Version};
use chrono::{DateTime, Local, Utc};
use uuid::Uuid;

pub struct PatientDbModel {
    pub id: Uuid,
    pub family: String,
    pub given: Option<Vec<String>>,
    pub gender: Option<Gender>,
    pub birth_date: DateTime<Utc>,
    pub active: bool,
    pub version: DateTime<Local>,
}

impl TryFrom<PatientDbModel> for Patient {
    type Error = anyhow::Error;

    fn try_from(model: PatientDbModel) -> Result<Self, Self::Error> {
        let name_id = NameId::new(model.id);
        let family = Family::try_from(model.family)?;
        let given = model.given.map(Given::try_from).transpose()?;
        let name = Name::new_with_id(name_id, family, given);

        let gender = model.gender;
        let birth_date = BirthDate::new(model.birth_date);
        let active = model.active;
        let version = Version::new(model.version);

        Ok(Patient::new_with_version(
            name, gender, birth_date, active, version,
        ))
    }
}
