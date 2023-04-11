use async_graphql::{dataloader::DataLoader, *};
use sqlx::{Pool, Postgres};

use crate::objects::{
    location::{InputLocation, Location, LocationLoader},
    piece::{InputPiece, Piece, PieceLoader}, location_entry::{LocationEntry, self, InputLocationEntry, LocationEntryLoader},
};

pub type IMSSchema = Schema<IMSQuery, IMSMutation, EmptySubscription>;

#[derive(Default)]
pub struct IMSQuery;

#[Object]
impl IMSQuery {
    pub async fn get_piece(&self, ctx: &Context<'_>, id: i32) -> Result<Piece> {
        let loader = ctx.data_unchecked::<DataLoader<PieceLoader>>();
        let piece = loader.load_one(id).await?;

        Ok(piece.unwrap())
    }

    pub async fn get_location(&self, ctx: &Context<'_>, id: i32) -> Result<Location> {
        let loader = ctx.data_unchecked::<DataLoader<LocationLoader>>();
        let location = loader.load_one(id).await?;

        Ok(location.unwrap())
    }

    pub async fn get_pieces(&self, ctx: &Context<'_>, ids: Vec<i32>) -> Result<Vec<Piece>> {
        let loader = ctx.data_unchecked::<DataLoader<PieceLoader>>();

        let load_pieces = loader.load_many(ids).await?;

        // Construct the pieces vector
        let mut pieces: Vec<Piece> = Vec::new();

        // Fill with loaded piece data
        for piece in load_pieces {
            pieces.push(piece.1);
        }

        Ok(pieces)
    }
    pub async fn get_locations(&self, ctx: &Context<'_>, ids: Vec<i32>) -> Result<Vec<Location>> {
        let loader = ctx.data_unchecked::<DataLoader<LocationLoader>>();

        let load_locations = loader.load_many(ids).await?;

        // Construct the locations vector
        let mut locations: Vec<Location> = Vec::new();

        // Fill with loaded location data
        for location in load_locations {
            locations.push(location.1);
        }

        Ok(locations)
    }

    pub async fn find_piece(&self, ctx: &Context<'_>, id: i32) -> Result<Vec<Location>> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let loader = ctx.data_unchecked::<DataLoader<LocationLoader>>();

        let locations = sqlx::query_as!(
            Location, 
            "SELECT * FROM ims.locations WHERE id IN 
            (SELECT location_id FROM ims.location_entries WHERE piece_id = $1)", 
            id)
            .fetch_all(pool)
            .await?;

        // Add the locations to the loader
        // Not taking credit for this if it works, fuck this shit or thanks copilot.
        loader.feed_many(locations.iter().map(
            |location| (location.id, location.clone())
        )).await;

        Ok(locations)
    }
}

#[derive(Default)]
pub struct IMSMutation;

#[Object]
impl IMSMutation {
    
    pub async fn create_piece(&self, ctx: &Context<'_>, new_piece: InputPiece) -> Result<Piece> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let loader = ctx.data_unchecked::<DataLoader<PieceLoader>>();

        // Create the new piece
        let piece = sqlx::query_as!(
            Piece,
            "INSERT INTO ims.pieces (name, description, category) VALUES ($1, $2, $3) RETURNING *",
            new_piece.name,
            new_piece.description,
            new_piece.category
        )
        .fetch_one(pool)
        .await?;

        // Add the new piece to the loader
        loader.feed_one(piece.id, piece.clone()).await;

        Ok(piece)
    }

    pub async fn create_location(
        &self,
        ctx: &Context<'_>,
        new_location: InputLocation,
    ) -> Result<Location> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let loader = ctx.data_unchecked::<DataLoader<LocationLoader>>();

        // Create the new location
        let location = sqlx::query_as!(
            Location,
            "INSERT INTO ims.locations (title, description) VALUES ($1, $2) RETURNING *",
            new_location.title,
            new_location.description
        )
        .fetch_one(pool)
        .await?;

        // Add the new location to the loader
        loader.feed_one(location.id, location.clone()).await;

        Ok(location)
    }

    pub async fn create_location_entry(
        &self,
        ctx: &Context<'_>,
        entry_input: InputLocationEntry,
    ) -> Result<LocationEntry> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let loader = ctx.data_unchecked::<DataLoader<LocationEntryLoader>>();

        // Create the new location entry
        let location_entry = sqlx::query_as!(
            LocationEntry,
            "INSERT INTO ims.location_entries (piece_id, location_id, quantity, unit) VALUES ($1, $2, $3, $4) RETURNING *",
            entry_input.piece_id,
            entry_input.location_id,
            entry_input.quantity,
            entry_input.unit
        )
        .fetch_one(pool)
        .await?;

        // Add the new location to the loader
        loader.feed_one(location_entry.id, location_entry.clone()).await;

        Ok(location_entry)
    }

    pub async fn update_piece(&self, id: i32, input: InputPiece) -> Result<Piece> {
        todo! {}
    }

    pub async fn update_location(&self, id: i32, input: InputLocation) -> Result<Location> {
        todo! {}
    }
}
