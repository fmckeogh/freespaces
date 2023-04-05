use {
    crate::{LocationOccupancy, OccupancyLevel},
    axum::{http::StatusCode, response::IntoResponse, Json},
};

/// Fetch current occupancy status
pub async fn fetch() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(vec![
            LocationOccupancy {
                name: "Main Library".to_owned(),
                occupancy: OccupancyLevel::Medium,
            },
            LocationOccupancy {
                name: "St Mary's".to_owned(),
                occupancy: OccupancyLevel::High,
            },
            LocationOccupancy {
                name: "Taste Cafe".to_owned(),
                occupancy: OccupancyLevel::Low,
            },
        ]),
    )
}
