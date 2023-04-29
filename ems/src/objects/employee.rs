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

#[Object]
impl Employee {
    async fn id(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let employee_id = sqlx::query!(
            r#"
                SELECT id FROM employee WHERE id = $1
            "#,
            self.id
        ).fetch_one(pool)
        .await?;
    }

    async fn employee_code(&self, ctx: &Context<'_>) -> Result<Uuid, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let employee_code = sqlx::query!(
            r#"
                SELECT employee_code FROM employee WHERE id = $1
            "#,
            self.id
        ).fetch_one(pool)
        .await?;
    }

    async fn first_name(&self, ctx: &Context<'_>) -> Result<String, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let first_name = sqlx::query!(
            r#"
                SELECT first_name FROM employee WHERE id = $1
            "#,
            self.id
        ).fetch_one(pool)
        .await?;
    }

    async fn last_name(&self, ctx: &Context<'_>) -> Result<String, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let last_name = sqlx::query!(
            r#"
                SELECT last_name FROM employee WHERE id = $1
            "#,
            self.id
        ).fetch_one(pool)
        .await?;
    }

    async fn password(&self, ctx: &Context<'_>) -> Result<String, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let password = sqlx::query!(
            r#"
                SELECT password FROM employee WHERE id = $1
            "#,
            self.id
        ).fetch_one(pool)
        .await?;
    }

    async fn created_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let created_at = sqlx::query!(
            r#"
                SELECT created_at FROM employee WHERE id = $1
            "#,
            self.id
        ).fetch_one(pool)
        .await?;
    }

    async fn updated_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let updated_at = sqlx::query!(
            r#"
                SELECT updated_at FROM employee WHERE id = $1
            "#,
            self.id
        ).fetch_one(pool)
        .await?;
    }
}