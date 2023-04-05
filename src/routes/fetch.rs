use {
    crate::{LocationOccupancy, OccupancyLevel, FETCH_MAX_AGE},
    axum::{
        extract::State,
        http::{header::CACHE_CONTROL, HeaderMap, HeaderValue, StatusCode},
        response::IntoResponse,
        Json,
    },
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

    let mut headers = HeaderMap::new();
    headers.insert(
        CACHE_CONTROL,
        HeaderValue::from_str(&format!("public, max-age={FETCH_MAX_AGE}, immutable")).unwrap(),
    );

    (StatusCode::OK, headers, Json(locations))
}
