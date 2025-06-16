use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_portfolio_margin::{
    DerivativesTradingPortfolioMarginRestApi, rest_api::QueryAllUmOrdersParams,
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

    // Create the DerivativesTradingPortfolioMargin REST API client
    let rest_client = DerivativesTradingPortfolioMarginRestApi::production(rest_conf);

    // Setup the API parameters
    let params = QueryAllUmOrdersParams::builder("symbol_example".to_string()).build()?;

    // Make the API call
    let response = rest_client
        .query_all_um_orders(params)
        .await
        .context("query_all_um_orders request failed")?;

    info!(?response.rate_limits, "query_all_um_orders rate limits");
    let data = response.data().await?;
    info!(?data, "query_all_um_orders data");

    Ok(())
}
