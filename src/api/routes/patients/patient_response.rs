use crate::domain::{Gender, Name, Patient};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum GenderResponse {
    Male,
    Female,
}

#[derive(Serialize, Deserialize)]
pub struct NameResponse {
    pub id: Uuid,
    pub family: String,
    pub given: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct PatientResponse {
    pub name: NameResponse,
    pub gender: Option<GenderResponse>,
    pub birth_date: String,
    pub active: bool,
    pub version: i64,
}

impl From<Patient> for PatientResponse {
    fn from(patient: Patient) -> Self {
        Self {
            name: patient.name.into(),
            gender: patient.gender.map(|gender| gender.into()),
            active: patient.active,
            birth_date: format!("{}", patient.birth_date.format("%Y-%m-%d")),
            version: patient.version.timestamp_millis(),
        }
    }
}

impl From<Name> for NameResponse {
    fn from(name: Name) -> Self {
        Self {
            id: name.id,
            family: name.family,
            given: name.given,
        }
    }
}

impl From<Gender> for GenderResponse {
    fn from(gender: Gender) -> Self {
        match gender {
            Gender::Male => GenderResponse::Male,
            Gender::Female => GenderResponse::Female,
        }
    }
}
