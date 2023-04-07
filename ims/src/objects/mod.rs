use async_graphql::MergedObject;

use self::{location::Location, piece::Piece, piece_category::PieceCategory};

pub mod location;
pub mod piece;
pub mod piece_category;

#[derive(MergedObject)]
pub struct InventoryQuery(
    Location, 
    Piece,
    PieceCategory
);
