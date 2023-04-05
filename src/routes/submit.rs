use {
    crate::{LocationOccupancy, OccupancyLevel},
    axum::{extract::State, http::StatusCode, response::IntoResponse, Json},
    sqlx::{Pool, Postgres},
};

/// Submit new occupancy
pub async fn submit(
    State(db): State<Pool<Postgres>>,
    Json(LocationOccupancy { name, occupancy }): Json<LocationOccupancy>,
) -> impl IntoResponse {
    sqlx::query!(
        "INSERT INTO occupancies (location, occupancy) VALUES ($1, $2)",
        name,
        occupancy as OccupancyLevel
    )
    .execute(&db)
    .await
    .unwrap();

    StatusCode::OK
}
