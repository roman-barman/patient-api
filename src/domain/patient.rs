use chrono::{DateTime, Local, Utc};
use thiserror::Error;
use uuid::Uuid;

const FAMILY_MAX_LENGTH: usize = 100;
const GIVEN_MAX_LENGTH: usize = 100;

#[derive(Debug, Clone)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone)]
pub struct Name {
    pub id: Uuid,
    pub family: String,
    pub given: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Patient {
    pub name: Name,
    pub gender: Option<Gender>,
    pub birth_date: DateTime<Utc>,
    pub active: bool,
    pub version: DateTime<Local>,
}

impl Name {
    fn new(family: String, given: Option<Vec<String>>) -> Result<Name, PatientValidationError> {
        if family.is_empty() {
            return Err(PatientValidationError::FamilyIsEmpty);
        }

        if family.len() > FAMILY_MAX_LENGTH {
            return Err(PatientValidationError::FamilyIsTooLong);
        }

        if let Some(ref values) = given {
            for value in values.iter() {
                if value.len() > GIVEN_MAX_LENGTH {
                    return Err(PatientValidationError::GivenValueIsTooLong);
                }
            }
        }

        Ok(Self {
            id: Uuid::new_v4(),
            family,
            given,
        })
    }
}

impl Patient {
    pub fn new(
        family: String,
        given: Option<Vec<String>>,
        gender: Option<Gender>,
        birth_date: DateTime<Utc>,
        active: bool,
    ) -> Result<Patient, PatientValidationError> {
        let name = Name::new(family, given)?;

        if birth_date > Local::now() {
            return Err(PatientValidationError::BirthDateGreaterThanNow);
        }

        Ok(Self {
            name,
            gender,
            birth_date,
            active,
            version: Local::now(),
        })
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PatientValidationError {
    #[error("family is empty")]
    FamilyIsEmpty,
    #[error("family length is greater than {0}", FAMILY_MAX_LENGTH)]
    FamilyIsTooLong,
    #[error("given value length is greater than {0}", GIVEN_MAX_LENGTH)]
    GivenValueIsTooLong,
    #[error("date of birth is greater than current date")]
    BirthDateGreaterThanNow,
}

#[cfg(test)]
mod patient_tests {
    use crate::domain::patient::{
        Name, Patient, PatientValidationError, FAMILY_MAX_LENGTH, GIVEN_MAX_LENGTH,
    };
    use chrono::{Days, Local};

    #[test]
    fn name_new_when_family_is_empty_error() {
        //Arrange
        let family = "".to_string();

        //Act
        let result = Name::new(family, None);

        //Assert
        assert_eq!(result.unwrap_err(), PatientValidationError::FamilyIsEmpty);
    }

    #[test]
    fn name_new_when_family_is_long_error() {
        //Arrange
        let family = (0..FAMILY_MAX_LENGTH + 1).map(|_| "X").collect::<String>();

        //Act
        let result = Name::new(family, None);

        //Assert
        assert_eq!(result.unwrap_err(), PatientValidationError::FamilyIsTooLong);
    }

    #[test]
    fn name_new_when_given_is_long_error() {
        //Arrange
        let family = "Test".to_string();
        let long_given = (0..GIVEN_MAX_LENGTH + 1).map(|_| "X").collect::<String>();
        let given = vec!["Test".to_string(), "Test".to_string(), long_given];

        //Act
        let result = Name::new(family, Some(given));

        //Assert
        assert_eq!(
            result.unwrap_err(),
            PatientValidationError::GivenValueIsTooLong
        );
    }

    #[test]
    fn patient_new_when_birth_date_is_greater_current_error() {
        //Arrange
        let family = "Test".to_string();

        //Act
        let result = Patient::new(
            family,
            None,
            None,
            Local::now()
                .checked_add_days(Days::new(1))
                .unwrap()
                .to_utc(),
            false,
        );

        //Assert
        assert_eq!(
            result.unwrap_err(),
            PatientValidationError::BirthDateGreaterThanNow
        );
    }
}
