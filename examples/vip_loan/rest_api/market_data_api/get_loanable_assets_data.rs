use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::vip_loan::{VIPLoanRestApi, rest_api::GetLoanableAssetsDataParams};

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

    // Create the VIPLoan REST API client
    let rest_client = VIPLoanRestApi::production(rest_conf);

    // Setup the API parameters
    let params = GetLoanableAssetsDataParams::default();

    // Make the API call
    let response = rest_client
        .get_loanable_assets_data(params)
        .await
        .context("get_loanable_assets_data request failed")?;

    info!(?response.rate_limits, "get_loanable_assets_data rate limits");
    let data = response.data().await?;
    info!(?data, "get_loanable_assets_data data");

    Ok(())
}
