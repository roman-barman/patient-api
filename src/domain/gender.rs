#[derive(Debug, Clone, sqlx::Type, serde::Serialize, serde::Deserialize)]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
}
