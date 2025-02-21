use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct NameId(Uuid);

impl NameId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }
}

impl Default for NameId {
    fn default() -> Self {
        NameId(Uuid::new_v4())
    }
}

impl From<Uuid> for NameId {
    fn from(id: Uuid) -> Self {
        NameId(id)
    }
}

impl From<NameId> for Uuid {
    fn from(id: NameId) -> Self {
        id.0
    }
}

impl Display for NameId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<Uuid> for NameId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}
