/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 25/10/25
******************************************************************************/

//! Tests for streaming model enums, specifically focusing on Display and Debug implementations.

use ig_client::model::streaming::{
    StreamingAccountDataField, StreamingMarketField, StreamingPriceField,
};
use std::collections::HashSet;

#[test]
fn test_streaming_market_field_debug_format() {
    // Test Debug implementation for each variant
    let mid_open = StreamingMarketField::MidOpen;
    let high = StreamingMarketField::High;
    let low = StreamingMarketField::Low;
    let change = StreamingMarketField::Change;
    let change_pct = StreamingMarketField::ChangePct;
    let update_time = StreamingMarketField::UpdateTime;
    let market_delay = StreamingMarketField::MarketDelay;
    let market_state = StreamingMarketField::MarketState;
    let bid = StreamingMarketField::Bid;
    let offer = StreamingMarketField::Offer;

    // Verify Debug format shows SCREAMING_SNAKE_CASE without quotes
    assert_eq!(format!("{:?}", mid_open), "MID_OPEN");
    assert_eq!(format!("{:?}", high), "HIGH");
    assert_eq!(format!("{:?}", low), "LOW");
    assert_eq!(format!("{:?}", change), "CHANGE");
    assert_eq!(format!("{:?}", change_pct), "CHANGE_PCT");
    assert_eq!(format!("{:?}", update_time), "UPDATE_TIME");
    assert_eq!(format!("{:?}", market_delay), "MARKET_DELAY");
    assert_eq!(format!("{:?}", market_state), "MARKET_STATE");
    assert_eq!(format!("{:?}", bid), "BID");
    assert_eq!(format!("{:?}", offer), "OFFER");
}

#[test]
fn test_streaming_market_field_display_format() {
    // Test Display implementation for each variant
    let mid_open = StreamingMarketField::MidOpen;
    let high = StreamingMarketField::High;
    let low = StreamingMarketField::Low;
    let change = StreamingMarketField::Change;
    let change_pct = StreamingMarketField::ChangePct;
    let update_time = StreamingMarketField::UpdateTime;
    let market_delay = StreamingMarketField::MarketDelay;
    let market_state = StreamingMarketField::MarketState;
    let bid = StreamingMarketField::Bid;
    let offer = StreamingMarketField::Offer;

    // Verify Display format matches Debug format (as per implementation)
    assert_eq!(format!("{}", mid_open), "MID_OPEN");
    assert_eq!(format!("{}", high), "HIGH");
    assert_eq!(format!("{}", low), "LOW");
    assert_eq!(format!("{}", change), "CHANGE");
    assert_eq!(format!("{}", change_pct), "CHANGE_PCT");
    assert_eq!(format!("{}", update_time), "UPDATE_TIME");
    assert_eq!(format!("{}", market_delay), "MARKET_DELAY");
    assert_eq!(format!("{}", market_state), "MARKET_STATE");
    assert_eq!(format!("{}", bid), "BID");
    assert_eq!(format!("{}", offer), "OFFER");
}

#[test]
fn test_streaming_market_field_debug_display_consistency() {
    // Verify that Debug and Display produce the same output for all variants
    let all_variants = vec![
        StreamingMarketField::MidOpen,
        StreamingMarketField::High,
        StreamingMarketField::Low,
        StreamingMarketField::Change,
        StreamingMarketField::ChangePct,
        StreamingMarketField::UpdateTime,
        StreamingMarketField::MarketDelay,
        StreamingMarketField::MarketState,
        StreamingMarketField::Bid,
        StreamingMarketField::Offer,
    ];

    for variant in all_variants {
        let debug_output = format!("{:?}", variant);
        let display_output = format!("{}", variant);
        assert_eq!(
            debug_output, display_output,
            "Debug and Display should produce the same output for {:?}",
            variant
        );
    }
}

#[cfg(test)]
mod streaming_account_data_field_tests {
    use super::*;

