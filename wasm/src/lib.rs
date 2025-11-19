// wasm/src/lib.rs
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use simple_find_core::{FileInput, MatchResult as CoreMatchResult};

#[derive(Deserialize)]
pub struct WasmFileInput {
    pub path: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct WasmMatchResult {
    pub path: String,
    pub line: u32,
    pub column: u32,
    pub line_text: String,
}

impl From<CoreMatchResult> for WasmMatchResult {
    fn from(m: CoreMatchResult) -> Self {
        Self {
            path: m.path,
            line: m.line,
            column: m.column,
            line_text: m.line_text,
        }
    }
}

#[wasm_bindgen]
pub fn search(pattern: &str, files: &JsValue, case_sensitive: bool) -> Result<JsValue, JsValue> {
    let files: Vec<WasmFileInput> =
        files.into_serde().map_err(|e| JsValue::from_str(&e.to_string()))?;

    let core_files: Vec<FileInput> = files
        .into_iter()
        .map(|f| FileInput {
            path: f.path,
            content: f.content,
        })
        .collect();

    let results = simple_find_core::search(pattern, &core_files, case_sensitive)
        .map_err(|e| JsValue::from_str(&e))?;

    let wasm_results: Vec<WasmMatchResult> =
        results.into_iter().map(WasmMatchResult::from).collect();

    JsValue::from_serde(&wasm_results)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}