use crate::application::{CommandHandler, CreatePatientCommand};
use crate::domain::{Gender, Patient, PatientValidationError};
use actix_web::{post, web, HttpResponse};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use std::sync::Arc;

#[derive(serde::Deserialize)]
pub enum RequestGender {
    Male,
    Female,
}

#[derive(serde::Deserialize)]
pub struct RequestName {
    family: String,
    given: Option<Vec<String>>,
}

#[derive(serde::Deserialize)]
pub struct RequestPatient {
    name: RequestName,
    gender: Option<RequestGender>,
    birth_date: String,
    active: bool,
}

#[post("/patients")]
pub async fn create_patient(
    handler: web::Data<Arc<dyn CommandHandler<CreatePatientCommand, Patient>>>,
    request: web::Json<RequestPatient>,
) -> HttpResponse {
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
                None => return HttpResponse::BadRequest().finish(),
            }
        }
        Err(_) => return HttpResponse::BadRequest().finish(),
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
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            if e.root_cause()
                .downcast_ref::<PatientValidationError>()
                .is_some()
            {
                HttpResponse::BadRequest().finish()
            } else {
                HttpResponse::InternalServerError().finish()
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
