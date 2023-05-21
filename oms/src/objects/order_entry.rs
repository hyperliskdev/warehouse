


#[derive(Clone, Debug, Default, sqlx::FromRow)]
pub struct OrderEntry {
    pub id: i32,
    pub order_id: i32,
    pub piece_id: i32,
    pub quantity: i32,
    pub unit: i32,
    pub line_price: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}