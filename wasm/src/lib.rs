// wasm/src/lib.rs
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use simple_find_core::{FileInput, MatchResult as CoreMatchResult};

/// WebAssembly用のファイル入力構造体
#[derive(Deserialize)]
pub struct WasmFileInput {
    /// ファイルのパス
    pub path: String,
    /// ファイルの内容
    pub content: String,
}

/// WebAssembly用の検索結果構造体
#[derive(Serialize)]
pub struct WasmMatchResult {
    /// マッチしたファイルのパス
    pub path: String,
    /// マッチした行番号（1ベース）
    pub line: u32,
    /// マッチした列番号（1ベース）
    pub column: u32,
    /// マッチした行のテキスト
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

/// パターンでファイルを検索する（WebAssembly用）
///
/// # Arguments
///
/// * `pattern` - 検索する正規表現パターン
/// * `files` - 検索対象のファイルリスト（JSON形式）
/// * `case_sensitive` - 大文字小文字を区別するかどうか
///
/// # Returns
///
/// 検索結果のリスト（JSON形式）、またはエラー
#[wasm_bindgen]
pub fn search(pattern: &str, files: &JsValue, case_sensitive: bool) -> Result<JsValue, JsValue> {
    let wasm_files: Vec<WasmFileInput> = serde_wasm_bindgen::from_value(files.clone())
        .map_err(|e| JsValue::from_str(&format!("Failed to deserialize files: {}", e)))?;

    let core_files: Vec<FileInput> = wasm_files
        .into_iter()
        .map(|f| FileInput {
            path: f.path,
            content: f.content,
        })
        .collect();

    let results = simple_find_core::search(pattern, &core_files, case_sensitive)
        .map_err(|e| JsValue::from_str(&format!("Search error: {}", e)))?;

    let wasm_results: Vec<WasmMatchResult> =
        results.into_iter().map(WasmMatchResult::from).collect();

    serde_wasm_bindgen::to_value(&wasm_results)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize results: {}", e)))
}