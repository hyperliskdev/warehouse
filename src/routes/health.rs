

// Health Route
pub async fn health() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&"Healthy"))
}