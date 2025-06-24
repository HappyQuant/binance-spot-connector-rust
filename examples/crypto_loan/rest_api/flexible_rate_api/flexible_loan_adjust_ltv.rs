use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::crypto_loan::{CryptoLoanRestApi, rest_api::FlexibleLoanAdjustLtvParams};

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

    // Create the CryptoLoan REST API client
    let rest_client = CryptoLoanRestApi::production(rest_conf);

    // Setup the API parameters
    let params = FlexibleLoanAdjustLtvParams::builder(
        "loan_coin_example".to_string(),
        "collateral_coin_example".to_string(),
        dec!(1.0),
        "direction_example".to_string(),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .flexible_loan_adjust_ltv(params)
        .await
        .context("flexible_loan_adjust_ltv request failed")?;

    info!(?response.rate_limits, "flexible_loan_adjust_ltv rate limits");
    let data = response.data().await?;
    info!(?data, "flexible_loan_adjust_ltv data");

    Ok(())
}
