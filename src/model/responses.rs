/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 19/10/25
******************************************************************************/
use crate::prelude::{Account, Activity, MarketDetails};
use crate::presentation::account::{
    AccountTransaction, ActivityMetadata, Position, TransactionMetadata, WorkingOrder,
};
use crate::presentation::instrument::InstrumentType;
use crate::presentation::market::{
    HistoricalPrice, MarketData, MarketNavigationNode, MarketNode, PriceAllowance,
};
use crate::presentation::order::{Direction, Status};
use crate::utils::parsing::{deserialize_null_as_empty_vec, deserialize_nullable_status};
use chrono::{DateTime, Utc};
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Database entry response for market instruments
#[derive(
    DebugPretty, DisplaySimple, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default,
)]
pub struct DBEntryResponse {
    /// The trading symbol identifier
    pub symbol: String,
    /// The Epic identifier used by the exchange
    pub epic: String,
    /// Human-readable name of the instrument
    pub name: String,
    /// Instrument type classification
    pub instrument_type: InstrumentType,
    /// The exchange where this instrument is traded
    pub exchange: String,
    /// Expiration date and time for the instrument
    pub expiry: String,
    /// Timestamp of the last update to this record
    pub last_update: DateTime<Utc>,
}

impl From<MarketNode> for DBEntryResponse {
    fn from(value: MarketNode) -> Self {
        let mut entry = DBEntryResponse::default();
        if !value.markets.is_empty() {
            let market = &value.markets[0];
            entry.symbol = market
                .epic
                .split('.')
                .nth(2)
                .unwrap_or_default()
                .to_string();
            entry.epic = market.epic.clone();
            entry.name = market.instrument_name.clone();
            entry.instrument_type = market.instrument_type;
            entry.exchange = "IG".to_string();
            entry.expiry = market.expiry.clone();
            entry.last_update = Utc::now();
        }
        entry
    }
}

impl From<MarketData> for DBEntryResponse {
    fn from(market: MarketData) -> Self {
        DBEntryResponse {
            symbol: market
                .epic
                .split('.')
                .nth(2)
                .unwrap_or_default()
                .to_string(),
            epic: market.epic.clone(),
            name: market.instrument_name.clone(),
            instrument_type: market.instrument_type,
            exchange: "IG".to_string(),
            expiry: market.expiry.clone(),
            last_update: Utc::now(),
        }
    }
}

impl From<&MarketNode> for DBEntryResponse {
    fn from(value: &MarketNode) -> Self {
        DBEntryResponse::from(value.clone())
    }
}

impl From<&MarketData> for DBEntryResponse {
    fn from(market: &MarketData) -> Self {
        DBEntryResponse::from(market.clone())
    }
}

/// Response containing multiple market details
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize, Default)]
pub struct MultipleMarketDetailsResponse {
    /// List of market details
    #[serde(rename = "marketDetails")]
    pub market_details: Vec<MarketDetails>,
}

impl MultipleMarketDetailsResponse {
    /// Returns the number of market details in the response
    ///
    /// # Returns
    /// Number of market details
    #[must_use]
    pub fn len(&self) -> usize {
        self.market_details.len()
    }

    /// Returns true if the response contains no market details
    ///
    /// # Returns
    /// True if empty, false otherwise
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.market_details.is_empty()
    }

    /// Returns a reference to the market details vector
    ///
    /// # Returns
    /// Reference to the vector of market details
    #[must_use]
    pub fn market_details(&self) -> &Vec<MarketDetails> {
        &self.market_details
    }

    /// Returns an iterator over the market details
    ///
    /// # Returns
    /// Iterator over market details
    pub fn iter(&self) -> impl Iterator<Item = &MarketDetails> {
        self.market_details.iter()
    }
}

/// Model for historical prices
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct HistoricalPricesResponse {
    /// List of historical price points
    pub prices: Vec<HistoricalPrice>,
    /// Type of the instrument
    #[serde(rename = "instrumentType")]
    pub instrument_type: InstrumentType,
    /// API usage allowance information
    #[serde(rename = "allowance", skip_serializing_if = "Option::is_none", default)]
    pub allowance: Option<PriceAllowance>,
}

