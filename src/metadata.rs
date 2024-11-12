


#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Model {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub version: String,
    pub created_at: chrono::NaiveDateTime,
}