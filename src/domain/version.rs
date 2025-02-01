use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct Version(DateTime<Local>);

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
