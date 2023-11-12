use cmcb_shared::formatter::{format, MarkdownCodeBlock};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// TODO: ts 側で型が format_wasm(data: any): any; になってしまうので、型が付くようにしたい
// 今は type data = { file_name: string; content: string; }[] を明示的に指定する必要あり
#[wasm_bindgen]
pub fn format_wasm(data: JsValue) -> Result<JsValue, JsValue> {
    let data: Vec<MarkdownCodeBlock> =
        from_value(data).map_err(|err| JsValue::from_str(&err.to_string()))?;

    let formatted = format(data);

    to_value(&formatted).map_err(|err| JsValue::from_str(&err.to_string()))
}
