//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use csv_reader::{csv::csv::TokenTransferJob, get_token_transfer_from_csv_string};
use ethers::types::H160;
use serde_wasm_bindgen::from_value;
use std::str::FromStr;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_get_token_transfer_from_csv_string() {
    // Test case 1: Valid CSV data
    let csv_data = "name,chain_id,to,symbol,amount\nJohn Doe,56,0x710587D0b618E1fBBD5016762F126009B52deCF5,USDT,1.234";

    let token_transfer = get_token_transfer_from_csv_string(csv_data).ok().unwrap();
    let token_transfer = from_value::<Vec<TokenTransferJob>>(token_transfer)
        .ok()
        .unwrap();

    console_log!("{token_transfer:#?}");
    assert!(token_transfer.len() > 0);

    let expected_tokens = vec![TokenTransferJob {
        name: String::from("John Doe"),
        chain_id: String::from("56"),
        to: H160::from_str("0x710587D0b618E1fBBD5016762F126009B52deCF5").unwrap(),
        symbol: String::from("USDT"),
        amount: 1.234,
    }];

    assert_eq!(token_transfer, expected_tokens);

    // Test case 2: Empty CSV data
    let empty_csv_data = "";

    let token_transfer = get_token_transfer_from_csv_string(empty_csv_data)
        .ok()
        .unwrap();
    let token_transfer = from_value::<Vec<TokenTransferJob>>(token_transfer)
        .ok()
        .unwrap();

    console_log!("{token_transfer:#?}");
    assert!(token_transfer.len() == 0);

    let expected_empty_tokens: Vec<TokenTransferJob> = vec![];
    assert_eq!(token_transfer, expected_empty_tokens);

    // Test case 3: Invalid CSV data (for example, missing fields)
    let invalid_csv_data =
        "chain_id,to,symbol,amount\n56,0x710587D0b618E1fBBD5016762F126009B52deCF5,USDT";
    let token_transfer = get_token_transfer_from_csv_string(invalid_csv_data);

    console_log!("{token_transfer:#?}");
    assert!(token_transfer.is_err());
}
