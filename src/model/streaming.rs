/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 25/10/25
******************************************************************************/

//! Streaming data field definitions for IG Markets API.
//!
//! This module provides enums and helper functions for working with streaming
//! subscriptions in the IG Markets API. It includes field definitions for:
//! - Market data (prices, market state)
//! - Price data (detailed bid/ask levels)
//! - Account data (P&L, margin, equity)

use crate::prelude::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::{Debug, Display};

/// Streaming market fields available for market subscriptions.
///
/// These fields represent the various market data points that can be subscribed to
/// in the IG Markets streaming API for market updates.
#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Default, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamingMarketField {
    /// Mid open price
    MidOpen,
    /// High price
    High,
    /// Low price
    Low,
    /// Price change
    Change,
    /// Percentage change
    ChangePct,
    /// Last update time
    UpdateTime,
    /// Market delay in milliseconds
    MarketDelay,
    /// Market state (e.g., TRADEABLE, CLOSED)
    MarketState,
    /// Bid price
    Bid,
    /// Offer/Ask price
    #[default]
    Offer,
}

impl Debug for StreamingMarketField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let field_name = serde_json::to_string(self).unwrap();
        write!(f, "{:?}", field_name)
    }
}

impl Display for StreamingMarketField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Constructs a vector of serialized streaming market field names from a given set of `StreamingMarketField`.
///
/// # Arguments
///
/// * `fields` - A reference to a `HashSet` containing `StreamingMarketField` items that need to be serialized.
///
/// # Returns
///
/// A `Vec<String>` where each `String` is a serialized representation of a `StreamingMarketField` from the input set.
///
/// # Panics
///
/// This function will panic if the serialization of any `StreamingMarketField` fails.
///
pub(crate) fn get_streaming_market_fields(fields: &HashSet<StreamingMarketField>) -> Vec<String> {
    let mut fields_vec = Vec::new();
    for field in fields {
        // Serialize to a JSON value and extract the underlying string without quotes
        let val = serde_json::to_value(field).expect("Failed to serialize StreamingMarketField");
        match val {
            serde_json::Value::String(s) => fields_vec.push(s),
            // Fallback: use Debug which yields SCREAMING_SNAKE_CASE variant name
            _ => fields_vec.push(format!("{:?}", field)),
        }
    }
    fields_vec
}