    #[test]
    fn test_streaming_account_data_field_debug_format() {
        // Test Debug format for various StreamingAccountDataField variants
        let pnl = StreamingAccountDataField::Pnl;
        let deposit = StreamingAccountDataField::Deposit;
        let available_cash = StreamingAccountDataField::AvailableCash;
        let pnl_lr = StreamingAccountDataField::PnlLr;
        let pnl_nlr = StreamingAccountDataField::PnlNlr;
        let funds = StreamingAccountDataField::Funds;
        let margin = StreamingAccountDataField::Margin;
        let margin_lr = StreamingAccountDataField::MarginLr;
        let margin_nlr = StreamingAccountDataField::MarginNlr;
        let available_to_deal = StreamingAccountDataField::AvailableToDeal;
        let equity = StreamingAccountDataField::Equity;
        let equity_used = StreamingAccountDataField::EquityUsed;

        // Verify Debug format shows SCREAMING_SNAKE_CASE without quotes
        assert_eq!(format!("{:?}", pnl), "PNL");
        assert_eq!(format!("{:?}", deposit), "DEPOSIT");
        assert_eq!(format!("{:?}", available_cash), "AVAILABLE_CASH");
        assert_eq!(format!("{:?}", pnl_lr), "PNL_LR");
        assert_eq!(format!("{:?}", pnl_nlr), "PNL_NLR");
        assert_eq!(format!("{:?}", funds), "FUNDS");
        assert_eq!(format!("{:?}", margin), "MARGIN");
        assert_eq!(format!("{:?}", margin_lr), "MARGIN_LR");
        assert_eq!(format!("{:?}", margin_nlr), "MARGIN_NLR");
        assert_eq!(format!("{:?}", available_to_deal), "AVAILABLE_TO_DEAL");
        assert_eq!(format!("{:?}", equity), "EQUITY");
        assert_eq!(format!("{:?}", equity_used), "EQUITY_USED");
    }

    #[test]
    fn test_streaming_account_data_field_display_format() {
        // Test Display format for various StreamingAccountDataField variants
        let pnl = StreamingAccountDataField::Pnl;
        let deposit = StreamingAccountDataField::Deposit;
        let available_cash = StreamingAccountDataField::AvailableCash;
        let pnl_lr = StreamingAccountDataField::PnlLr;
        let pnl_nlr = StreamingAccountDataField::PnlNlr;
        let funds = StreamingAccountDataField::Funds;
        let margin = StreamingAccountDataField::Margin;
        let margin_lr = StreamingAccountDataField::MarginLr;
        let margin_nlr = StreamingAccountDataField::MarginNlr;
        let available_to_deal = StreamingAccountDataField::AvailableToDeal;
        let equity = StreamingAccountDataField::Equity;
        let equity_used = StreamingAccountDataField::EquityUsed;

        // Verify Display format matches Debug format (as per implementation)
        assert_eq!(format!("{}", pnl), "PNL");
        assert_eq!(format!("{}", deposit), "DEPOSIT");
        assert_eq!(format!("{}", available_cash), "AVAILABLE_CASH");
        assert_eq!(format!("{}", pnl_lr), "PNL_LR");
        assert_eq!(format!("{}", pnl_nlr), "PNL_NLR");
        assert_eq!(format!("{}", funds), "FUNDS");
        assert_eq!(format!("{}", margin), "MARGIN");
        assert_eq!(format!("{}", margin_lr), "MARGIN_LR");
        assert_eq!(format!("{}", margin_nlr), "MARGIN_NLR");
        assert_eq!(format!("{}", available_to_deal), "AVAILABLE_TO_DEAL");
        assert_eq!(format!("{}", equity), "EQUITY");
        assert_eq!(format!("{}", equity_used), "EQUITY_USED");
    }

    #[test]
    fn test_streaming_account_data_field_debug_display_consistency() {
        // Test that Debug and Display formats are consistent
        let test_fields = vec![
            StreamingAccountDataField::Pnl,
            StreamingAccountDataField::Deposit,
            StreamingAccountDataField::AvailableCash,
            StreamingAccountDataField::PnlLr,
            StreamingAccountDataField::PnlNlr,
            StreamingAccountDataField::Funds,
            StreamingAccountDataField::Margin,
            StreamingAccountDataField::MarginLr,
            StreamingAccountDataField::MarginNlr,
            StreamingAccountDataField::AvailableToDeal,
            StreamingAccountDataField::Equity,
            StreamingAccountDataField::EquityUsed,
        ];

        for field in test_fields {
            let debug_output = format!("{:?}", field);
            let display_output = format!("{}", field);
            assert_eq!(
                debug_output, display_output,
                "Debug and Display should be consistent for {:?}",
                field
            );
        }
    }

