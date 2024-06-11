// A unit is the denomination of a specific amount of a product.

pub(crate) struct Unit {
    id: uuid::Uuid,
    title: String,
    short_form: String,
    description: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Unit {
    pub fn new(title: String, short_form: String, description: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            title,
            short_form,
            description,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn short_form(&self) -> &str {
        &self.short_form
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }
    
}