/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 25/10/25
******************************************************************************/

//! Example demonstrating how to use the StreamerClient for real-time market data.
//!
//! This example shows how to subscribe to market data updates including
//! bid/offer prices, high/low, changes, and market state.

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

    info!("Starting market streaming example...");

    // Create the streaming client
    let mut client = StreamerClient::new().await?;

    // Define the instruments to subscribe to
    let epics = vec![
        "IX.D.DAX.DAILY.IP".to_string(),   // Germany 40
        "IX.D.FTSE.DAILY.IP".to_string(),  // UK 100
        "IX.D.DOW.DAILY.IP".to_string(),   // Wall Street
        "IX.D.SPTRD.DAILY.IP".to_string(), // US 500
    ];

    // Define which market fields we want to receive
    let fields = HashSet::from([
        StreamingMarketField::Bid,         // Bid price
        StreamingMarketField::Offer,       // Offer/Ask price
        StreamingMarketField::High,        // High price
        StreamingMarketField::Low,         // Low price
        StreamingMarketField::MidOpen,     // Mid open price
        StreamingMarketField::Change,      // Price change
        StreamingMarketField::ChangePct,   // Percentage change
        StreamingMarketField::UpdateTime,  // Last update time
        StreamingMarketField::MarketDelay, // Market delay
        StreamingMarketField::MarketState, // Market state (TRADEABLE, etc.)
    ]);

    // Set up the market subscription (non-blocking)
    info!(
        "Setting up market data subscription for {} instruments...",
        epics.len()
    );
    let mut receiver = client
        .market_subscribe(epics.clone(), fields)
        .await?;

    // Spawn a task to handle incoming market data updates
    tokio::spawn(async move {
        while let Some(price_data) = receiver.recv().await {
            info!("Market update - {}", price_data);
        }
    });

    // You can add more subscriptions here if needed
    // For example, price data subscriptions:
    // client.price_subscribe(...).await?;
    // client.trade_subscribe(...).await?;
    // client.account_subscribe(...).await?;

    // Connect and maintain the connection
    // This will block until SIGINT/SIGTERM or connection failure
    info!("Connecting to Lightstreamer server...");
    info!(
        "Monitoring {} instruments. Press Ctrl+C to exit.",
        epics.len()
    );
    client.connect(None).await?;

    // Cleanup (only reached after graceful shutdown)
    info!("Market streaming example completed");
    Ok(())
}
