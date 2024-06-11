use serde::{Deserialize, Serialize};

use crate::things::{thing::Thing, unit::Unit};

use super::compound::Compound;

/*  */
#[derive(Serialize, Deserialize)]
pub(crate) struct Constituent {
    id: uuid::Uuid,
    name: String,
    description: String,
    constituent_in: Vec<Compound>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}


impl Constituent {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            description,
            constituent_in: Vec::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
}

impl Thing for Constituent {
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