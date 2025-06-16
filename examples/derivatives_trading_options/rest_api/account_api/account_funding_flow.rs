use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_options::{
    DerivativesTradingOptionsRestApi, rest_api::AccountFundingFlowParams,
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

    // Create the DerivativesTradingOptions REST API client
    let rest_client = DerivativesTradingOptionsRestApi::production(rest_conf);

    // Setup the API parameters
    let params = AccountFundingFlowParams::builder("currency_example".to_string()).build()?;

    // Make the API call
    let response = rest_client
        .account_funding_flow(params)
        .await
        .context("account_funding_flow request failed")?;

    info!(?response.rate_limits, "account_funding_flow rate limits");
    let data = response.data().await?;
    info!(?data, "account_funding_flow data");

    Ok(())
}
