use axum::{http::StatusCode, response::IntoResponse};

/// Tests node health
pub async fn health() -> impl IntoResponse {
    // todo fix this
    let can_fetch_from_saint_sport = true;

    if can_fetch_from_saint_sport {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}
