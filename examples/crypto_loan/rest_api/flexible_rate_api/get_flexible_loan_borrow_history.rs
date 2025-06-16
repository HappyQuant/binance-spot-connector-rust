use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::crypto_loan::{CryptoLoanRestApi, rest_api::GetFlexibleLoanBorrowHistoryParams};

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
    let params = GetFlexibleLoanBorrowHistoryParams::default();

    // Make the API call
    let response = rest_client
        .get_flexible_loan_borrow_history(params)
        .await
        .context("get_flexible_loan_borrow_history request failed")?;

    info!(?response.rate_limits, "get_flexible_loan_borrow_history rate limits");
    let data = response.data().await?;
    info!(?data, "get_flexible_loan_borrow_history data");

    Ok(())
}
