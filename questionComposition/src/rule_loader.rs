// rule_loader.rs
// 目的: 選択肢生成ルール (.md ファイル) を読み込み、LLM プロンプトへ渡す文字列を返す。
// デフォルトルールは AES-192 で暗号化して保持し、LLM 生成時のみ復号する。
// UI には表示しない。

use std::path::Path;
use crate::model::DefaultRules;
use crate::config::crypto;

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

/// デフォルトルール情報（暗号化版を保持）
#[derive(Debug, Clone)]
pub struct EncryptedDefaultRules {
    /// 暗号化された development ルール
    pub development_encrypted: String,
    /// 暗号化された guideline ルール
    pub guideline_encrypted: String,
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

/// デフォルトルールを読み込んで AES-192 で暗号化する
/// AES-192 用の固定キー（192ビット = 24バイト）を使用
pub fn load_and_encrypt_default_rules(key: &[u8; 24]) -> Result<DefaultRules, std::io::Error> {
    // test_development.md を読み込む
    let dev_content = std::fs::read_to_string(DEFAULT_RULE_PATHS[0])
        .unwrap_or_else(|_| fallback_rule());

    // test_guideline.md を読み込む（存在しない場合はfallback）
    let guide_content = if DEFAULT_RULE_PATHS.len() > 1 {
        std::fs::read_to_string(DEFAULT_RULE_PATHS[1])
            .unwrap_or_else(|_| fallback_rule())
    } else {
        fallback_rule()
    };

    // AES-192 で暗号化
    let dev_encrypted = crypto::encrypt_aes192(dev_content.as_bytes(), key)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let guide_encrypted = crypto::encrypt_aes192(guide_content.as_bytes(), key)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    Ok(DefaultRules {
        development_rule_encrypted: dev_encrypted,
        guideline_rule_encrypted: guide_encrypted,
    })
}

/// AES-192 で暗号化されたデフォルトルールを復号する
pub fn decrypt_default_rules(
    rules: &DefaultRules,
    key: &[u8; 24],
) -> Result<(String, String), anyhow::Error> {
    use anyhow::Context;

    let dev_bytes = crypto::decrypt_aes192(&rules.development_rule_encrypted, key)
        .context("Failed to decrypt development rule")?;
    let dev_text = String::from_utf8(dev_bytes).context("Invalid UTF-8 in decrypted development rule")?;

    let guide_bytes = crypto::decrypt_aes192(&rules.guideline_rule_encrypted, key)
        .context("Failed to decrypt guideline rule")?;
    let guide_text = String::from_utf8(guide_bytes)
        .context("Invalid UTF-8 in decrypted guideline rule")?;

    Ok((dev_text, guide_text))
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
