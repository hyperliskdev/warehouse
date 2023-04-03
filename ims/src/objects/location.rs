// Locations contain Pieces

use async_graphql::Object;

#[derive(Clone, Debug, Default)]
pub struct Location {
    id: i32,
    name: String,
}

#[Object]
impl Location {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }
}

// LocationLoader
