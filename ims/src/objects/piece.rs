// Piece's go inside Locations

use async_graphql::Object;
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct Piece {
    id: i32,
    code: Uuid,
    name: String,
    category: i32,
}


#[Object]
impl Piece {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn code(&self) -> Uuid {
        self.code
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn category(&self) -> i32 {
        self.category
    }
}

// PieceLoader