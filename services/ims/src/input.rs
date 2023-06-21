


#[derive(InputObject, Serialize, Deserialize)]
struct OrderInput {
    customer_id: i32,
    products: Vec<ProductEntryInput>,
    total: f64,
}

#[derive(InputObject, Serialize, Deserialize)]
struct ProductEntryInput {
    product_id: i32,
    quantity: i32,
}