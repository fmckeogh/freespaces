use {
    crate::{LocationOccupancy, OccupancyLevel},
    axum::{extract::State, http::StatusCode, response::IntoResponse, Json},
    chrono::{DateTime, Utc},
    sqlx::{Pool, Postgres},
};

struct DbOccupancy {
    location: String,
    #[allow(dead_code)]
    timestamp: Option<DateTime<Utc>>,
    occupancy: OccupancyLevel,
}

/// Fetch current occupancy status
pub async fn fetch(State(db): State<Pool<Postgres>>) -> impl IntoResponse {
    let locations = sqlx::query_as!(
        DbOccupancy,
        r#"SELECT location, MAX(timestamp) AS timestamp, occupancy as "occupancy: _" FROM occupancies GROUP BY location, occupancy"#
    )
    .fetch_all(&db)
    .await
    .unwrap()
        .into_iter()
        .map(
            |DbOccupancy {
                 location,
                 occupancy,
                 ..
             }| LocationOccupancy {
                name: location,
                occupancy,
            },
        )
        .collect::<Vec<_>>();

    (StatusCode::OK, Json(locations))
}
