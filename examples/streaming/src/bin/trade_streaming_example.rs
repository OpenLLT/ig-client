/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 25/10/25
******************************************************************************/

//! Example demonstrating how to use the StreamerClient for real-time trade updates.
//!
//! This example shows how to subscribe to comprehensive trade data including:
//! - CONFIRMS: Trade confirmations and deal references
//! - OPU: Open Position Updates (new positions, position changes)
//! - WOU: Working Order Updates (order status changes, fills, cancellations)
//!
//! The trade streaming provides real-time updates about your trading activity,
//! allowing you to monitor position changes, order executions, and trade confirmations
//! as they happen.

use ig_client::application::client::StreamerClient;
use ig_client::error::AppError;
use ig_client::prelude::setup_logger;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize logging with detailed output
    setup_logger();

    info!("=== Starting Trade Streaming Example ===");
    info!("This example demonstrates real-time trade data streaming from IG Markets");
    info!("You will receive updates for:");
    info!("  • Trade confirmations (CONFIRMS)");
    info!("  • Open position updates (OPU)");
    info!("  • Working order updates (WOU)");
    info!("");

    // Create the streaming client
    let mut client = StreamerClient::new().await?;

    // Set up the trade subscription with comprehensive data handling
    // This subscription will receive all trade-related updates for your account
    info!("Setting up trade data subscription...");
    let mut receiver = client.trade_subscribe().await.map_err(|e| {
        error!("Failed to set up trade subscription: {}", e);
        e
    })?;

    // Spawn a task to handle incoming trade data updates
    tokio::spawn(async move {
        while let Some(trade_fields) = receiver.recv().await {
            // Log that we received a trade update
            info!("📊 Trade Update Received");

            // Process CONFIRMS field - Trade confirmations and deal references
            if let Some(confirms) = &trade_fields.confirms {
                info!("✅ TRADE CONFIRMATION:");
                info!("  Deal Reference: {}", confirms);
                info!("  This indicates a trade has been executed or confirmed");
            }

            // Process OPU (Open Position Update) - New positions or position changes
            if let Some(opu) = &trade_fields.opu {
                info!("📈 OPEN POSITION UPDATE:");

                if let Some(deal_ref) = &opu.deal_reference {
                    info!("  Deal Reference: {}", deal_ref);
                }

                if let Some(deal_id) = &opu.deal_id {
                    info!("  Deal ID: {}", deal_id);
                }

                if let Some(epic) = &opu.epic {
                    info!("  Instrument (Epic): {}", epic);
                }

                if let Some(direction) = &opu.direction {
                    info!("  Direction: {:?}", direction);
                }

                if let Some(size) = opu.size {
                    info!("  Position Size: {}", size);
                }

                if let Some(level) = opu.level {
                    info!("  Price Level: {}", level);
                }

                if let Some(currency) = &opu.currency {
                    info!("  Currency: {}", currency);
                }

                if let Some(status) = &opu.status {
                    info!("  Status: {:?}", status);
                }

                if let Some(deal_status) = &opu.deal_status {
                    info!("  Deal Status: {:?}", deal_status);
                }

                if let Some(timestamp) = &opu.timestamp {
                    info!("  Timestamp: {}", timestamp);
                }

                if let Some(channel) = &opu.channel {
                    info!("  Channel: {}", channel);
                }

                if let Some(expiry) = &opu.expiry {
                    info!("  Expiry: {}", expiry);
                }

                if let Some(deal_id_origin) = &opu.deal_id_origin {
                    info!("  Original Deal ID: {}", deal_id_origin);
                }
            }

            // Process WOU (Working Order Update) - Order status changes, fills, cancellations
            if let Some(wou) = &trade_fields.wou {
                info!("📋 WORKING ORDER UPDATE:");

                if let Some(deal_ref) = &wou.deal_reference {
                    info!("  Deal Reference: {}", deal_ref);
                }

                if let Some(deal_id) = &wou.deal_id {
                    info!("  Deal ID: {}", deal_id);
                }

                if let Some(epic) = &wou.epic {
                    info!("  Instrument (Epic): {}", epic);
                }

                if let Some(direction) = &wou.direction {
                    info!("  Direction: {:?}", direction);
                }

                if let Some(size) = wou.size {
                    info!("  Order Size: {}", size);
                }

                if let Some(level) = wou.level {
                    info!("  Order Level: {}", level);
                }

                if let Some(currency) = &wou.currency {
                    info!("  Currency: {}", currency);
                }

                if let Some(status) = &wou.status {
                    info!("  Status: {:?}", status);
                }

                if let Some(deal_status) = &wou.deal_status {
                    info!("  Deal Status: {:?}", deal_status);
                }

                if let Some(timestamp) = &wou.timestamp {
                    info!("  Timestamp: {}", timestamp);
                }

                if let Some(channel) = &wou.channel {
                    info!("  Channel: {}", channel);
                }

                if let Some(expiry) = &wou.expiry {
                    info!("  Expiry: {}", expiry);
                }

                if let Some(stop_distance) = wou.stop_distance {
                    info!("  Stop Distance: {}", stop_distance);
                }

                if let Some(limit_distance) = wou.limit_distance {
                    info!("  Limit Distance: {}", limit_distance);
                }

                if let Some(guaranteed_stop) = wou.guaranteed_stop {
                    info!("  Guaranteed Stop: {}", guaranteed_stop);
                }

                if let Some(order_type) = &wou.order_type {
                    info!("  Order Type: {:?}", order_type);
                }

                if let Some(time_in_force) = &wou.time_in_force {
                    info!("  Time in Force: {:?}", time_in_force);
                }

                if let Some(good_till_date) = &wou.good_till_date {
                    info!("  Good Till Date: {}", good_till_date);
                }
            }

            // Check if no trade data was received (empty update)
            if trade_fields.confirms.is_none()
                && trade_fields.opu.is_none()
                && trade_fields.wou.is_none()
            {
                warn!("⚠️  Received empty trade update (no CONFIRMS, OPU, or WOU data)");
            }

            info!(""); // Add spacing between updates
        }
    });

    info!("✅ Trade subscription configured successfully");
    info!("");
    info!("🔌 Connecting to Lightstreamer server...");
    info!("The connection will remain active until you press Ctrl+C");
    info!("Start trading to see real-time updates!");
    info!("");

    // Connect and maintain the connection
    // This will block until SIGINT/SIGTERM or connection failure
    match client.connect(None).await {
        Ok(_) => {
            info!("✅ Connected successfully to trade streaming");
        }
        Err(e) => {
            error!("❌ Failed to connect to streaming server: {}", e);
            return Err(e);
        }
    }

    // Cleanup (only reached after graceful shutdown)
    info!("🔌 Disconnecting from streaming server...");
    if let Err(e) = client.disconnect().await {
        warn!("Warning during disconnect: {}", e);
    }

    info!("=== Trade Streaming Example Completed ===");
    Ok(())
}
