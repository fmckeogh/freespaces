use {
    crate::{error::Error, LocationOccupancy, OccupancyLevel, FETCH_MAX_AGE},
    axum::{
        extract::State,
        http::{header::CACHE_CONTROL, HeaderMap, HeaderValue, StatusCode},
        response::IntoResponse,
        Json,
    },
    chrono::{DateTime, Utc},
    itertools::{Group, Itertools},
    sqlx::{Pool, Postgres},
};

struct Model {
    location: String,
    timestamp: DateTime<Utc>,
    occupancy: OccupancyLevel,
}

/// Fetch current occupancy status
pub async fn fetch(State(db): State<Pool<Postgres>>) -> Result<impl IntoResponse, Error> {
    let groups = sqlx::query_as!(
        Model,
        r#"
            SELECT location, timestamp, occupancy as "occupancy: _"
            FROM occupancies
            ORDER BY location, timestamp DESC
        "#
    )
    .fetch_all(&db)
    .await?
    .into_iter()
    .group_by(|Model { location, .. }| location.clone());

    let locations = groups
        .into_iter()
        .map(process_submissions)
        .collect::<Vec<_>>();

    let mut headers = HeaderMap::new();
    headers.insert(
        CACHE_CONTROL,
        HeaderValue::from_str(&format!("public, max-age={FETCH_MAX_AGE}, immutable")).unwrap(),
    );

    Ok((StatusCode::OK, headers, Json(locations)))
}

/// Algorithm for taking a series of occupancy levels for a location and producing a single occupancy level.
///
/// Currently submissions are weighted inversely to their age.
fn process_submissions<F: FnMut(&Model) -> String, I: Iterator<Item = Model>>(
    (name, group): (String, Group<String, I, F>),
) -> LocationOccupancy {
    const SCALE: f64 = 1.0;
    const TIME: f64 = 3600.0;

    let values = group
        .into_iter()
        .map(
            |Model {
                 timestamp,
                 occupancy,
                 ..
             }| {
                let occupancy_value = match occupancy {
                    OccupancyLevel::Low => -1.0,
                    OccupancyLevel::Medium => 0.0,
                    OccupancyLevel::High => 1.0,
                };

                (timestamp.timestamp(), occupancy_value)
            },
        )
        .collect::<Vec<_>>();

    let most_recent = values[0].0;

    let weighted_mean = values
        .iter()
        .map(|(ts, value)| {
            let elapsed = (most_recent - *ts) as f64;

            let weight = SCALE * (-elapsed / TIME).exp();

            *value * weight
        })
        .sum::<f64>()
        / values.len() as f64;

    let occupancy = if weighted_mean > 0.5 {
        OccupancyLevel::High
    } else if weighted_mean > -0.5 {
        OccupancyLevel::Medium
    } else {
        OccupancyLevel::Low
    };

    LocationOccupancy { name, occupancy }
}
