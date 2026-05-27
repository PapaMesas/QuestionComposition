// config/mod.rs
// 目的: APIキーと選択プロバイダーをファイルに永続化する。
// APIキーは暗号化された状態でのみ保存し、平文は常にメモリ上にのみ保持する。

pub mod crypto;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::model::LlmProvider;

/// 設定ファイルに保存するデータ構造 (APIキーは暗号化済み文字列)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    /// 選択中の LLM プロバイダー
    pub provider: LlmProvider,
    /// 暗号化された API キー ("nonce_b64:ciphertext_b64" 形式)
    pub encrypted_api_key: Option<String>,
}

/// 設定ファイルのパス: ./outputs/config.toml
pub fn config_path() -> PathBuf {
    PathBuf::from("../outputs/config.toml")
}

/// 設定をファイルから読み込む。ファイルが存在しない場合はデフォルト値を返す
pub fn load() -> AppConfig {
    let path = config_path();
    if !path.exists() {
        return AppConfig::default();
    }
    let content = std::fs::read_to_string(&path).unwrap_or_default();
    toml::from_str(&content).unwrap_or_default()
}

/// 設定をファイルへ保存する
pub fn save(config: &AppConfig) -> Result<()> {
    let path = config_path();
    // 出力先ディレクトリが存在しない場合は作成する
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).context("Failed to create config directory")?;
    }
    let content = toml::to_string_pretty(config).context("Failed to serialize config")?;
    std::fs::write(&path, content).context("Failed to write config file")?;
    Ok(())
}

/// 平文 APIキーを暗号化して設定に格納し、ファイルへ保存する
pub fn store_api_key(config: &mut AppConfig, plaintext_key: &str) -> Result<()> {
    let encrypted = crypto::encrypt(plaintext_key).context("Failed to encrypt API key")?;
    config.encrypted_api_key = Some(encrypted);
    save(config)
}

/// 設定から APIキーを復号して返す
pub fn load_api_key(config: &AppConfig) -> Result<Option<String>> {
    match &config.encrypted_api_key {
        None => Ok(None),
        Some(enc) => {
            let key = crypto::decrypt(enc).context("Failed to decrypt API key")?;
            Ok(Some(key))
        }
    }
}

// --- テスト用: 任意パスへの保存・読み込みを可能にする関数 ---

/// 指定パスから設定を読み込む (統合テスト用)
pub fn load_from(path: &std::path::Path) -> AppConfig {
    if !path.exists() {
        return AppConfig::default();
    }
    let content = std::fs::read_to_string(path).unwrap_or_default();
    toml::from_str(&content).unwrap_or_default()
}

/// 指定パスへ設定を保存し、APIキーを暗号化して格納する (統合テスト用)
pub fn store_api_key_to(
    config: &mut AppConfig,
    path: &std::path::Path,
    plaintext_key: &str,
) -> Result<()> {
    let encrypted = crypto::encrypt(plaintext_key).context("Failed to encrypt API key")?;
    config.encrypted_api_key = Some(encrypted);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).context("Failed to create config directory")?;
    }
    let content = toml::to_string_pretty(config).context("Failed to serialize config")?;
    std::fs::write(path, content).context("Failed to write config file")?;
    Ok(())
}
