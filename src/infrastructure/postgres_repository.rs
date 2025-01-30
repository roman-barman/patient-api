use crate::application::Repository;
use crate::domain::{Gender, Patient};
use async_trait::async_trait;
use chrono::{DateTime, Local, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Repository for PostgresRepository {
    #[tracing::instrument(name = "Add patient to the DB", skip(self))]
    async fn create(&self, patient: &Patient) -> Result<(), anyhow::Error> {
        let db_model = PatientDbModel::from(patient.clone());
        sqlx::query_unchecked!(
            r#"
                INSERT INTO patients
                VALUES
                ($1, $2, $3, $4, $5, $6, $7)
            "#,
            db_model.id,
            db_model.family,
            db_model.given,
            db_model.gender,
            db_model.birth_date,
            db_model.active,
            db_model.version,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
enum GenderDbModel {
    Male,
    Female,
}

struct PatientDbModel {
    id: Uuid,
    family: String,
    given: Option<Vec<String>>,
    gender: Option<GenderDbModel>,
    birth_date: DateTime<Utc>,
    active: bool,
    version: DateTime<Local>,
}

impl From<Patient> for PatientDbModel {
    fn from(patient: Patient) -> Self {
        Self {
            id: patient.name.id,
            family: patient.name.family,
            given: patient.name.given,
            gender: patient.gender.map(|gender| gender.into()),
            birth_date: patient.birth_date,
            active: patient.active,
            version: patient.version,
        }
    }
}

impl From<Gender> for GenderDbModel {
    fn from(value: Gender) -> Self {
        match value {
            Gender::Male => GenderDbModel::Male,
            Gender::Female => GenderDbModel::Female,
        }
    }
}
