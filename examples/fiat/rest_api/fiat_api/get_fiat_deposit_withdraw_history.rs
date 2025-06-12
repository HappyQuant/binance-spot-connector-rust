use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::fiat::{FiatRestApi, rest_api::GetFiatDepositWithdrawHistoryParams};

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

    // Create the Fiat REST API client
    let rest_client = FiatRestApi::production(rest_conf);

    // Setup the API parameters
    let params =
        GetFiatDepositWithdrawHistoryParams::builder("transaction_type_example".to_string())
            .build()?;

    // Make the API call
    let response = rest_client
        .get_fiat_deposit_withdraw_history(params)
        .await
        .context("get_fiat_deposit_withdraw_history request failed")?;

    info!(?response.rate_limits, "get_fiat_deposit_withdraw_history rate limits");
    let data = response.data().await?;
    info!(?data, "get_fiat_deposit_withdraw_history data");

    Ok(())
}