/// Streaming price fields available for price subscriptions.
///
/// These fields represent the various price data points that can be subscribed to
/// in the IG Markets streaming API for price updates.
#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Default, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum StreamingPriceField {
    /// Mid open price
    #[serde(rename = "MID_OPEN")]
    MidOpen,
    /// High price
    High,
    /// Low price
    Low,
    /// Bid quote ID
    BidQuoteId,
    /// Ask quote ID
    AskQuoteId,
    /// Bid price level 1
    BidPrice1,
    /// Bid price level 2
    BidPrice2,
    /// Bid price level 3
    BidPrice3,
    /// Bid price level 4
    BidPrice4,
    /// Bid price level 5
    BidPrice5,
    /// Ask price level 1
    AskPrice1,
    /// Ask price level 2
    AskPrice2,
    /// Ask price level 3
    AskPrice3,
    /// Ask price level 4
    AskPrice4,
    /// Ask price level 5
    #[default]
    AskPrice5,
    /// Bid size level 1
    BidSize1,
    /// Bid size level 2
    BidSize2,
    /// Bid size level 3
    BidSize3,
    /// Bid size level 4
    BidSize4,
    /// Bid size level 5
    BidSize5,
    /// Ask size level 1
    AskSize1,
    /// Ask size level 2
    AskSize2,
    /// Ask size level 3
    AskSize3,
    /// Ask size level 4
    AskSize4,
    /// Ask size level 5
    AskSize5,
    /// Currency 0
    Currency0,
    /// Currency 1
    Currency1,
    /// Currency 1 bid size level 1
    C1BidSize1,
    /// Currency 1 bid size level 2
    C1BidSize2,
    /// Currency 1 bid size level 3
    C1BidSize3,
    /// Currency 1 bid size level 4
    C1BidSize4,
    /// Currency 1 bid size level 5
    C1BidSize5,
    /// Currency 1 ask size level 1
    C1AskSize1,
    /// Currency 1 ask size level 2
    C1AskSize2,
    /// Currency 1 ask size level 3
    C1AskSize3,
    /// Currency 1 ask size level 4
    C1AskSize4,
    /// Currency 1 ask size level 5
    C1AskSize5,
    /// Currency 2
    Currency2,
    /// Currency 2 bid size level 1
    C2BidSize1,
    /// Currency 2 bid size level 2
    C2BidSize2,
    /// Currency 2 bid size level 3
    C2BidSize3,
    /// Currency 2 bid size level 4
    C2BidSize4,
    /// Currency 2 bid size level 5
    C2BidSize5,
    /// Currency 2 ask size level 1
    C2AskSize1,
    /// Currency 2 ask size level 2
    C2AskSize2,
    /// Currency 2 ask size level 3
    C2AskSize3,
    /// Currency 2 ask size level 4
    C2AskSize4,
    /// Currency 2 ask size level 5
    C2AskSize5,
    /// Currency 3
    Currency3,
    /// Currency 3 bid size level 1
    C3BidSize1,
    /// Currency 3 bid size level 2
    C3BidSize2,
    /// Currency 3 bid size level 3
    C3BidSize3,
    /// Currency 3 bid size level 4
    C3BidSize4,
    /// Currency 3 bid size level 5
    C3BidSize5,
    /// Currency 3 ask size level 1
    C3AskSize1,
    /// Currency 3 ask size level 2
    C3AskSize2,
    /// Currency 3 ask size level 3
    C3AskSize3,
    /// Currency 3 ask size level 4
    C3AskSize4,
    /// Currency 3 ask size level 5
    C3AskSize5,
    /// Currency 4
    Currency4,
    /// Currency 4 bid size level 1
    C4BidSize1,
    /// Currency 4 bid size level 2
    C4BidSize2,
    /// Currency 4 bid size level 3
    C4BidSize3,
    /// Currency 4 bid size level 4
    C4BidSize4,
    /// Currency 4 bid size level 5
    C4BidSize5,
    /// Currency 4 ask size level 1
    C4AskSize1,
    /// Currency 4 ask size level 2
    C4AskSize2,
    /// Currency 4 ask size level 3
    C4AskSize3,
    /// Currency 4 ask size level 4
    C4AskSize4,
    /// Currency 4 ask size level 5
    C4AskSize5,
    /// Currency 5
    Currency5,
    /// Currency 5 bid size level 1
    C5BidSize1,
    /// Currency 5 bid size level 2
    C5BidSize2,
    /// Currency 5 bid size level 3
    C5BidSize3,
    /// Currency 5 bid size level 4
    C5BidSize4,
    /// Currency 5 bid size level 5
    C5BidSize5,
    /// Currency 5 ask size level 1
    C5AskSize1,
    /// Currency 5 ask size level 2
    C5AskSize2,
    /// Currency 5 ask size level 3
    C5AskSize3,
    /// Currency 5 ask size level 4
    C5AskSize4,
    /// Currency 5 ask size level 5
    C5AskSize5,
    /// Timestamp of the price update
    Timestamp,
    /// Dealing flag
    #[serde(rename = "DLG_FLAG")]
    DlgFlag,
}

impl Debug for StreamingPriceField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let field_name = serde_json::to_string(self).unwrap();
        write!(f, "{:?}", field_name)
    }
}

