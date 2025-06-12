use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::wallet::WalletRestApi;

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

    // Create the Wallet REST API client
    let rest_client = WalletRestApi::production(rest_conf);

    // Make the API call
    let response = rest_client
        .get_open_symbol_list()
        .await
        .context("get_open_symbol_list request failed")?;

    info!(?response.rate_limits, "get_open_symbol_list rate limits");
    let data = response.data().await?;
    info!(?data, "get_open_symbol_list data");

    Ok(())
}
