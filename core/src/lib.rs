use regex::{Regex, RegexBuilder};

/// ファイルのパスとコンテンツを表す構造体
pub struct FileInput {
    /// ファイルのパス
    pub path: String,
    /// ファイルの内容
    pub content: String,
}

/// 検索結果を表す構造体
pub struct MatchResult {
    /// マッチしたファイルのパス
    pub path: String,
    /// マッチした行番号（1ベース）
    pub line: u32,
    /// マッチした列番号（1ベース）
    pub column: u32,
    /// マッチした行のテキスト
    pub line_text: String,
}

/// パターンでファイルを検索する
///
/// # Arguments
///
/// * `pattern` - 検索する正規表現パターン
/// * `files` - 検索対象のファイルリスト
/// * `case_sensitive` - 大文字小文字を区別するかどうか
///
/// # Returns
///
/// 検索結果のリスト、または正規表現パターンが無効な場合のエラー
pub fn search(
    pattern: &str,
    files: &[FileInput],
    case_sensitive: bool,
) -> Result<Vec<MatchResult>, String> {
    let re = if case_sensitive {
        Regex::new(pattern).map_err(|e| format!("Invalid regex pattern '{}': {}", pattern, e))?
    } else {
        RegexBuilder::new(pattern)
            .case_insensitive(true)
            .build()
            .map_err(|e| format!("Invalid regex pattern '{}': {}", pattern, e))?
    };

    let mut results = Vec::new();

    for f in files {
        for (line_idx, line) in f.content.lines().enumerate() {
            for m in re.find_iter(line) {
                results.push(MatchResult {
                    path: f.path.clone(),
                    line: (line_idx + 1) as u32,
                    column: (m.start() + 1) as u32,
                    line_text: line.to_string(),
                });
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_search_match() {
        let files = vec![FileInput {
            path: "test.txt".to_string(),
            content: "Hello, world!".to_string(),
        }];
        let results = search("world", &files, true).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].path, "test.txt");
        assert_eq!(results[0].line, 1);
        assert_eq!(results[0].column, 8);
        assert_eq!(results[0].line_text, "Hello, world!");
    }

    #[test]
    fn test_search_no_match() {
        let files = vec![FileInput {
            path: "test.txt".to_string(),
            content: "Hello, world!".to_string(),
        }];
        let results = search("foo", &files, true).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_case_insensitive_search() {
        let files = vec![FileInput {
            path: "test.txt".to_string(),
            content: "Hello, WORLD!".to_string(),
        }];
        let results = search("world", &files, false).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].line_text, "Hello, WORLD!");
    }

    #[test]
    fn test_case_sensitive_search() {
        let files = vec![FileInput {
            path: "test.txt".to_string(),
            content: "Hello, WORLD!".to_string(),
        }];
        let results = search("world", &files, true).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_multiline_file() {
        let files = vec![FileInput {
            path: "test.txt".to_string(),
            content: "Line 1\nLine 2\nLine 3".to_string(),
        }];
        let results = search("Line", &files, true).unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].line, 1);
        assert_eq!(results[1].line, 2);
        assert_eq!(results[2].line, 3);
    }

    #[test]
    fn test_multiple_files() {
        let files = vec![
            FileInput {
                path: "file1.txt".to_string(),
                content: "Hello from file1".to_string(),
            },
            FileInput {
                path: "file2.txt".to_string(),
                content: "Hello from file2".to_string(),
            },
        ];
        let results = search("Hello", &files, true).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].path, "file1.txt");
        assert_eq!(results[1].path, "file2.txt");
    }

    #[test]
    fn test_multiple_matches_same_line() {
        let files = vec![FileInput {
            path: "test.txt".to_string(),
            content: "foo bar foo baz".to_string(),
        }];
        let results = search("foo", &files, true).unwrap();
        // re.find_iter() により、すべてのマッチが返される
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].column, 1);
        assert_eq!(results[1].column, 9);
    }

    #[test]
    fn test_regex_pattern() {
        let files = vec![FileInput {
            path: "test.txt".to_string(),
            content: "abc123 def456".to_string(),
        }];
        let results = search(r"\d+", &files, true).unwrap();
        // re.find_iter() により、すべてのマッチが返される
        // "abc123 def456" では "123" と "456" の2つにマッチ
        // "123" は位置3 (0ベース) = 列4 (1ベース)
        // "456" は位置10 (0ベース) = 列11 (1ベース)
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].column, 4); // "123" の開始位置
        assert_eq!(results[1].column, 11); // "456" の開始位置
        assert_eq!(results[0].line_text, "abc123 def456");
        assert_eq!(results[1].line_text, "abc123 def456");
    }

    #[test]
    fn test_invalid_regex_pattern() {
        let files = vec![FileInput {
            path: "test.txt".to_string(),
            content: "Hello, world!".to_string(),
        }];
        let result = search("[", &files, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_file() {
        let files = vec![FileInput {
            path: "empty.txt".to_string(),
            content: "".to_string(),
        }];
        let results = search("test", &files, true).unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_empty_pattern() {
        let files = vec![FileInput {
            path: "test.txt".to_string(),
            content: "Hello, world!".to_string(),
        }];
        let results = search("", &files, true).unwrap();
        // 空のパターンはすべての位置（文字の間）にマッチする
        // "Hello, world!" は13文字なので、14個の位置がある
        assert_eq!(results.len(), 14);
    }

    #[test]
    fn test_column_position() {
        let files = vec![FileInput {
            path: "test.txt".to_string(),
            content: "  Hello".to_string(),
        }];
        let results = search("Hello", &files, true).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].column, 3);
    }
}
