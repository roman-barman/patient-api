use chrono::{DateTime, Local, TimeZone};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Version(DateTime<Local>);

impl Version {
    pub fn new(date: DateTime<Local>) -> Self {
        Self(date)
    }
}

impl Default for Version {
    fn default() -> Self {
        Self(Local::now())
    }
}

impl From<Version> for DateTime<Local> {
    fn from(version: Version) -> Self {
        version.0
    }
}

impl From<Version> for i64 {
    fn from(version: Version) -> Self {
        version.0.timestamp_millis()
    }
}

impl TryFrom<i64> for Version {
    type Error = VersionValidationError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let result = Local.timestamp_millis_opt(value).single();
        match result {
            Some(value) => Ok(Self(value)),
            None => Err(VersionValidationError::InvalidVersion),
        }
    }
}

impl AsRef<DateTime<Local>> for Version {
    fn as_ref(&self) -> &DateTime<Local> {
        &self.0
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum VersionValidationError {
    #[error("invalid version")]
    InvalidVersion,
}
