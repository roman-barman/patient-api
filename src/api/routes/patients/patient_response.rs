use crate::domain::{Gender, Name, Patient};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct NameResponse {
    pub id: Uuid,
    pub family: String,
    pub given: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct PatientResponse {
    pub name: NameResponse,
    pub gender: Option<Gender>,
    pub birth_date: String,
    pub active: bool,
    pub version: i64,
}

impl From<Patient> for PatientResponse {
    fn from(patient: Patient) -> Self {
        Self {
            name: patient.name.into(),
            gender: patient.gender,
            active: patient.active,
            birth_date: patient.birth_date.into(),
            version: patient.version.into(),
        }
    }
}

impl From<Name> for NameResponse {
    fn from(name: Name) -> Self {
        Self {
            id: name.id.into(),
            family: name.family.into(),
            given: name.given.map(|g| g.into()),
        }
    }
}
