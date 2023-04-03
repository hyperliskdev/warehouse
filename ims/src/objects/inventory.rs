// Async-GraphQL Inventory

use async_graphql::*;

use super::{location::Location, piece::Piece};



#[derive(Clone, Debug, Default)]
pub struct Inventory {
    pub id: i32,
    pub name: String,
    pub locations: Vec<Location>,
    pub pieces: Vec<Piece>,
    // pub entries: Vec<LocationEntry>,
}

#[Object]
impl Inventory {
    
    async fn id(&self) -> i32 {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn locations(&self) -> &Vec<Location> {
        &self.locations
    }

    async fn pieces(&self) -> &Vec<Piece> {
        &self.pieces
    }

    // async fn entries(&self) -> &Vec<LocationEntry> {
    //     &self.entries
    // }


}