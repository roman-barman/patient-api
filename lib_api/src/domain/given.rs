use thiserror::Error;

const GIVEN_MAX_LENGTH: usize = 100;

#[derive(Debug, Clone)]
pub struct Given(Vec<String>);

#[derive(Error, Debug, PartialEq, Eq)]
pub enum GivenValidationError {
    #[error("given value length is greater than {0}", GIVEN_MAX_LENGTH)]
    GivenValueIsTooLong,
}

impl TryFrom<Vec<String>> for Given {
    type Error = GivenValidationError;
    fn try_from(given: Vec<String>) -> Result<Self, Self::Error> {
        for value in given.iter() {
            if value.len() > GIVEN_MAX_LENGTH {
                return Err(GivenValidationError::GivenValueIsTooLong);
            }
        }

        Ok(Given(given))
    }
}

impl From<Given> for Vec<String> {
    fn from(given: Given) -> Self {
        given.0
    }
}

impl AsRef<Vec<String>> for Given {
    fn as_ref(&self) -> &Vec<String> {
        &self.0
    }
}

#[cfg(test)]
mod given_tests {
    use crate::domain::given::{Given, GivenValidationError, GIVEN_MAX_LENGTH};

    #[test]
    fn given_new_when_given_is_long_error() {
        //Arrange
        let long_given = (0..GIVEN_MAX_LENGTH + 1).map(|_| "X").collect::<String>();
        let given = vec!["Test".to_string(), "Test".to_string(), long_given];

        //Act
        let result = Given::try_from(given);

        //Assert
        assert_eq!(
            result.unwrap_err(),
            GivenValidationError::GivenValueIsTooLong
        );
    }
}