    #[test]
    fn test_streaming_account_data_field_default() {
        // Test that the default variant is Pnl
        let default_field = StreamingAccountDataField::default();
        assert_eq!(default_field, StreamingAccountDataField::Pnl);
        assert_eq!(format!("{:?}", default_field), "PNL");
        assert_eq!(format!("{}", default_field), "PNL");
    }

    #[test]
    fn test_streaming_account_data_field_serialization_format() {
        // Test serialization format for various fields
        let test_cases = vec![
            (StreamingAccountDataField::Pnl, "PNL"),
            (StreamingAccountDataField::Deposit, "DEPOSIT"),
            (StreamingAccountDataField::AvailableCash, "AVAILABLE_CASH"),
            (StreamingAccountDataField::PnlLr, "PNL_LR"),
            (StreamingAccountDataField::PnlNlr, "PNL_NLR"),
            (StreamingAccountDataField::Funds, "FUNDS"),
            (StreamingAccountDataField::Margin, "MARGIN"),
            (StreamingAccountDataField::MarginLr, "MARGIN_LR"),
            (StreamingAccountDataField::MarginNlr, "MARGIN_NLR"),
            (
                StreamingAccountDataField::AvailableToDeal,
                "AVAILABLE_TO_DEAL",
            ),
            (StreamingAccountDataField::Equity, "EQUITY"),
            (StreamingAccountDataField::EquityUsed, "EQUITY_USED"),
        ];

        for (field, expected_serialized) in test_cases {
            // Test serialization
            let serialized = serde_json::to_string(&field).unwrap();
            assert_eq!(serialized, format!("\"{}\"", expected_serialized));

            // Verify that Debug output matches the serialized string (without quotes)
            let debug_output = format!("{:?}", field);
            assert_eq!(debug_output, expected_serialized);
        }
    }

    #[test]
    fn test_streaming_account_data_field_clone_and_equality() {
        // Test Clone and PartialEq implementations
        let original = StreamingAccountDataField::Pnl;
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(format!("{:?}", original), format!("{:?}", cloned));
    }

    #[cfg(test)]
    mod streaming_price_field_tests {
        use super::*;

        #[test]
        fn test_streaming_price_field_debug_format() {
            // Test Debug format for various StreamingPriceField variants
            let mid_open = StreamingPriceField::MidOpen;
            let high = StreamingPriceField::High;
            let low = StreamingPriceField::Low;
            let bid_quote_id = StreamingPriceField::BidQuoteId;
            let ask_quote_id = StreamingPriceField::AskQuoteId;
            let bid_price1 = StreamingPriceField::BidPrice1;
            let ask_price5 = StreamingPriceField::AskPrice5;
            let timestamp = StreamingPriceField::Timestamp;
            let dlg_flag = StreamingPriceField::DlgFlag;

            // Verify Debug format shows UPPERCASE format (no underscores) except for MID_OPEN and DLG_FLAG
            assert_eq!(format!("{:?}", mid_open), "MID_OPEN");
            assert_eq!(format!("{:?}", high), "HIGH");
            assert_eq!(format!("{:?}", low), "LOW");
            assert_eq!(format!("{:?}", bid_quote_id), "BIDQUOTEID");
            assert_eq!(format!("{:?}", ask_quote_id), "ASKQUOTEID");
            assert_eq!(format!("{:?}", bid_price1), "BIDPRICE1");
            assert_eq!(format!("{:?}", ask_price5), "ASKPRICE5");
            assert_eq!(format!("{:?}", timestamp), "TIMESTAMP");
            assert_eq!(format!("{:?}", dlg_flag), "DLG_FLAG");
        }

