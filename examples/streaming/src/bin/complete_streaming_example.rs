/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 25/10/25
******************************************************************************/

//! Complete example demonstrating all streaming capabilities.
//!
//! This example shows how to combine multiple subscription types:
//! - Market data (prices, market state)
//! - Price data (detailed bid/ask levels)
//! - Trade updates (confirmations, order updates)
//! - Account data (P&L, margin, equity)

use ig_client::application::client::StreamerClient;
use ig_client::error::AppError;
use ig_client::model::streaming::{
    StreamingAccountDataField, StreamingMarketField, StreamingPriceField,
};
use ig_client::prelude::setup_logger;
use std::collections::HashSet;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize logging
    setup_logger();

    info!("Starting complete streaming example...");

    // Create the streaming client
    let mut client = StreamerClient::new().await?;

    // Define instruments to monitor
    let epics = vec![
        "IX.D.DAX.DAILY.IP".to_string(),  // Germany 40
        "IX.D.FTSE.DAILY.IP".to_string(), // UK 100
    ];

    // 1. Subscribe to market data
    info!("Setting up market data subscription...");
    let market_fields = HashSet::from([
        StreamingMarketField::Bid,
        StreamingMarketField::Offer,
        StreamingMarketField::High,
        StreamingMarketField::Low,
        StreamingMarketField::Change,
        StreamingMarketField::ChangePct,
        StreamingMarketField::MarketState,
    ]);

    let mut market_receiver = client
        .market_subscribe(epics.clone(), market_fields)
        .await?;

    tokio::spawn(async move {
        while let Some(price_data) = market_receiver.recv().await {
            info!("Market update: {}", price_data);
        }
    });

    // 2. Subscribe to detailed price data
    info!("Setting up price data subscription...");
    let price_fields = HashSet::from([
        StreamingPriceField::BidPrice1,
        StreamingPriceField::BidPrice2,
        StreamingPriceField::AskPrice1,
        StreamingPriceField::AskPrice2,
        StreamingPriceField::BidSize1,
        StreamingPriceField::AskSize1,
        StreamingPriceField::Timestamp,
    ]);

    let mut price_receiver = client.price_subscribe(epics.clone(), price_fields).await?;

    tokio::spawn(async move {
        while let Some(price_data) = price_receiver.recv().await {
            info!("Price update: {}", price_data);
        }
    });

    // 3. Subscribe to trade updates
    info!("Setting up trade subscription...");
    let mut trade_receiver = client.trade_subscribe().await?;

    tokio::spawn(async move {
        while let Some(trade_data) = trade_receiver.recv().await {
            info!("Trade update: {}", trade_data);
        }
    });

    // 4. Subscribe to account data
    info!("Setting up account data subscription...");
    let account_fields = HashSet::from([
        StreamingAccountDataField::Pnl,
        StreamingAccountDataField::Margin,
        StreamingAccountDataField::Equity,
        StreamingAccountDataField::AvailableToDeal,
        StreamingAccountDataField::AvailableCash,
    ]);

    let mut account_receiver = client.account_subscribe(account_fields).await?;

    tokio::spawn(async move {
        while let Some(account_data) = account_receiver.recv().await {
            info!("Account update: {}", account_data);
        }
    });

    // Connect and maintain all connections
    // Market, trade, and account subscriptions use market_streamer_client
    // Price subscriptions use price_streamer_client
    // Both run in parallel
    info!("Connecting to Lightstreamer server...");
    info!("All subscriptions are now active. Press Ctrl+C to exit.");
    client.connect(None).await?;

    // Cleanup (only reached after graceful shutdown)
    info!("Complete streaming example completed");
    Ok(())
}
