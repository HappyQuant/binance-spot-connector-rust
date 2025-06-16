use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_coin_futures::{
    DerivativesTradingCoinFuturesRestApi, rest_api::GetCurrentPositionModeParams,
};

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

    // Create the DerivativesTradingCoinFutures REST API client
    let rest_client = DerivativesTradingCoinFuturesRestApi::production(rest_conf);

    // Setup the API parameters
    let params = GetCurrentPositionModeParams::default();

    // Make the API call
    let response = rest_client
        .get_current_position_mode(params)
        .await
        .context("get_current_position_mode request failed")?;

    info!(?response.rate_limits, "get_current_position_mode rate limits");
    let data = response.data().await?;
    info!(?data, "get_current_position_mode data");

    Ok(())
}