        #[test]
        fn test_streaming_price_field_display_format() {
            // Test Display format for various StreamingPriceField variants
            let mid_open = StreamingPriceField::MidOpen;
            let high = StreamingPriceField::High;
            let low = StreamingPriceField::Low;
            let bid_quote_id = StreamingPriceField::BidQuoteId;
            let ask_quote_id = StreamingPriceField::AskQuoteId;
            let bid_price1 = StreamingPriceField::BidPrice1;
            let ask_price5 = StreamingPriceField::AskPrice5;
            let timestamp = StreamingPriceField::Timestamp;
            let dlg_flag = StreamingPriceField::DlgFlag;

            // Verify Display format matches Debug format (as per implementation)
            assert_eq!(format!("{}", mid_open), "MID_OPEN");
            assert_eq!(format!("{}", high), "HIGH");
            assert_eq!(format!("{}", low), "LOW");
            assert_eq!(format!("{}", bid_quote_id), "BIDQUOTEID");
            assert_eq!(format!("{}", ask_quote_id), "ASKQUOTEID");
            assert_eq!(format!("{}", bid_price1), "BIDPRICE1");
            assert_eq!(format!("{}", ask_price5), "ASKPRICE5");
            assert_eq!(format!("{}", timestamp), "TIMESTAMP");
            assert_eq!(format!("{}", dlg_flag), "DLG_FLAG");
        }

        #[test]
        fn test_streaming_price_field_debug_display_consistency() {
            // Test that Debug and Display formats are consistent for a sample of fields
            let test_fields = vec![
                StreamingPriceField::MidOpen,
                StreamingPriceField::High,
                StreamingPriceField::Low,
                StreamingPriceField::BidQuoteId,
                StreamingPriceField::AskQuoteId,
                StreamingPriceField::BidPrice1,
                StreamingPriceField::AskPrice5,
                StreamingPriceField::Currency0,
                StreamingPriceField::C1BidSize1,
                StreamingPriceField::C5AskSize5,
                StreamingPriceField::Timestamp,
                StreamingPriceField::DlgFlag,
            ];

            for field in test_fields {
                let debug_output = format!("{:?}", field);
                let display_output = format!("{}", field);
                assert_eq!(
                    debug_output, display_output,
                    "Debug and Display should be consistent for {:?}",
                    field
                );
            }
        }

        #[test]
        fn test_streaming_price_field_default() {
            // Test that the default variant is AskPrice5
            let default_field = StreamingPriceField::default();
            assert_eq!(default_field, StreamingPriceField::AskPrice5);
            assert_eq!(format!("{:?}", default_field), "ASKPRICE5");
            assert_eq!(format!("{}", default_field), "ASKPRICE5");
        }

        #[test]
        fn test_streaming_price_field_serialization_format() {
            // Test serialization format for various fields
            // Note: StreamingPriceField uses #[serde(rename_all = "UPPERCASE")]
            // which means most fields are serialized as UPPERCASE without underscores
            // except for those with explicit #[serde(rename = "...")] attributes
            let test_cases = vec![
                (StreamingPriceField::MidOpen, "MID_OPEN"), // Has explicit rename
                (StreamingPriceField::High, "HIGH"),
                (StreamingPriceField::Low, "LOW"),
                (StreamingPriceField::BidQuoteId, "BIDQUOTEID"), // UPPERCASE format
                (StreamingPriceField::AskQuoteId, "ASKQUOTEID"), // UPPERCASE format
                (StreamingPriceField::BidPrice1, "BIDPRICE1"),   // UPPERCASE format
                (StreamingPriceField::AskPrice5, "ASKPRICE5"),   // UPPERCASE format
                (StreamingPriceField::Currency0, "CURRENCY0"),
                (StreamingPriceField::C1BidSize1, "C1BIDSIZE1"), // UPPERCASE format
                (StreamingPriceField::C5AskSize5, "C5ASKSIZE5"), // UPPERCASE format
                (StreamingPriceField::Timestamp, "TIMESTAMP"),
                (StreamingPriceField::DlgFlag, "DLG_FLAG"), // Has explicit rename
            ];

            for (field, expected_serialized) in test_cases {
                // Test serialization
                let serialized = serde_json::to_string(&field).unwrap();
                assert_eq!(serialized, format!("\"{}\"", expected_serialized));

                // Verify that Debug output matches the serialized string (without quotes)
                let debug_output = format!("{:?}", field);
                assert_eq!(debug_output, expected_serialized);
            }
        }

        #[test]
        fn test_streaming_price_field_clone_and_equality() {
            // Test Clone and PartialEq implementations
            let original = StreamingPriceField::MidOpen;
            let cloned = original.clone();

            assert_eq!(original, cloned);
            assert_eq!(format!("{:?}", original), format!("{:?}", cloned));
        }

