use async_graphql::Enum;
use serde::Serialize;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, sqlx::Type)]
pub enum OrderStatus {
    InProgress,
    AwaitingPayment,
    PaymentRecieved,
    Complete,
}

