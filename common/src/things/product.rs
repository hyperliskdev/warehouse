// A product can be sold or bought. After manufacturing, a result can be converted into a product.
use crate::things::thing::Thing;
use crate::things::unit::Unit;


struct Product {
    id: uuid::Uuid,
    name: String,
    description: String,

    // Specific to Product
    amount: f64,
    price: f64,
    unit: Unit,

    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Product {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            description,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            amount: todo!(),
            price: todo!(),
            unit: todo!(),
            
        }
    }

}

impl Thing for Product {
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