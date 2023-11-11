use serde_wasm_bindgen::{from_value, to_value};
use shared::formatter::{format, MarkdownCodeBlock};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn format_wasm(data: JsValue) -> Result<JsValue, JsValue> {
    let data: Vec<MarkdownCodeBlock> =
        from_value(data).map_err(|err| JsValue::from_str(&err.to_string()))?;

    let formatted = format(data);

    to_value(&formatted).map_err(|err| JsValue::from_str(&err.to_string()))
}