impl HistoricalPricesResponse {
    /// Returns the number of price points in the response
    ///
    /// # Returns
    /// Number of price points
    #[must_use]
    pub fn len(&self) -> usize {
        self.prices.len()
    }

    /// Returns true if the response contains no price points
    ///
    /// # Returns
    /// True if empty, false otherwise
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.prices.is_empty()
    }

    /// Returns a reference to the prices vector
    ///
    /// # Returns
    /// Reference to the vector of historical prices
    #[must_use]
    pub fn prices(&self) -> &Vec<HistoricalPrice> {
        &self.prices
    }

    /// Returns an iterator over the prices
    ///
    /// # Returns
    /// Iterator over historical prices
    pub fn iter(&self) -> impl Iterator<Item = &HistoricalPrice> {
        self.prices.iter()
    }
}

/// Model for market search results
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct MarketSearchResponse {
    /// List of markets matching the search criteria
    pub markets: Vec<MarketData>,
}

impl MarketSearchResponse {
    /// Returns the number of markets in the response
    ///
    /// # Returns
    /// Number of markets
    #[must_use]
    pub fn len(&self) -> usize {
        self.markets.len()
    }

    /// Returns true if the response contains no markets
    ///
    /// # Returns
    /// True if empty, false otherwise
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.markets.is_empty()
    }

    /// Returns a reference to the markets vector
    ///
    /// # Returns
    /// Reference to the vector of markets
    #[must_use]
    pub fn markets(&self) -> &Vec<MarketData> {
        &self.markets
    }

    /// Returns an iterator over the markets
    ///
    /// # Returns
    /// Iterator over markets
    pub fn iter(&self) -> impl Iterator<Item = &MarketData> {
        self.markets.iter()
    }
}

/// Response model for market navigation
#[derive(DebugPretty, DisplaySimple, Clone, Deserialize, Serialize)]
pub struct MarketNavigationResponse {
    /// List of navigation nodes at the current level
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub nodes: Vec<MarketNavigationNode>,
    /// List of markets at the current level
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub markets: Vec<MarketData>,
}

/// Response containing user accounts
#[derive(DebugPretty, DisplaySimple, Clone, Deserialize, Serialize, Default)]
pub struct AccountsResponse {
    /// List of accounts owned by the user
    pub accounts: Vec<Account>,
}

/// Open positions
#[derive(DebugPretty, DisplaySimple, Clone, Deserialize, Serialize, Default)]
pub struct PositionsResponse {
    /// List of open positions
    pub positions: Vec<Position>,
}

impl PositionsResponse {
    /// Compact positions by epic, combining positions with the same epic
    ///
    /// This method takes a vector of positions and returns a new vector where
    /// positions with the same epic have been combined into a single position.
    ///
    /// # Arguments
    /// * `positions` - A vector of positions to compact
    ///
    /// # Returns
    /// A vector of positions with unique epics
    pub fn compact_by_epic(positions: Vec<Position>) -> Vec<Position> {
        let mut epic_map: HashMap<String, Position> = std::collections::HashMap::new();

        for position in positions {
            let epic = position.market.epic.clone();
            epic_map
                .entry(epic)
                .and_modify(|existing| {
                    *existing = existing.clone() + position.clone();
                })
                .or_insert(position);
        }

        epic_map.into_values().collect()
    }
}

/// Working orders
#[derive(DebugPretty, DisplaySimple, Clone, Deserialize, Serialize)]
pub struct WorkingOrdersResponse {
    /// List of pending working orders
    #[serde(rename = "workingOrders")]
    pub working_orders: Vec<WorkingOrder>,
}

/// Account activity
#[derive(DebugPretty, DisplaySimple, Clone, Deserialize, Serialize)]
pub struct AccountActivityResponse {
    /// List of activities on the account
    pub activities: Vec<Activity>,
    /// Metadata about pagination
    pub metadata: Option<ActivityMetadata>,
}

