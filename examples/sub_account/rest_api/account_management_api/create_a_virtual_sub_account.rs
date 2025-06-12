use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::sub_account::{SubAccountRestApi, rest_api::CreateAVirtualSubAccountParams};

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

    // Create the SubAccount REST API client
    let rest_client = SubAccountRestApi::production(rest_conf);

    // Setup the API parameters
    let params = CreateAVirtualSubAccountParams::builder("sub_account_string_example".to_string())
        .build()?;

    // Make the API call
    let response = rest_client
        .create_a_virtual_sub_account(params)
        .await
        .context("create_a_virtual_sub_account request failed")?;

    info!(?response.rate_limits, "create_a_virtual_sub_account rate limits");
    let data = response.data().await?;
    info!(?data, "create_a_virtual_sub_account data");

    Ok(())
}
