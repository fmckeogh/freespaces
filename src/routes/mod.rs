use axum::{http::Uri, response::IntoResponse};

mod fetch;
mod health;
mod static_files;
mod submit;

pub use {fetch::fetch, health::health, static_files::static_files, submit::submit};

pub async fn index() -> impl IntoResponse {
    static_files(Uri::from_static("/index.html")).await
}
