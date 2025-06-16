use anyhow::{Context, Result};
use std::env;
use tracing::info;

use binance_sdk::config::ConfigurationRestApi;
use binance_sdk::derivatives_trading_portfolio_margin_pro::{
    DerivativesTradingPortfolioMarginProRestApi,
    rest_api::PortfolioMarginProBankruptcyLoanRepayParams,
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

    // Create the DerivativesTradingPortfolioMarginPro REST API client
    let rest_client = DerivativesTradingPortfolioMarginProRestApi::production(rest_conf);

    // Setup the API parameters
    let params = PortfolioMarginProBankruptcyLoanRepayParams::default();

    // Make the API call
    let response = rest_client
        .portfolio_margin_pro_bankruptcy_loan_repay(params)
        .await
        .context("portfolio_margin_pro_bankruptcy_loan_repay request failed")?;

    info!(?response.rate_limits, "portfolio_margin_pro_bankruptcy_loan_repay rate limits");
    let data = response.data().await?;
    info!(?data, "portfolio_margin_pro_bankruptcy_loan_repay data");

    Ok(())
}
