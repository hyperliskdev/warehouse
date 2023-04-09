use async_graphql::MergedObject;

use self::{location::Location, piece::Piece, piece_category::PieceCategory, location_entry::LocationEntry};

pub mod location;
pub mod piece;
pub mod piece_category;
pub mod location_entry;