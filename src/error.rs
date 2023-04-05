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
    /// Failed to get gym status information
    StatusRequestFailed,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("Error occurred when handling request: {}", self);

        let status_code = match self {
            Error::StatusRequestFailed => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, self.to_string()).into_response()
    }
}
