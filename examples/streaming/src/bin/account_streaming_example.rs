/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 25/10/25
******************************************************************************/

//! Example demonstrating how to use the StreamerClient for real-time account data.
//!
//! This example shows how to subscribe to account updates including profit/loss,
//! margin, equity, and available funds.

use ig_client::application::client::StreamerClient;
use ig_client::error::AppError;
use ig_client::model::streaming::StreamingAccountDataField;
use ig_client::prelude::setup_logger;
use std::collections::HashSet;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize logging
    setup_logger();

    info!("Starting account streaming example...");

    // Create the streaming client
    let mut client = StreamerClient::new().await?;

    // Define which account fields we want to receive
    let fields = HashSet::from([
        StreamingAccountDataField::Pnl,             // Profit and loss
        StreamingAccountDataField::Margin,          // Total margin
        StreamingAccountDataField::MarginLr,        // Margin with guaranteed stops
        StreamingAccountDataField::MarginNlr,       // Margin without guaranteed stops
        StreamingAccountDataField::Equity,          // Total equity
        StreamingAccountDataField::EquityUsed,      // Equity used
        StreamingAccountDataField::AvailableToDeal, // Available to deal
        StreamingAccountDataField::AvailableCash,   // Available cash
        StreamingAccountDataField::Funds,           // Total funds
        StreamingAccountDataField::Deposit,         // Deposit amount
    ]);

    // Set up the account subscription (non-blocking)
    info!("Setting up account data subscription...");
    let mut receiver = client
        .account_subscribe(fields)
        .await?;

    // Spawn a task to handle incoming account data updates
    tokio::spawn(async move {
        while let Some(account_data) = receiver.recv().await {
            info!("Account update: {}", account_data);
        }
    });

    // You can also add other subscriptions
    // client.market_subscribe(...).await?;
    // client.price_subscribe(...).await?;
    // client.trade_subscribe(...).await?;

    // Connect and maintain the connection
    // This will block until SIGINT/SIGTERM or connection failure
    info!("Connecting to Lightstreamer server...");
    client.connect(None).await?;

    // Cleanup (only reached after graceful shutdown)
    info!("Account streaming example completed");
    Ok(())
}
