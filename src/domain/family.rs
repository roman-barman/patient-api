use thiserror::Error;

const FAMILY_MAX_LENGTH: usize = 100;

#[derive(Debug, Clone)]
pub struct Family(String);

impl TryFrom<String> for Family {
    type Error = FamilyValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(FamilyValidationError::FamilyIsEmpty);
        }

        if value.len() > FAMILY_MAX_LENGTH {
            return Err(FamilyValidationError::FamilyIsTooLong);
        }

        Ok(Self(value))
    }
}

impl From<Family> for String {
    fn from(family: Family) -> Self {
        family.0
    }
}

impl AsRef<str> for Family {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum FamilyValidationError {
    #[error("family is empty")]
    FamilyIsEmpty,
    #[error("family length is greater than {0}", FAMILY_MAX_LENGTH)]
    FamilyIsTooLong,
}

#[cfg(test)]
mod family_tests {
    use crate::domain::family::{Family, FamilyValidationError, FAMILY_MAX_LENGTH};

    #[test]
    fn family_new_when_family_is_empty_error() {
        //Arrange
        let family = "".to_string();

        //Act
        let result = Family::try_from(family);

        //Assert
        assert_eq!(result.unwrap_err(), FamilyValidationError::FamilyIsEmpty);
    }

    #[test]
    fn family_new_when_family_is_long_error() {
        //Arrange
        let family = (0..FAMILY_MAX_LENGTH + 1).map(|_| "X").collect::<String>();

        //Act
        let result = Family::try_from(family);

        //Assert
        assert_eq!(result.unwrap_err(), FamilyValidationError::FamilyIsTooLong);
    }
}
