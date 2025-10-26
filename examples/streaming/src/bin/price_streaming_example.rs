/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 25/10/25
******************************************************************************/

//! Example demonstrating how to use the StreamerClient for real-time price data.
//!
//! This example shows how to subscribe to detailed price information including
//! multiple price levels, sizes, and currency information.

use ig_client::application::client::StreamerClient;
use ig_client::error::AppError;
use ig_client::model::streaming::StreamingPriceField;
use ig_client::prelude::setup_logger;
use std::collections::HashSet;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize logging
    setup_logger();

    info!("Starting price streaming example...");

    // Create the streaming client
    let mut client = StreamerClient::new().await?;

    // Define the instruments to subscribe to
    let epics = vec![
        "IX.D.DAX.DAILY.IP".to_string(),  // Germany 40
        "IX.D.FTSE.DAILY.IP".to_string(), // UK 100
        "IX.D.DOW.DAILY.IP".to_string(),  // Wall Street
    ];

    // Define which price fields we want to receive
    // This includes multiple price levels and sizes
    let fields = HashSet::from([
        StreamingPriceField::BidPrice1,
        StreamingPriceField::BidPrice2,
        StreamingPriceField::BidPrice3,
        StreamingPriceField::AskPrice1,
        StreamingPriceField::AskPrice2,
        StreamingPriceField::AskPrice3,
        StreamingPriceField::BidSize1,
        StreamingPriceField::BidSize2,
        StreamingPriceField::BidSize3,
        StreamingPriceField::AskSize1,
        StreamingPriceField::AskSize2,
        StreamingPriceField::AskSize3,
        StreamingPriceField::MidOpen,
        StreamingPriceField::High,
        StreamingPriceField::Low,
        StreamingPriceField::Timestamp,
        StreamingPriceField::BidQuoteId,
        StreamingPriceField::AskQuoteId,
    ]);

    // Set up the price subscription (non-blocking)
    info!("Setting up price data subscription...");
    let mut receiver = client
        .price_subscribe(epics.clone(), fields)
        .await?;

    // Spawn a task to handle incoming price data updates
    tokio::spawn(async move {
        while let Some(price_data) = receiver.recv().await {
            info!("Price update - {}", price_data);
        }
    });

    // You can add more subscriptions here if needed
    // For example, market data subscriptions:
    // client.market_subscribe(...).await?;

    // Connect and maintain the connection
    // This will block until SIGINT/SIGTERM or connection failure
    info!("Connecting to Lightstreamer server...");
    client.connect(None).await?;

    // Cleanup (only reached after graceful shutdown)
    info!("Price streaming example completed");
    Ok(())
}
