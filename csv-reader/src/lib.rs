use std::io;

use ethers::types::H160;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TokenTransferJob {
    name: String,     // "John Doe"
    chain_id: String, // "56"
    to: H160, // "0x710587D0b618E1fBBD5016762F126009B52deCF5", "0xD7f07B4686616712bFDD84787cc04a5d8F282e8A"
    symbol: String, // "USDT"
    amount: f64, // 1.234
}

pub fn csv_from_str(s: &str) -> anyhow::Result<Vec<TokenTransferJob>> {
    let cursor = io::Cursor::new(s);
    let mut rdr = csv::Reader::from_reader(cursor);
    let mut tokens = vec![];
    for result in rdr.deserialize() {
        let token: TokenTransferJob = result?;
        tokens.push(token)
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_from_str() {
        // Test case 1: Valid CSV data
        let csv_data = "name,chain_id,to,symbol,amount\nJohn Doe,56,0x710587D0b618E1fBBD5016762F126009B52deCF5,USDT,1.234";
        let expected_tokens = vec![TokenTransferJob {
            name: String::from("John Doe"),
            chain_id: String::from("56"),
            to: H160([
                113, 5, 135, 208, 182, 24, 225, 251, 189, 80, 22, 118, 47, 18, 96, 9, 181, 45, 236,
                245,
            ]),
            symbol: String::from("USDT"),
            amount: 1.234,
        }];
        assert_eq!(csv_from_str(csv_data).unwrap(), expected_tokens);

        // Test case 2: Empty CSV data
        let empty_csv_data = "";
        let expected_empty_tokens: Vec<TokenTransferJob> = vec![];
        assert_eq!(csv_from_str(empty_csv_data).unwrap(), expected_empty_tokens);

        // Test case 3: Invalid CSV data (for example, missing fields)
        let invalid_csv_data =
            "chain_id,to,symbol,amount\n56,0x710587D0b618E1fBBD5016762F126009B52deCF5,USDT";
        assert!(csv_from_str(invalid_csv_data).is_err());
    }
}
