use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::crypto_loan::{
    CryptoLoanRestApi, rest_api::CheckCollateralRepayRateStableRateParams,
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

    // Create the CryptoLoan REST API client
    let rest_client = CryptoLoanRestApi::production(rest_conf);

    // Setup the API parameters
    let params = CheckCollateralRepayRateStableRateParams::builder(
        "loan_coin_example".to_string(),
        "collateral_coin_example".to_string(),
        dec!(1.0),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .check_collateral_repay_rate_stable_rate(params)
        .await
        .context("check_collateral_repay_rate_stable_rate request failed")?;

    info!(?response.rate_limits, "check_collateral_repay_rate_stable_rate rate limits");
    let data = response.data().await?;
    info!(?data, "check_collateral_repay_rate_stable_rate data");

    Ok(())
}
