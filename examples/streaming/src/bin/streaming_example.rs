/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 25/10/25
******************************************************************************/

//! Example demonstrating how to use the StreamerClient for real-time market data.
//!
//! This example shows the improved pattern for streaming data:
//! 1. Create the client
//! 2. Set up one or more subscriptions
//! 3. Connect and maintain the connection
//! 4. Handle graceful shutdown

use ig_client::application::client::StreamerClient;
use ig_client::error::AppError;
use ig_client::model::streaming::StreamingMarketField;
use ig_client::prelude::setup_logger;
use std::collections::HashSet;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize logging
    setup_logger();

    info!("Starting streaming example...");

    // Create the streaming client
    let mut client = StreamerClient::new().await?;

    // Define the instruments to subscribe to
    let epics = vec![
        "IX.D.DAX.DAILY.IP".to_string(),  // Germany 40
        "IX.D.FTSE.DAILY.IP".to_string(), // UK 100
        "IX.D.DOW.DAILY.IP".to_string(),  // Wall Street
    ];

    // Define which fields we want to receive
    let fields = HashSet::from([
        StreamingMarketField::Bid,
        StreamingMarketField::Offer,
        StreamingMarketField::High,
        StreamingMarketField::Low,
        StreamingMarketField::MidOpen,
        StreamingMarketField::Change,
        StreamingMarketField::ChangePct,
        StreamingMarketField::UpdateTime,
        StreamingMarketField::MarketDelay,
        StreamingMarketField::MarketState,
    ]);

    // Set up the subscription (non-blocking)
    info!("Setting up market data subscription...");
    let mut receiver = client.market_subscribe(epics.clone(), fields).await?;

    // Spawn a task to handle incoming market data updates
    tokio::spawn(async move {
        while let Some(price_data) = receiver.recv().await {
            info!("Price update - {}", price_data);
        }
    });

    // You can add more subscriptions here if needed
    // client.market_subscribe(...).await?;

    // Connect and maintain the connection
    // This will block until SIGINT/SIGTERM or connection failure
    info!("Connecting to Lightstreamer server...");
    client.connect(None).await?;

    // Cleanup (only reached after graceful shutdown)
    info!("Streaming example completed");
    Ok(())
}
