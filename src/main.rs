use {
    color_eyre::eyre::Result,
    freespace::{start, Config},
};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    start(&Config::new()?).await?.join().await
}
