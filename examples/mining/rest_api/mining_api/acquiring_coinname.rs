use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::mining::MiningRestApi;

#[tokio::main]
async fn main() -> Result<()> {
    // Load credentials from env
    let api_key = env::var("API_KEY").context("API_KEY must be set")?;
    let api_secret = env::var("API_SECRET").context("API_SECRET must be set")?;

    // Build REST config
    let rest_conf = ConfigurationRestApi::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .build()?;

    // Create the Mining REST API client
    let rest_client = MiningRestApi::production(rest_conf);

    // Make the API call
    let response = rest_client
        .acquiring_coinname()
        .await
        .context("acquiring_coinname request failed")?;

    info!(?response.rate_limits, "acquiring_coinname rate limits");
    let data = response.data().await?;
    info!(?data, "acquiring_coinname data");

    Ok(())
}
