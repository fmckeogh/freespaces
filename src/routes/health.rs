use {
    axum::{extract::State, http::StatusCode, response::IntoResponse},
    sqlx::{Pool, Postgres},
};

/// Tests node health
pub async fn health(State(db): State<Pool<Postgres>>) -> impl IntoResponse {
    if sqlx::query("SELECT 1").fetch_one(&db).await.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}
