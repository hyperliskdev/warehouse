// Employee

#[derive(Clone, Debug, Default, sqlx::FromRow)]
pub struct Employee {
    pub id: i32,
    pub employee_code: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub password: String,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

