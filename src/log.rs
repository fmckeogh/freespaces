//! Logging and tracing utility functions

use {
    color_eyre::eyre::Result,
    tower_http::{
        classify::{ServerErrorsAsFailures, SharedClassifier},
        trace::{
            DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer,
        },
    },
    tracing::Level,
    tracing_subscriber::{filter::EnvFilter, fmt, prelude::*},
};

/// Initialises the global tracing subscriber
pub fn tracing_init() -> Result<()> {
    Ok(tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .try_init()?)
}

/// Creates a TraceLayer for request, response and failure logging
pub fn create_trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(
            DefaultMakeSpan::new()
                .level(Level::INFO)
                .include_headers(true),
        )
        // failures have the ERROR level
        .on_failure(DefaultOnFailure::new().level(Level::ERROR))
        // requests have the INFO level
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        // responses have the INFO level
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .include_headers(true),
        )
}
