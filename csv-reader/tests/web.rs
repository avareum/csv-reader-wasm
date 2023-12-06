//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use csv_reader::{csv::csv::TokenTransferJob, get_token_transfer_from_csv_string};
use serde_wasm_bindgen::from_value;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_get_token_transfer_from_csv_string() {
    let token_transfer = get_token_transfer_from_csv_string().ok().unwrap();

    let token_transfer = from_value::<Vec<TokenTransferJob>>(token_transfer)
        .ok()
        .unwrap();

    console_log!("{token_transfer:#?}");
    assert!(token_transfer.len() > 0);

    // Test case 1: Valid CSV data
    // let csv_data = "name,chain_id,to,symbol,amount\nJohn Doe,56,0x710587D0b618E1fBBD5016762F126009B52deCF5,USDT,1.234";
    // let expected_tokens = vec![TokenTransferJob {
    //     name: String::from("John Doe"),
    //     chain_id: String::from("56"),
    //     to: H160([
    //         113, 5, 135, 208, 182, 24, 225, 251, 189, 80, 22, 118, 47, 18, 96, 9, 181, 45, 236, 245,
    //     ]),
    //     symbol: String::from("USDT"),
    //     amount: 1.234,
    // }];
    // assert_eq!(csv_from_str(csv_data).unwrap(), expected_tokens);

    // // Test case 2: Empty CSV data
    // let empty_csv_data = "";
    // let expected_empty_tokens: Vec<TokenTransferJob> = vec![];
    // assert_eq!(csv_from_str(empty_csv_data).unwrap(), expected_empty_tokens);

    // // Test case 3: Invalid CSV data (for example, missing fields)
    // let invalid_csv_data =
    //     "chain_id,to,symbol,amount\n56,0x710587D0b618E1fBBD5016762F126009B52deCF5,USDT";
    // assert!(csv_from_str(invalid_csv_data).is_err());
}
