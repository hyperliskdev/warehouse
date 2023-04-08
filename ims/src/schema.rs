use async_graphql::*;
use sqlx::{pool, Pool, Postgres};
use uuid::Uuid;

use crate::objects::{
    location::{self, InputLocation, Location},
    piece::{InputPiece, Piece},
};

pub type IMSSchema = Schema<IMSQuery, IMSMutation, EmptySubscription>;

#[derive(Default)]
pub struct IMSQuery;

#[Object]
impl IMSQuery {
    pub async fn get_piece(&self, ctx: &Context<'_>, id: i32) -> Result<Piece> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let piece = sqlx::query_as!(Piece, "SELECT * FROM ims.pieces WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(piece)
    }

    pub async fn get_location(&self, ctx: &Context<'_>, id: i32) -> Result<Location> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let location = sqlx::query_as!(Location, "SELECT * FROM ims.locations WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(location)
    }

    pub async fn get_pieces(&self, ctx: &Context<'_>) -> Result<Vec<Piece>> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let pieces = sqlx::query_as!(Piece, "SELECT * FROM ims.pieces")
            .fetch_all(pool)
            .await?;

        Ok(pieces)
    }

    pub async fn get_locations(&self, ctx: &Context<'_>) -> Result<Vec<Location>> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let locations = sqlx::query_as!(Location, "SELECT * FROM ims.locations")
            .fetch_all(pool)
            .await?;

        Ok(locations)
    }

    pub async fn find_piece(&self, ctx: &Context<'_>, id: i32) -> Result<Vec<Location>> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        // Find all the locations that have a related location entry in the database based on the piece id

        let locations = sqlx::query_as!(Location, "SELECT * FROM ims.locations 
                                                        WHERE id IN 
                                                        (SELECT location_id FROM ims.location_entries 
                                                            WHERE piece_id = $1
                                                        )", 
                                                        id)
            .fetch_all(pool)
            .await?;

        Ok(locations)
    }
}

#[derive(Default)]
pub struct IMSMutation;

#[Object]
impl IMSMutation {
    pub async fn create_piece(&self, ctx: &Context<'_>, new_piece: InputPiece) -> Result<Piece> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let piece = sqlx::query_as!(
            Piece,
            "INSERT INTO ims.pieces (name, description, category) VALUES ($1, $2, $3)  RETURNING *",
            new_piece.name,
            new_piece.description,
            new_piece.category
        ).fetch_one(pool)
        .await?;

        Ok(piece)
    }

    pub async fn create_location(
        &self,
        ctx: &Context<'_>,
        new_location: InputLocation,
    ) -> Result<Location> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let location = sqlx::query_as!(
            Location,
            "INSERT INTO ims.locations (title, description) VALUES ($1, $2) RETURNING *",
            new_location.title,
            new_location.description
        )
        .fetch_one(pool)
        .await?;

        Ok(location)
    }

    pub async fn update_piece(&self, id: i32, name: String) -> Result<Piece> {
        todo! {}
    }

    pub async fn update_location(&self, id: i32, name: String) -> Result<Location> {
        todo! {}
    }
}
