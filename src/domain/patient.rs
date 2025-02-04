use crate::domain::birth_date::BirthDate;
use crate::domain::family::Family;
use crate::domain::gender::Gender;
use crate::domain::given::Given;
use crate::domain::name_id::NameId;
use crate::domain::version::Version;

#[derive(Debug, Clone)]
pub struct Name {
    pub id: NameId,
    pub family: Family,
    pub given: Option<Given>,
}

#[derive(Debug, Clone)]
pub struct Patient {
    pub name: Name,
    pub gender: Option<Gender>,
    pub birth_date: BirthDate,
    pub active: bool,
    pub version: Version,
}

impl Name {
    pub fn new(family: Family, given: Option<Given>) -> Self {
        Self {
            id: NameId::default(),
            family,
            given,
        }
    }

    pub fn new_with_id(id: NameId, family: Family, given: Option<Given>) -> Self {
        Self { id, family, given }
    }
}

impl Patient {
    pub fn new(name: Name, gender: Option<Gender>, birth_date: BirthDate, active: bool) -> Self {
        Self {
            name,
            gender,
            birth_date,
            active,
            version: Version::default(),
        }
    }

    pub fn new_with_version(
        name: Name,
        gender: Option<Gender>,
        birth_date: BirthDate,
        active: bool,
        version: Version,
    ) -> Self {
        Self {
            name,
            gender,
            birth_date,
            active,
            version,
        }
    }
}
