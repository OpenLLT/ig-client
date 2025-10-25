/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 25/10/25
 ******************************************************************************/
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use crate::prelude::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamingMarketField {
    MidOpen,
    High,
    Low,
    Change,
    ChangePct,
    UpdateTime,
    MarketDelay,
    MarketState,
    Bid,
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