impl Display for StreamingPriceField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Constructs a vector of serialized streaming price field names from a given set of `StreamingPriceField`.
///
/// # Arguments
///
/// * `fields` - A reference to a `HashSet` containing `StreamingPriceField` items that need to be serialized.
///
/// # Returns
///
/// A `Vec<String>` where each `String` is a serialized representation of a `StreamingPriceField` from the input set.
///
/// # Panics
///
/// This function will panic if the serialization of any `StreamingPriceField` fails.
///
pub(crate) fn get_streaming_price_fields(fields: &HashSet<StreamingPriceField>) -> Vec<String> {
    // Map each enum variant to the exact IG Lightstreamer field identifier.
    let map_field = |f: &StreamingPriceField| -> &'static str {
        match f {
            // Core prices
            StreamingPriceField::MidOpen => "MID_OPEN",
            StreamingPriceField::High => "HIGH",
            StreamingPriceField::Low => "LOW",
            StreamingPriceField::BidQuoteId => "BIDQUOTEID",
            StreamingPriceField::AskQuoteId => "ASKQUOTEID",

            // Bid ladder prices
            StreamingPriceField::BidPrice1 => "BIDPRICE1",
            StreamingPriceField::BidPrice2 => "BIDPRICE2",
            StreamingPriceField::BidPrice3 => "BIDPRICE3",
            StreamingPriceField::BidPrice4 => "BIDPRICE4",
            StreamingPriceField::BidPrice5 => "BIDPRICE5",

            // Ask ladder prices
            StreamingPriceField::AskPrice1 => "ASKPRICE1",
            StreamingPriceField::AskPrice2 => "ASKPRICE2",
            StreamingPriceField::AskPrice3 => "ASKPRICE3",
            StreamingPriceField::AskPrice4 => "ASKPRICE4",
            StreamingPriceField::AskPrice5 => "ASKPRICE5",

            // Bid sizes
            StreamingPriceField::BidSize1 => "BIDSIZE1",
            StreamingPriceField::BidSize2 => "BIDSIZE2",
            StreamingPriceField::BidSize3 => "BIDSIZE3",
            StreamingPriceField::BidSize4 => "BIDSIZE4",
            StreamingPriceField::BidSize5 => "BIDSIZE5",

            // Ask sizes
            StreamingPriceField::AskSize1 => "ASKSIZE1",
            StreamingPriceField::AskSize2 => "ASKSIZE2",
            StreamingPriceField::AskSize3 => "ASKSIZE3",
            StreamingPriceField::AskSize4 => "ASKSIZE4",
            StreamingPriceField::AskSize5 => "ASKSIZE5",

            // Currencies
            StreamingPriceField::Currency0 => "CURRENCY0",
            StreamingPriceField::Currency1 => "CURRENCY1",
            StreamingPriceField::Currency2 => "CURRENCY2",
            StreamingPriceField::Currency3 => "CURRENCY3",
            StreamingPriceField::Currency4 => "CURRENCY4",
            StreamingPriceField::Currency5 => "CURRENCY5",

            // Currency 1 bid sizes
            StreamingPriceField::C1BidSize1 => "C1BIDSIZE1",
            StreamingPriceField::C1BidSize2 => "C1BIDSIZE2",
            StreamingPriceField::C1BidSize3 => "C1BIDSIZE3",
            StreamingPriceField::C1BidSize4 => "C1BIDSIZE4",
            StreamingPriceField::C1BidSize5 => "C1BIDSIZE5",
            // Currency 1 ask sizes
            StreamingPriceField::C1AskSize1 => "C1ASKSIZE1",
            StreamingPriceField::C1AskSize2 => "C1ASKSIZE2",
            StreamingPriceField::C1AskSize3 => "C1ASKSIZE3",
            StreamingPriceField::C1AskSize4 => "C1ASKSIZE4",
            StreamingPriceField::C1AskSize5 => "C1ASKSIZE5",

            // Currency 2 bid sizes
            StreamingPriceField::C2BidSize1 => "C2BIDSIZE1",
            StreamingPriceField::C2BidSize2 => "C2BIDSIZE2",
            StreamingPriceField::C2BidSize3 => "C2BIDSIZE3",
            StreamingPriceField::C2BidSize4 => "C2BIDSIZE4",
            StreamingPriceField::C2BidSize5 => "C2BIDSIZE5",
            // Currency 2 ask sizes
            StreamingPriceField::C2AskSize1 => "C2ASKSIZE1",
            StreamingPriceField::C2AskSize2 => "C2ASKSIZE2",
            StreamingPriceField::C2AskSize3 => "C2ASKSIZE3",
            StreamingPriceField::C2AskSize4 => "C2ASKSIZE4",
            StreamingPriceField::C2AskSize5 => "C2ASKSIZE5",

            // Currency 3 bid sizes
            StreamingPriceField::C3BidSize1 => "C3BIDSIZE1",
            StreamingPriceField::C3BidSize2 => "C3BIDSIZE2",
            StreamingPriceField::C3BidSize3 => "C3BIDSIZE3",
            StreamingPriceField::C3BidSize4 => "C3BIDSIZE4",
            StreamingPriceField::C3BidSize5 => "C3BIDSIZE5",
            // Currency 3 ask sizes
            StreamingPriceField::C3AskSize1 => "C3ASKSIZE1",
            StreamingPriceField::C3AskSize2 => "C3ASKSIZE2",
            StreamingPriceField::C3AskSize3 => "C3ASKSIZE3",
            StreamingPriceField::C3AskSize4 => "C3ASKSIZE4",
            StreamingPriceField::C3AskSize5 => "C3ASKSIZE5",

            // Currency 4 bid sizes
            StreamingPriceField::C4BidSize1 => "C4BIDSIZE1",
            StreamingPriceField::C4BidSize2 => "C4BIDSIZE2",
            StreamingPriceField::C4BidSize3 => "C4BIDSIZE3",
            StreamingPriceField::C4BidSize4 => "C4BIDSIZE4",
            StreamingPriceField::C4BidSize5 => "C4BIDSIZE5",
            // Currency 4 ask sizes
            StreamingPriceField::C4AskSize1 => "C4ASKSIZE1",
            StreamingPriceField::C4AskSize2 => "C4ASKSIZE2",
            StreamingPriceField::C4AskSize3 => "C4ASKSIZE3",
            StreamingPriceField::C4AskSize4 => "C4ASKSIZE4",
            StreamingPriceField::C4AskSize5 => "C4ASKSIZE5",

            // Currency 5 bid sizes
            StreamingPriceField::C5BidSize1 => "C5BIDSIZE1",
            StreamingPriceField::C5BidSize2 => "C5BIDSIZE2",
            StreamingPriceField::C5BidSize3 => "C5BIDSIZE3",
            StreamingPriceField::C5BidSize4 => "C5BIDSIZE4",
            StreamingPriceField::C5BidSize5 => "C5BIDSIZE5",
            // Currency 5 ask sizes
            StreamingPriceField::C5AskSize1 => "C5ASKSIZE1",
            StreamingPriceField::C5AskSize2 => "C5ASKSIZE2",
            StreamingPriceField::C5AskSize3 => "C5ASKSIZE3",
            StreamingPriceField::C5AskSize4 => "C5ASKSIZE4",
            StreamingPriceField::C5AskSize5 => "C5ASKSIZE5",

            // Misc
            StreamingPriceField::Timestamp => "TIMESTAMP",
            StreamingPriceField::DlgFlag => "DLG_FLAG",
        }
    };

    let mut fields_vec = Vec::with_capacity(fields.len());
    for field in fields {
        fields_vec.push(map_field(field).to_string());
    }
    fields_vec
}

