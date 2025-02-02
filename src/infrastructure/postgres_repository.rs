use crate::application::Repository;
use crate::domain::Patient;
use async_trait::async_trait;
use sqlx::PgPool;

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
}
