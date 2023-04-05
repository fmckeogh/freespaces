//! Logging and tracing utility functions

use {
    color_eyre::eyre::Result,
    std::path::Path,
    tower_http::{
        classify::{ServerErrorsAsFailures, SharedClassifier},
        trace::{
            DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer,
        },
    },
    tracing::Level,
    tracing_appender::{
        non_blocking::WorkerGuard,
        rolling::{RollingFileAppender, Rotation},
    },
    tracing_subscriber::{
        filter::{self, EnvFilter},
        fmt,
        prelude::*,
    },
};

/// Initialises the global tracing subscriber
pub fn tracing_init<P: AsRef<Path>>(logfile_path: P) -> Result<WorkerGuard> {
    let (non_blocking, guard) = tracing_appender::non_blocking(RollingFileAppender::new(
        Rotation::DAILY,
        logfile_path,
        "log.txt",
    ));

    let mut layer = fmt::Layer::default();
    layer.set_ansi(false);

    let logfile = layer
        .with_writer(non_blocking)
        .with_filter(filter::LevelFilter::WARN);

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .with(logfile)
        .try_init()?;

    Ok(guard)
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
