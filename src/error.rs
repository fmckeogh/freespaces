//! Error handling

use {
    axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
    },
    tracing::error,
};

/// Route error, presented to user through `IntoResponse` impl
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum Error {
    /// Unknown location name {0:?}
    UnknownLocationName(String),
    /// File {0:?} not found
    FileNotFound(String),
    /// Database error
    DatabaseError(#[from] sqlx::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("Error occurred when handling request: {}", self);

        let status_code = match self {
            Error::UnknownLocationName(_) => StatusCode::BAD_REQUEST,
            Error::FileNotFound(_) => StatusCode::NOT_FOUND,
            Error::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, self.to_string()).into_response()
    }
}
