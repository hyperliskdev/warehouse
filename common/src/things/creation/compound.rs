use serde::{Deserialize, Serialize};

use crate::things::thing::Thing;

use super::constituent::Constituent;

/* Compound */
#[derive(Serialize, Deserialize)]
pub(crate) struct Compound {
    id: uuid::Uuid,
    name: String,
    description: String,
    constituents: Vec<Constituent>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Compound {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            description,
            constituents: Vec::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}

impl Thing for Compound {
    fn id(&self) -> uuid::Uuid {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }
}
