use anyhow::{Context, Result};
use rust_decimal::prelude::*;
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::wallet::{WalletRestApi, rest_api::WithdrawTravelRuleParams};

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

    // Setup the API parameters
    let params = WithdrawTravelRuleParams::builder(
        "coin_example".to_string(),
        "address_example".to_string(),
        dec!(1.0),
        "questionnaire_example".to_string(),
    )
    .build()?;

    // Make the API call
    let response = rest_client
        .withdraw_travel_rule(params)
        .await
        .context("withdraw_travel_rule request failed")?;

    info!(?response.rate_limits, "withdraw_travel_rule rate limits");
    let data = response.data().await?;
    info!(?data, "withdraw_travel_rule data");

    Ok(())
}
