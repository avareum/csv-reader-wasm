pub mod csv;
mod utils;

use csv::csv::csv_from_str;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_token_transfer_from_csv_string(csv_data: &str) -> Result<JsValue, JsError> {
    set_panic_hook();

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