/// Transaction history
#[derive(DebugPretty, DisplaySimple, Clone, Deserialize, Serialize)]
pub struct TransactionHistoryResponse {
    /// List of account transactions
    pub transactions: Vec<AccountTransaction>,
    /// Metadata about the transaction list
    pub metadata: TransactionMetadata,
}

/// Response to order creation
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    /// Client-generated reference for the deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
}

/// Response to closing a position
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct ClosePositionResponse {
    /// Client-generated reference for the closing deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
}

/// Response to updating a position
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct UpdatePositionResponse {
    /// Client-generated reference for the update deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
}

/// Response to working order creation
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct CreateWorkingOrderResponse {
    /// Client-generated reference for the deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
}

/// Details of a confirmed order
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct OrderConfirmationResponse {
    /// Date and time of the confirmation
    pub date: String,
    /// Status of the order (accepted, rejected, etc.)
    /// This can be null in some responses (e.g., when market is closed)
    #[serde(deserialize_with = "deserialize_nullable_status")]
    pub status: Status,
    /// Reason for rejection if applicable
    pub reason: Option<String>,
    /// Unique identifier for the deal
    #[serde(rename = "dealId")]
    pub deal_id: Option<String>,
    /// Client-generated reference for the deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
    /// Status of the deal
    #[serde(rename = "dealStatus")]
    pub deal_status: Option<String>,
    /// Instrument EPIC identifier
    pub epic: Option<String>,
    /// Expiry date for the order
    #[serde(rename = "expiry")]
    pub expiry: Option<String>,
    /// Whether a guaranteed stop was used
    #[serde(rename = "guaranteedStop")]
    pub guaranteed_stop: Option<bool>,
    /// Price level of the order
    #[serde(rename = "level")]
    pub level: Option<f64>,
    /// Distance for take profit
    #[serde(rename = "limitDistance")]
    pub limit_distance: Option<f64>,
    /// Price level for take profit
    #[serde(rename = "limitLevel")]
    pub limit_level: Option<f64>,
    /// Size/quantity of the order
    pub size: Option<f64>,
    /// Distance for stop loss
    #[serde(rename = "stopDistance")]
    pub stop_distance: Option<f64>,
    /// Price level for stop loss
    #[serde(rename = "stopLevel")]
    pub stop_level: Option<f64>,
    /// Whether a trailing stop was used
    #[serde(rename = "trailingStop")]
    pub trailing_stop: Option<bool>,
    /// Direction of the order (buy or sell)
    pub direction: Option<Direction>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::fs;

    #[test]
    fn test_deserialize_working_orders_from_file() {
        // Load the JSON file
        let json_content = fs::read_to_string("Data/working_orders.json")
            .expect("Failed to read Data/working_orders.json");

        // Parse as a generic JSON Value first to inspect the structure
        let json_value: Value =
            serde_json::from_str(&json_content).expect("Failed to parse JSON as Value");

        println!(
            "JSON structure:\n{}",
            serde_json::to_string_pretty(&json_value).unwrap()
        );

        // Attempt to deserialize into WorkingOrdersResponse
        let result: Result<WorkingOrdersResponse, _> = serde_json::from_str(&json_content);

        match result {
            Ok(response) => {
                println!(
                    "Successfully deserialized {} working orders",
                    response.working_orders.len()
                );
                for (idx, order) in response.working_orders.iter().enumerate() {
                    println!(
                        "Order {}: epic={}, direction={:?}, size={}, level={}",
                        idx + 1,
                        order.working_order_data.epic,
                        order.working_order_data.direction,
                        order.working_order_data.order_size,
                        order.working_order_data.order_level
                    );
                }
            }
            Err(e) => {
                panic!(
                    "Failed to deserialize WorkingOrdersResponse: {}\n\nJSON was:\n{}",
                    e,
                    serde_json::to_string_pretty(&json_value).unwrap()
                );
            }
        }
    }
}
