use {
    crate::LocationOccupancy,
    axum::{http::StatusCode, response::IntoResponse, Json},
};

/// Submit new occupancy
pub async fn submit(Json(_payload): Json<LocationOccupancy>) -> impl IntoResponse {
    StatusCode::OK
}
