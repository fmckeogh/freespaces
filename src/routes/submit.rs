use {
    crate::{error::Error, OccupancyLevel},
    axum::{
        extract::State,
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    },
    serde::{Deserialize, Serialize},
    sqlx::{Pool, Postgres},
};

const LOCATION_NOT_EXIST_ERROR_CODE: &str = "23503";

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationOccupancy {
    name: String,
    occupancy: OccupancyLevel,
}

/// Submit new occupancy
pub async fn submit(
    State(db): State<Pool<Postgres>>,
    Json(LocationOccupancy { name, occupancy }): Json<LocationOccupancy>,
) -> Response {
    let result = sqlx::query!(
        "INSERT INTO occupancies (location, occupancy) VALUES ($1, $2)",
        &name,
        occupancy as OccupancyLevel
    )
    .execute(&db)
    .await;

    if let Err(sqlx::Error::Database(e)) = &result {
        if e.code().as_deref() == Some(LOCATION_NOT_EXIST_ERROR_CODE) {
            return Error::UnknownLocationName(name).into_response();
        }
    }

    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => Error::DatabaseError(e).into_response(),
    }
}
