pub mod csv;
mod utils;

use csv::csv::csv_from_str;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_token_transfer_from_csv_string() -> Result<JsValue, JsError> {
    set_panic_hook();

    let csv_data = "name,chain_id,to,symbol,amount\nJohn Doe,56,0x710587D0b618E1fBBD5016762F126009B52deCF5,USDT,1.234";
    let data = csv_from_str(csv_data);

    let data = match data {
        Ok(data) => data,
        Err(err) => return Err(JsError::new(&err.to_string())),
    };

    match serde_wasm_bindgen::to_value(&data) {
        Ok(data) => Ok(data),
        Err(err) => Err(JsError::new(&err.to_string())),
    }
}
