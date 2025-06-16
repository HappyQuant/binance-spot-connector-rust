use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::vip_loan::{VIPLoanRestApi, rest_api::QueryApplicationStatusParams};

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
    let params = QueryApplicationStatusParams::default();

    // Make the API call
    let response = rest_client
        .query_application_status(params)
        .await
        .context("query_application_status request failed")?;

    info!(?response.rate_limits, "query_application_status rate limits");
    let data = response.data().await?;
    info!(?data, "query_application_status data");

    Ok(())
}
