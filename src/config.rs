//! App configuration

use {
    async_once_cell::OnceCell,
    color_eyre::eyre::{Result, WrapErr},
    config::Environment,
    serde::Deserialize,
    std::net::SocketAddr,
};

static CONFIG: OnceCell<Config> = OnceCell::new();

pub async fn init(config: Config) {
    CONFIG.get_or_init(async { config }).await;
}

pub fn get() -> &'static Config {
    CONFIG.get().unwrap()
}

/// Contractor configuration parameters
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    /// Socket to bind HTTP server to
    pub address: SocketAddr,

    /// Database URL
    pub database_url: String,
}

impl Config {
    /// Builds a new Config instance from an optional file (the path of which is supplied as a argument) and, with a greater priority, environment variables
    pub fn new() -> Result<Self> {
        dotenv::dotenv().ok();

        Ok(config::Config::builder()
            .add_source(Environment::default())
            .build()
            .wrap_err("Failed build configuration")?
            .try_deserialize()
            .wrap_err("Failed deserialize configuration")?)
    }
}
