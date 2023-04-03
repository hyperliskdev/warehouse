
use async_graphql::*;

use crate::objects::{piece::Piece, location::Location};


pub type InventorySchema = Schema<IMSQuery, IMSMutation, EmptySubscription>;

pub struct IMSQuery;




#[Object]
impl IMSQuery {
    pub async fn get_piece(&self, ctx: &Context<'_>, id: i32) -> Result<Piece> {
        // Get a piece from the database by id
        todo!{}
    }

    pub async fn get_location(&self, ctx: &Context<'_>, id: i32) -> Result<Location> {
        // Get a location from the database by id
        todo!{}
    }

    pub async fn get_pieces(&self, ctx: &Context<'_>) -> Result<Vec<Piece>> {
        // Get all pieces from the database
        todo!{}
    }

    pub async fn get_locations(&self, ctx: &Context<'_>) -> Result<Vec<Location>> {
        // Get all locations from the database
        todo!{}
    }

    pub async fn find_piece(&self, ctx: &Context<'_>, name: String) -> Result<Vec<Location>> {
        // Find a piece by name
        todo!{}
    }

}


pub struct IMSMutation;

#[Object]
impl IMSMutation {
    
    pub async fn create_piece(&self, name: String) -> Result<Piece> {
        // Create a new piece in the database
        todo!{}
    }

    pub async fn create_location(&self, name: String) -> Result<Location> {
        // Create a new location in the database
        todo!{}
    }

    pub async fn update_piece(&self, id: i32, name: String) -> Result<Piece> {
        
        todo!{}
    }

    pub async fn update_location(&self, id: i32, name: String) -> Result<Location> {
        todo!{}
    }



}