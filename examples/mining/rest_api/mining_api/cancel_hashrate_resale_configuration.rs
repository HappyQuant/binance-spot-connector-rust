use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::mining::{MiningRestApi, rest_api::CancelHashrateResaleConfigurationParams};

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

    // Setup the API parameters
    let params =
        CancelHashrateResaleConfigurationParams::builder(1, "user_name_example".to_string())
            .build()?;

    // Make the API call
    let response = rest_client
        .cancel_hashrate_resale_configuration(params)
        .await
        .context("cancel_hashrate_resale_configuration request failed")?;

    info!(?response.rate_limits, "cancel_hashrate_resale_configuration rate limits");
    let data = response.data().await?;
    info!(?data, "cancel_hashrate_resale_configuration data");

    Ok(())
}
