use std::io::Error;

use crate::things::creation::{compound::Compound, constituent::Constituent};


pub(crate) trait Action {
    fn id(&self) -> uuid::Uuid;
    fn title(&self) -> &str;
    fn description(&self) -> &str;

    // Any number of constituents can be involved in an action
    fn act(&self, constituents: Vec<Constituent>) -> Result<Compound, Error>;    

    
    fn created_at(&self) -> chrono::DateTime<chrono::Utc>;
    fn updated_at(&self) -> chrono::DateTime<chrono::Utc>;
}