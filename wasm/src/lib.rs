use serde_wasm_bindgen::{from_value, to_value};
use shared::{formatter::format_like_markdown, FileContentWithFileName};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn format_like_markdown_wasm(data: JsValue) -> Result<JsValue, JsValue> {
    let data: Vec<FileContentWithFileName> =
        from_value(data).map_err(|err| JsValue::from_str(&err.to_string()))?;

    let formatted = format_like_markdown(data);

    to_value(&formatted).map_err(|err| JsValue::from_str(&err.to_string()))
}
