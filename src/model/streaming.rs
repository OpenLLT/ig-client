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
        fields_vec.push(serde_json::to_string(field).unwrap());
    }
    fields_vec
}

/// Streaming price fields available for price subscriptions.
///
/// These fields represent the various price data points that can be subscribed to
/// in the IG Markets streaming API for price updates.
#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Default, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamingPriceField {
    /// Mid open price
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
    let mut fields_vec = Vec::new();
    for field in fields {
        fields_vec.push(serde_json::to_string(field).unwrap());
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
        fields_vec.push(serde_json::to_string(field).unwrap());
    }
    fields_vec
}
