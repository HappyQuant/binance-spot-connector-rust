use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::spot::{SpotRestApi, rest_api::RateLimitOrderParams};

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

    // Create the Spot REST API client
    let rest_client = SpotRestApi::production(rest_conf);

    // Setup the API parameters
    let params = RateLimitOrderParams::default();

    // Make the API call
    let response = rest_client
        .rate_limit_order(params)
        .await
        .context("rate_limit_order request failed")?;

    info!(?response.rate_limits, "rate_limit_order rate limits");
    let data = response.data().await?;
    info!(?data, "rate_limit_order data");

    Ok(())
}
