// wasm/src/lib.rs
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use simple_find_core::{FileInput, MatchResult as CoreMatchResult};

/// WebAssembly用のファイル入力構造体
#[derive(Deserialize, Serialize)]
pub struct WasmFileInput {
    /// ファイルのパス
    pub path: String,
    /// ファイルの内容
    pub content: String,
}

/// WebAssembly用の検索結果構造体
#[derive(Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    fn create_test_files() -> JsValue {
        let files = vec![
            WasmFileInput {
                path: "test.txt".to_string(),
                content: "Hello, world!".to_string(),
            },
        ];
        serde_wasm_bindgen::to_value(&files).unwrap()
    }

    #[wasm_bindgen_test]
    fn test_basic_search_match() {
        let files = create_test_files();
        let result = search("world", &files, true).unwrap();
        let results: Vec<WasmMatchResult> = serde_wasm_bindgen::from_value(result).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].path, "test.txt");
        assert_eq!(results[0].line, 1);
        assert_eq!(results[0].column, 8);
        assert_eq!(results[0].line_text, "Hello, world!");
    }

    #[wasm_bindgen_test]
    fn test_search_no_match() {
        let files = create_test_files();
        let result = search("foo", &files, true).unwrap();
        let results: Vec<WasmMatchResult> = serde_wasm_bindgen::from_value(result).unwrap();

        assert_eq!(results.len(), 0);
    }

    #[wasm_bindgen_test]
    fn test_case_insensitive_search() {
        let files = vec![WasmFileInput {
            path: "test.txt".to_string(),
            content: "Hello, WORLD!".to_string(),
        }];
        let files_js = serde_wasm_bindgen::to_value(&files).unwrap();
        let result = search("world", &files_js, false).unwrap();
        let results: Vec<WasmMatchResult> = serde_wasm_bindgen::from_value(result).unwrap();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].line_text, "Hello, WORLD!");
    }

    #[wasm_bindgen_test]
    fn test_case_sensitive_search() {
        let files = vec![WasmFileInput {
            path: "test.txt".to_string(),
            content: "Hello, WORLD!".to_string(),
        }];
        let files_js = serde_wasm_bindgen::to_value(&files).unwrap();
        let result = search("world", &files_js, true).unwrap();
        let results: Vec<WasmMatchResult> = serde_wasm_bindgen::from_value(result).unwrap();

        assert_eq!(results.len(), 0);
    }

    #[wasm_bindgen_test]
    fn test_multiline_file() {
        let files = vec![WasmFileInput {
            path: "test.txt".to_string(),
            content: "Line 1\nLine 2\nLine 3".to_string(),
        }];
        let files_js = serde_wasm_bindgen::to_value(&files).unwrap();
        let result = search("Line", &files_js, true).unwrap();
        let results: Vec<WasmMatchResult> = serde_wasm_bindgen::from_value(result).unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(results[0].line, 1);
        assert_eq!(results[1].line, 2);
        assert_eq!(results[2].line, 3);
    }

    #[wasm_bindgen_test]
    fn test_multiple_files() {
        let files = vec![
            WasmFileInput {
                path: "file1.txt".to_string(),
                content: "Hello from file1".to_string(),
            },
            WasmFileInput {
                path: "file2.txt".to_string(),
                content: "Hello from file2".to_string(),
            },
        ];
        let files_js = serde_wasm_bindgen::to_value(&files).unwrap();
        let result = search("Hello", &files_js, true).unwrap();
        let results: Vec<WasmMatchResult> = serde_wasm_bindgen::from_value(result).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].path, "file1.txt");
        assert_eq!(results[1].path, "file2.txt");
    }

    #[wasm_bindgen_test]
    fn test_multiple_matches_same_line() {
        let files = vec![WasmFileInput {
            path: "test.txt".to_string(),
            content: "foo bar foo baz".to_string(),
        }];
        let files_js = serde_wasm_bindgen::to_value(&files).unwrap();
        let result = search("foo", &files_js, true).unwrap();
        let results: Vec<WasmMatchResult> = serde_wasm_bindgen::from_value(result).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].column, 1);
        assert_eq!(results[1].column, 9);
    }

    #[wasm_bindgen_test]
    fn test_regex_pattern() {
        let files = vec![WasmFileInput {
            path: "test.txt".to_string(),
            content: "abc123 def456".to_string(),
        }];
        let files_js = serde_wasm_bindgen::to_value(&files).unwrap();
        let result = search(r"\d+", &files_js, true).unwrap();
        let results: Vec<WasmMatchResult> = serde_wasm_bindgen::from_value(result).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].column, 4);
        assert_eq!(results[1].column, 11);
    }

    #[wasm_bindgen_test]
    fn test_invalid_regex_pattern() {
        let files = create_test_files();
        let result = search("[", &files, true);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().as_string().unwrap();
        assert!(error_msg.contains("Search error"));
    }

    #[wasm_bindgen_test]
    fn test_empty_file() {
        let files = vec![WasmFileInput {
            path: "empty.txt".to_string(),
            content: "".to_string(),
        }];
        let files_js = serde_wasm_bindgen::to_value(&files).unwrap();
        let result = search("test", &files_js, true).unwrap();
        let results: Vec<WasmMatchResult> = serde_wasm_bindgen::from_value(result).unwrap();

        assert_eq!(results.len(), 0);
    }

    #[wasm_bindgen_test]
    fn test_invalid_json_input() {
        let invalid_json = JsValue::from_str("not valid json");
        let result = search("test", &invalid_json, true);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().as_string().unwrap();
        assert!(error_msg.contains("Failed to deserialize files"));
    }
}