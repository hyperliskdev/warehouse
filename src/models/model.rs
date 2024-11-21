use serde::{Deserialize, Serialize};




#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    pub id: String,
    pub title: String,
    pub description: String,
    pub version: String,
    pub s3_key: String,
    pub created_at: String,
    pub updated_at: String,
}