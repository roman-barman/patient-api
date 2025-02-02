use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use thiserror::Error;

const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Debug, Clone)]
pub struct BirthDate(DateTime<Utc>);

#[derive(Error, Debug, PartialEq, Eq)]
pub enum BirthDateValidationError {
    #[error("date of birth is greater than current date")]
    BirthDateGreaterThanNow,
    #[error("invalid birth date format")]
    InvalidBirthDateFormat,
}

impl TryFrom<String> for BirthDate {
    type Error = BirthDateValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let birth_date = NaiveDate::parse_from_str(&value, DATE_FORMAT);
        let birth_date = match birth_date {
            Ok(birth_date) => {
                let time = NaiveTime::from_hms_opt(0, 0, 0);
                match time {
                    Some(time) => {
                        let datetime = NaiveDateTime::new(birth_date, time);
                        Utc.from_utc_datetime(&datetime)
                    }
                    None => {
                        let datetime = NaiveDateTime::new(birth_date, NaiveTime::default());
                        Utc.from_utc_datetime(&datetime)
                    }
                }
            }
            Err(_) => return Err(BirthDateValidationError::InvalidBirthDateFormat),
        };

        if birth_date > Local::now() {
            return Err(BirthDateValidationError::BirthDateGreaterThanNow);
        }

        Ok(BirthDate(birth_date))
    }
}

impl From<BirthDate> for DateTime<Utc> {
    fn from(birth_date: BirthDate) -> Self {
        birth_date.0
    }
}

impl AsRef<DateTime<Utc>> for BirthDate {
    fn as_ref(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl From<BirthDate> for String {
    fn from(value: BirthDate) -> Self {
        format!("{}", value.0.format(DATE_FORMAT))
    }
}

#[cfg(test)]
mod birth_day_tests {
    use crate::domain::birth_date::{BirthDate, BirthDateValidationError};

    #[test]
    fn birth_date_new_when_birth_date_is_greater_current_error() {
        //Act
        let result = BirthDate::try_from(String::from("2100-01-01"));

        //Assert
        assert_eq!(
            result.unwrap_err(),
            BirthDateValidationError::BirthDateGreaterThanNow
        );
    }

    #[test]
    fn birth_date_new_when_birth_date_invalid_error() {
        //Arrange
        let values = vec!["2000-01", "2000", "2000/01/01", "2000-01-01 12:00:00"];

        //Act
        for value in values {
            let result = BirthDate::try_from(String::from(value));

            //Assert
            assert_eq!(
                result.unwrap_err(),
                BirthDateValidationError::InvalidBirthDateFormat
            );
        }
    }
}
