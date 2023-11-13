use cmcb_core::formatter::{format, MarkdownCodeBlock};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// TODO: Currently, the type in TypeScript is inferred as `format_wasm(data: any): any;`, we want the types to be automatically inferred more specifically.
// At the moment, it's necessary to manually specify the type data as `{ file_name: string; content: string; }[]`
#[wasm_bindgen]
pub fn format_wasm(data: JsValue) -> Result<JsValue, JsValue> {
    let data: Vec<MarkdownCodeBlock> =
        from_value(data).map_err(|err| JsValue::from_str(&err.to_string()))?;

    let formatted = format(data);

    to_value(&formatted).map_err(|err| JsValue::from_str(&err.to_string()))
}
