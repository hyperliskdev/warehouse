pub(crate) trait Thing {
    fn id(&self) -> uuid::Uuid;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn created_at(&self) -> chrono::DateTime<chrono::Utc>;
    fn updated_at(&self) -> chrono::DateTime<chrono::Utc>;
}

