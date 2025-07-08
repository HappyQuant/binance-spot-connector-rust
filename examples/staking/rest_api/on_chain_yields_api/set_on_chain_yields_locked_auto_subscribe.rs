use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::staking::{StakingRestApi, rest_api::SetOnChainYieldsLockedAutoSubscribeParams};

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

    // Create the Staking REST API client
    let rest_client = StakingRestApi::production(rest_conf);

    // Setup the API parameters
    let params =
        SetOnChainYieldsLockedAutoSubscribeParams::builder("1".to_string(), true).build()?;

    // Make the API call
    let response = rest_client
        .set_on_chain_yields_locked_auto_subscribe(params)
        .await
        .context("set_on_chain_yields_locked_auto_subscribe request failed")?;

    info!(?response.rate_limits, "set_on_chain_yields_locked_auto_subscribe rate limits");
    let data = response.data().await?;
    info!(?data, "set_on_chain_yields_locked_auto_subscribe data");

    Ok(())
}
