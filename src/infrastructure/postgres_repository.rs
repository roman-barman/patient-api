use crate::application::Repository;
use crate::domain::Gender;
use crate::domain::Patient;
use crate::infrastructure::patient_db_model::PatientDbModel;
use async_trait::async_trait;
use chrono::DateTime;
use chrono::Local;
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
        sqlx::query_unchecked!(
            r#"
                INSERT INTO patients
                VALUES
                ($1, $2, $3, $4, $5, $6, $7)
            "#,
            patient.name.id.as_ref(),
            patient.name.family.as_ref(),
            patient.name.given.as_ref().map(|x| x.as_ref()),
            patient.gender.as_ref(),
            patient.birth_date.as_ref(),
            patient.active,
            patient.version.as_ref(),
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[tracing::instrument(name = "Get patient by ID from the DB", skip(self))]
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Patient>, anyhow::Error> {
        let patient = sqlx::query_as!(
            PatientDbModel,
            r#"
                SELECT
                    id,
                    family,
                    given,
                    gender as "gender: Gender",
                    birth_date,
                    active,
                    version as "version: DateTime<Local>"
                FROM patients
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match patient {
            Some(r) => {
                let patient = Patient::try_from(r)?;
                Ok(Some(patient))
            }
            None => Ok(None),
        }
    }
}
