use crate::api::api_error::ApiError;
use crate::api::routes::patients::patient_response::PatientResponse;
use crate::application::{CommandHandler, CreatePatientCommand};
use crate::domain::{Gender, Patient, PatientValidationError};
use actix_web::{post, web, HttpResponse};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

#[derive(serde::Deserialize, Debug)]
pub enum RequestGender {
    Male,
    Female,
}

#[derive(serde::Deserialize, Debug)]
pub struct RequestName {
    family: String,
    given: Option<Vec<String>>,
}

#[derive(serde::Deserialize, Debug)]
pub struct RequestPatient {
    name: RequestName,
    gender: Option<RequestGender>,
    birth_date: String,
    active: bool,
}

#[post("/patients")]
#[tracing::instrument(name = "Adding a new patient", skip(handler))]
pub async fn create_patient(
    handler: web::Data<Box<dyn CommandHandler<CreatePatientCommand, Patient>>>,
    request: web::Json<RequestPatient>,
) -> Result<HttpResponse, ApiError> {
    let patient = request.into_inner();
    let gender: Option<Gender> = patient.gender.map(|request_gender| request_gender.into());
    let birth_date = NaiveDate::parse_from_str(&patient.birth_date, "%Y-%m-%d");
    let birth_date = match birth_date {
        Ok(birth_date) => {
            let time = NaiveTime::from_hms_opt(0, 0, 0);

            match time {
                Some(time) => {
                    let datetime = NaiveDateTime::new(birth_date, time);
                    Utc.from_utc_datetime(&datetime)
                }
                None => return Err(ApiError::InternalServerError),
            }
        }
        Err(_) => return Err(ApiError::BadRequest("invalid birth date format".into())),
    };
    let command = CreatePatientCommand::new(
        patient.name.family,
        patient.name.given,
        gender,
        birth_date,
        patient.active,
    );
    let result = handler.handle_command(command).await;

    match result {
        Ok(patient) => Ok(HttpResponse::Created()
            .append_header(("Location", format!("/patients/{}", patient.name.id)))
            .json(PatientResponse::from(patient))),
        Err(e) => {
            if let Some(validation_error) = e.root_cause().downcast_ref::<PatientValidationError>()
            {
                Err(ApiError::BadRequest(validation_error.to_string()))
            } else {
                Err(ApiError::InternalServerError)
            }
        }
    }
}

impl From<RequestGender> for Gender {
    fn from(request_gender: RequestGender) -> Self {
        match request_gender {
            RequestGender::Male => Gender::Male,
            RequestGender::Female => Gender::Female,
        }
    }
}
