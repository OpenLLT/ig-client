//! Example demonstrating how to use the StreamerClient for real-time chart data.
//!
//! This example shows how to subscribe to tick or candle (aggregated) chart updates,
//! receive live OHLCV information, and process chart updates in real time.

use ig_client::application::client::StreamerClient;
use ig_client::error::AppError;
use ig_client::model::streaming::StreamingChartField;
use ig_client::prelude::{ChartScale, setup_logger};
use std::collections::HashSet;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize logging
    setup_logger();

    info!("Starting chart streaming example...");

    // Create the streaming client
    let mut client = StreamerClient::new().await?;

    // Define the instruments (EPICs) to subscribe to
    let epics = vec![
        "OP.D.OTCBTCWK.114500C.IP".to_string(),
        "DO.D.OTCDETH.21.IP".to_string(),
    ];

    // Choose which chart scale to stream
    // You can use ChartScale::Tick for tick-by-tick, or OneMinute, FiveMinute, Hour, etc.
    let scale = ChartScale::OneMinute;

    // Define which chart fields we want to receive
    // For tick data you might want Bid, Ofr, Ltp.
    let fields = HashSet::from([
        StreamingChartField::Ltv,
        StreamingChartField::Ttv,
        StreamingChartField::Utm,
        StreamingChartField::DayOpenMid,
        StreamingChartField::DayNetChgMid,
        StreamingChartField::DayPercChgMid,
        StreamingChartField::DayHigh,
        StreamingChartField::DayLow,
        StreamingChartField::OfrOpen,
        StreamingChartField::OfrHigh,
        StreamingChartField::OfrLow,
        StreamingChartField::OfrClose,
        StreamingChartField::BidOpen,
        StreamingChartField::BidHigh,
        StreamingChartField::BidLow,
        StreamingChartField::BidClose,
        StreamingChartField::LtpOpen,
        StreamingChartField::LtpHigh,
        StreamingChartField::LtpLow,
        StreamingChartField::LtpClose,
        StreamingChartField::ConsEnd,
        StreamingChartField::ConsTickCount,
    ]);

    // Set up the chart subscription (non-blocking)
    info!("Setting up chart data subscription for {scale:?} candles...");
    let mut receiver = client.chart_subscribe(epics.clone(), scale, fields).await?;

    // Spawn a task to handle incoming chart data updates
    tokio::spawn(async move {
        while let Some(chart_data) = receiver.recv().await {
            // You can use the helper methods like .is_candle() or .is_tick()
            if chart_data.is_candle() {
                info!(
                    "[{}] {:?} Candle update: {:?}",
                    chart_data.item_name, chart_data.scale, chart_data.fields
                );
            } else {
                info!(
                    "[{}] Tick update: {:?}",
                    chart_data.item_name, chart_data.fields
                );
            }
        }
    });

    // Maintain the connection
    info!("Connecting to Lightstreamer server...");
    client.connect(None).await?;

    // Cleanup after graceful shutdown
    info!("Chart streaming example completed");
    Ok(())
}
