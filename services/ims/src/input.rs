use async_graphql::InputObject;




#[derive(InputObject)]
pub struct OrderInput {
    customer_id: i32,
    products: Vec<ProductEntryInput>,
    total: f64,
}

#[derive(InputObject)]
pub struct ProductEntryInput {
    product_id: i32,
    quantity: i32,
}