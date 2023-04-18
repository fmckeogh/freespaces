use {
    crate::{
        log::{create_trace_layer, tracing_init},
        routes::{fetch, health, images, index, static_files, submit},
    },
    axum::{routing::get, Router},
    color_eyre::eyre::Result,
    serde::{Deserialize, Serialize},
    sqlx::postgres::PgPoolOptions,
    std::{net::SocketAddr, time::Duration},
    tokio::task::JoinHandle,
    tower_http::compression::CompressionLayer,
    tracing::info,
};

pub mod config;
pub mod error;
pub mod log;
pub mod routes;

pub use crate::config::Config;

/// Static files cached time in seconds
const STATIC_FILES_MAX_AGE: u64 = 300;

/// Location status cached time in seconds
const FETCH_MAX_AGE: u64 = 10;

/// Starts a new instance of the contractor returning a handle
pub async fn start(config: &Config) -> Result<Handle> {
    // initialize global tracing subscriber
    tracing_init()?;

    config::init(config.clone()).await;

    let pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let compression = CompressionLayer::new().br(true).deflate(true).gzip(true);

    // create router with all routes and tracing layer
    let router = Router::new()
        .route("/health", get(health))
        .route("/locations", get(fetch).post(submit))
        .route("/images/:name", get(images))
        .route("/", get(index))
        .fallback(static_files)
        .with_state(pool)
        .layer(compression)
        .layer(create_trace_layer());

    // bind axum server to socket address and use router to create a service factory
    let server = axum::Server::bind(&config::get().address).serve(router.into_make_service());

    // get address server is bound to (may be different to address passed to Server::bind)
    let address = server.local_addr();

    // spawn server on new tokio task
    let handle = tokio::spawn(async { server.await.map_err(Into::into) });

    info!("started on http://{}", address);

    // return handles
    Ok(Handle { address, handle })
}

/// Handle for running an instance
pub struct Handle {
    // Socket address instance is bound to
    address: SocketAddr,
    // JoinHandle for server task
    handle: JoinHandle<Result<()>>,
}

impl Handle {
    /// Gets the socket address the running instance is bound to
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    /// Awaits on the instance's task
    pub async fn join(self) -> Result<()> {
        self.handle.await??;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationOccupancy {
    name: String,
    occupancy: OccupancyLevel,
    timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "occupancy_level", rename_all = "lowercase")]
pub enum OccupancyLevel {
    Low,
    Medium,
    High,
}
