use crate::domain::{Gender, Name, Patient};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
enum GenderResponse {
    Male,
    Female,
}

#[derive(Serialize)]
struct NameResponse {
    id: Uuid,
    family: String,
    given: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct PatientResponse {
    name: NameResponse,
    gender: Option<GenderResponse>,
    birth_date: String,
    active: bool,
    version: i64,
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
