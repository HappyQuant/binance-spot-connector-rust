// Class name: websocket_market_streams_api
use anyhow::{Context, Result};
use tokio::time::{Duration, sleep};
use tracing::info;

use binance_sdk::config::ConfigurationWebsocketStreams;
use binance_sdk::derivatives_trading_options::{
    DerivativesTradingOptionsWsStreams,
    websocket_streams::Ticker24HourByUnderlyingAssetAndExpirationDataParams,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Build WebSocket Streams config
    let ws_streams_conf = ConfigurationWebsocketStreams::builder().build()?;

    // Create the DerivativesTradingOptions WebSocket Streams client
    let ws_streams_client = DerivativesTradingOptionsWsStreams::production(ws_streams_conf);

    // Connect to WebSocket
    let connection = ws_streams_client
        .connect()
        .await
        .context("Failed to connect to WebSocket Streams")?;

    // Setup the stream parameters
    let params = Ticker24HourByUnderlyingAssetAndExpirationDataParams::builder(
        "ETH".to_string(),
        "220930".to_string(),
    )
    .build()?;

    // Subscribe to the stream
    let stream = connection
        .ticker24_hour_by_underlying_asset_and_expiration_data(params)
        .await
        .context("Failed to subscribe to the stream")?;

    // Register callback for incoming messages
    stream.on_message(|data| {
        info!("{:?}", data);
    });

    // Disconnect after 20 seconds
    sleep(Duration::from_secs(20)).await;
    connection
        .disconnect()
        .await
        .context("Failed to disconnect WebSocket client")?;

    Ok(())
}
