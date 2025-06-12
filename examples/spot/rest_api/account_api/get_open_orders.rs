use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::spot::{SpotRestApi, rest_api::GetOpenOrdersParams};

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
    let params = GetOpenOrdersParams::default();

    // Make the API call
    let response = rest_client
        .get_open_orders(params)
        .await
        .context("get_open_orders request failed")?;

    info!(?response.rate_limits, "get_open_orders rate limits");
    let data = response.data().await?;
    info!(?data, "get_open_orders data");

    Ok(())
}
