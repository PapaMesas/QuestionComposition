// rule_loader.rs
// 目的: 選択肢生成ルール (.md ファイル) を読み込み、LLM プロンプトへ渡す文字列を返す。
// デフォルトルールは ./InputMaterials/ 内の2ファイルをあらかじめ組み込む。

use std::path::Path;

/// デフォルトルールのパス一覧 (プロジェクトルートからの相対パス)
const DEFAULT_RULE_PATHS: &[&str] = &[
    "../InputMaterials/test_development.md",
    "../InputMaterials/test_guideline.md",
];

/// ルールファイルの状態を保持する構造体
#[derive(Debug, Clone)]
pub struct RuleSet {
    /// 現在読み込まれているルール本文 (LLM に渡す文字列)
    pub content: String,
    /// ルールのソース説明 (UI 表示用)
    pub source_label: String,
}

impl Default for RuleSet {
    fn default() -> Self {
        Self::load_defaults()
    }
}

impl RuleSet {
    /// デフォルトルールを読み込む。どちらかのファイルが存在すれば読み込む
    pub fn load_defaults() -> Self {
        let mut parts = Vec::new();
        for path_str in DEFAULT_RULE_PATHS {
            let path = Path::new(path_str);
            if let Ok(content) = std::fs::read_to_string(path) {
                parts.push(content);
            }
        }

        if parts.is_empty() {
            // ファイルが見つからない場合は最低限のデフォルトルールを埋め込む
            return Self {
                content: fallback_rule(),
                source_label: "内蔵デフォルトルール".to_string(),
            };
        }

        Self {
            content: parts.join("\n\n---\n\n"),
            source_label: "デフォルト (InputMaterials)".to_string(),
        }
    }

    /// カスタムルールファイルを読み込む
    pub fn load_from_file(path: &Path) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let label = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.display().to_string());
        Ok(Self {
            content,
            source_label: label,
        })
    }
}

/// ファイルが見つからない場合の最低限ルール
fn fallback_rule() -> String {
    r#"
# 選択肢生成ルール

- いずれの選択肢もそれらしい内容であること
- 正答枝と誤答枝が明確に区別できること
- 選択肢の長さをおおむね揃えること
- 否定表現・強意語を使わないこと
- 明らかな誤答や遊び選択肢を含めないこと
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // UT-16: デフォルトルールが空でない
    #[test]
    fn default_ruleset_content_is_not_empty() {
        let rule = RuleSet::load_defaults();
        assert!(!rule.content.is_empty());
    }

    // UT-17: カスタム .md ファイルを読み込めることを確認する
    #[test]
    fn loads_custom_md_file() {
        let mut tmp = NamedTempFile::with_suffix(".md").unwrap();
        write!(tmp, "# カスタムルール\n- テスト項目").unwrap();

        let rule = RuleSet::load_from_file(tmp.path()).unwrap();
        assert!(rule.content.contains("カスタムルール"));
        assert!(!rule.source_label.is_empty());
    }

    // UT-18: 存在しないファイルを渡した場合にエラーを返す
    #[test]
    fn returns_error_for_nonexistent_md_file() {
        let result = RuleSet::load_from_file(Path::new("/tmp/nonexistent_rule_99999.md"));
        assert!(result.is_err());
    }
}