/// Streaming account data fields available for account subscriptions.
///
/// These fields represent the various account data points that can be subscribed to
/// in the IG Markets streaming API for account updates.
#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Default, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamingAccountDataField {
    /// Profit and loss
    #[default]
    Pnl,
    /// Deposit amount
    Deposit,
    /// Available cash
    AvailableCash,
    /// Profit and loss for long positions with guaranteed stops
    PnlLr,
    /// Profit and loss for long positions without guaranteed stops
    PnlNlr,
    /// Total funds
    Funds,
    /// Total margin
    Margin,
    /// Margin for positions with guaranteed stops
    MarginLr,
    /// Margin for positions without guaranteed stops
    MarginNlr,
    /// Available amount to deal
    AvailableToDeal,
    /// Total equity
    Equity,
    /// Equity used
    EquityUsed,
}

impl Debug for StreamingAccountDataField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let field_name = serde_json::to_string(self).unwrap();
        write!(f, "{:?}", field_name)
    }
}

impl Display for StreamingAccountDataField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Constructs a vector of serialized streaming account data field names from a given set of `StreamingAccountDataField`.
///
/// # Arguments
///
/// * `fields` - A reference to a `HashSet` containing `StreamingAccountDataField` items that need to be serialized.
///
/// # Returns
///
/// A `Vec<String>` where each `String` is a serialized representation of a `StreamingAccountDataField` from the input set.
///
/// # Panics
///
/// This function will panic if the serialization of any `StreamingAccountDataField` fails.
///
pub(crate) fn get_streaming_account_data_fields(
    fields: &HashSet<StreamingAccountDataField>,
) -> Vec<String> {
    let mut fields_vec = Vec::new();
    for field in fields {
        let val =
            serde_json::to_value(field).expect("Failed to serialize StreamingAccountDataField");
        match val {
            serde_json::Value::String(s) => fields_vec.push(s),
            _ => fields_vec.push(format!("{:?}", field)),
        }
    }
    fields_vec
}