        #[test]
        fn test_streaming_price_field_in_hashset() {
            // Test that the enum can be used in a HashSet (Hash implementation)
            let mut set = HashSet::new();
            set.insert(StreamingPriceField::MidOpen);
            set.insert(StreamingPriceField::High);
            set.insert(StreamingPriceField::MidOpen); // Duplicate should not increase size

            assert_eq!(set.len(), 2);
            assert!(set.contains(&StreamingPriceField::MidOpen));
            assert!(set.contains(&StreamingPriceField::High));
            assert!(!set.contains(&StreamingPriceField::Low));
        }
    }

    #[test]
    fn test_streaming_account_data_field_in_hashset() {
        // Test that the enum can be used in a HashSet (Hash implementation)
        let mut set = HashSet::new();
        set.insert(StreamingAccountDataField::Pnl);
        set.insert(StreamingAccountDataField::Deposit);
        set.insert(StreamingAccountDataField::Pnl); // Duplicate should not increase size

        assert_eq!(set.len(), 2);
        assert!(set.contains(&StreamingAccountDataField::Pnl));
        assert!(set.contains(&StreamingAccountDataField::Deposit));
        assert!(!set.contains(&StreamingAccountDataField::AvailableCash));
    }
}

#[test]
fn test_streaming_market_field_default() {
    // Test that the default variant is Offer
    let default_field = StreamingMarketField::default();
    assert_eq!(default_field, StreamingMarketField::Offer);
    assert_eq!(format!("{:?}", default_field), "OFFER");
    assert_eq!(format!("{}", default_field), "OFFER");
}

#[test]
fn test_streaming_market_field_serialization_format() {
    // Test that the serialized format matches expected SCREAMING_SNAKE_CASE
    let fields = vec![
        (StreamingMarketField::MidOpen, "MID_OPEN"),
        (StreamingMarketField::High, "HIGH"),
        (StreamingMarketField::Low, "LOW"),
        (StreamingMarketField::Change, "CHANGE"),
        (StreamingMarketField::ChangePct, "CHANGE_PCT"),
        (StreamingMarketField::UpdateTime, "UPDATE_TIME"),
        (StreamingMarketField::MarketDelay, "MARKET_DELAY"),
        (StreamingMarketField::MarketState, "MARKET_STATE"),
        (StreamingMarketField::Bid, "BID"),
        (StreamingMarketField::Offer, "OFFER"),
    ];

    for (field, expected_serialized) in fields {
        let serialized = serde_json::to_string(&field).unwrap();
        assert_eq!(serialized, format!("\"{}\"", expected_serialized));

        // Verify that Debug output matches the serialized string (without quotes)
        let debug_output = format!("{:?}", field);
        assert_eq!(debug_output, expected_serialized);
    }
}

#[test]
fn test_streaming_market_field_in_hashset() {
    // Test that fields work correctly in HashSet (used in get_streaming_market_fields)
    let mut fields = HashSet::new();
    fields.insert(StreamingMarketField::Bid);
    fields.insert(StreamingMarketField::Offer);
    fields.insert(StreamingMarketField::High);
    fields.insert(StreamingMarketField::Low);

    assert_eq!(fields.len(), 4);
    assert!(fields.contains(&StreamingMarketField::Bid));
    assert!(fields.contains(&StreamingMarketField::Offer));
    assert!(fields.contains(&StreamingMarketField::High));
    assert!(fields.contains(&StreamingMarketField::Low));
    assert!(!fields.contains(&StreamingMarketField::MidOpen));

    // Test that we can format each field in the set
    for field in &fields {
        let debug_str = format!("{:?}", field);
        let display_str = format!("{}", field);
        assert!(!debug_str.is_empty());
        assert!(!display_str.is_empty());
        assert_eq!(debug_str, display_str);
    }
}

#[test]
fn test_streaming_market_field_clone_and_equality() {
    // Test Clone and PartialEq implementations work with Debug/Display
    let original = StreamingMarketField::MarketState;
    let cloned = original.clone();

    assert_eq!(original, cloned);
    assert_eq!(format!("{:?}", original), format!("{:?}", cloned));
    assert_eq!(format!("{}", original), format!("{}", cloned));

    // Test inequality
    let different = StreamingMarketField::Bid;
    assert_ne!(original, different);
    assert_ne!(format!("{:?}", original), format!("{:?}", different));
    assert_ne!(format!("{}", original), format!("{}", different));
}
