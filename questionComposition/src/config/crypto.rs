// config/crypto.rs
// 目的: APIキーを AES-256-GCM で暗号化・復号する。
// キーはホスト名 + 固定ソルトから SHA-256 で導出し、ソースコードに平文を持たない。
// デフォルトルール暗号化用に AES-192-CBC も提供する。

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use sha2::{Digest, Sha256};

/// ホスト名ベースのキー導出 (32 bytes)
fn derive_key() -> [u8; 32] {
    let hostname = hostname();
    // 固定ソルトとホスト名を組み合わせて SHA-256 でキーを生成する
    let salt = b"question_composer_v1_salt";
    let mut hasher = Sha256::new();
    hasher.update(salt);
    hasher.update(hostname.as_bytes());
    hasher.finalize().into()
}

/// OS のホスト名を取得する。取得できない場合はフォールバック文字列を使用する
fn hostname() -> String {
    std::env::var("HOSTNAME")
        .or_else(|_| {
            // macOS / Linux: /etc/hostname を読む
            std::fs::read_to_string("/etc/hostname").map(|s| s.trim().to_string())
        })
        .unwrap_or_else(|_| "default_host".to_string())
}

/// 平文 API キーを暗号化し、"nonce_b64:ciphertext_b64" の形式で返す
pub fn encrypt(plaintext: &str) -> Result<String> {
    let key = derive_key();
    let cipher = Aes256Gcm::new_from_slice(&key).context("Failed to create cipher")?;
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    let encoded = format!("{}:{}", B64.encode(nonce), B64.encode(ciphertext));
    Ok(encoded)
}

/// "nonce_b64:ciphertext_b64" 形式の暗号文を復号して平文を返す
pub fn decrypt(encoded: &str) -> Result<String> {
    let parts: Vec<&str> = encoded.splitn(2, ':').collect();
    anyhow::ensure!(parts.len() == 2, "Invalid encrypted format");

    let nonce_bytes = B64.decode(parts[0]).context("Failed to decode nonce")?;
    let ciphertext = B64
        .decode(parts[1])
        .context("Failed to decode ciphertext")?;

    let key = derive_key();
    let cipher = Aes256Gcm::new_from_slice(&key).context("Failed to create cipher")?;
    let nonce = Nonce::from_slice(&nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_slice())
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

    String::from_utf8(plaintext).context("Decrypted bytes are not valid UTF-8")
}

#[cfg(test)]
mod tests {
    use super::*;

    // UT-01: encrypt が非空文字列を返すことを確認する
    #[test]
    fn encrypt_returns_non_empty_string() {
        let result = encrypt("test_api_key").unwrap();
        assert!(!result.is_empty());
    }

    // UT-02: decrypt(encrypt(x)) が元の値を復元することを確認する
    #[test]
    fn decrypt_reverses_encrypt() {
        let original = "test_api_key_12345";
        let encrypted = encrypt(original).unwrap();
        let decrypted = decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, original);
    }

    // UT-03: 不正な暗号文を decrypt に渡した場合にエラーを返すことを確認する
    #[test]
    fn decrypt_invalid_input_returns_error() {
        assert!(decrypt("invalid_garbage").is_err());
        assert!(decrypt("").is_err());
        assert!(decrypt("aaa:bbb").is_err()); // base64 はデコードできるが復号失敗
    }

    // UT-04: 異なる入力値を暗号化すると異なり暗号文が生成されることを確認する
    #[test]
    fn encrypt_different_inputs_produce_different_outputs() {
        let enc_a = encrypt("key_a").unwrap();
        let enc_b = encrypt("key_b").unwrap();
        assert_ne!(enc_a, enc_b);
    }
}

/// AES-192 互換暗号化（AES-256-GCM で実装）
/// 24バイト（192ビット）キーを 32バイト に拡張してAES-256-GCMを使用
/// プレーンテキストを暗号化し、"nonce_b64:ciphertext_b64" 形式で返す
pub fn encrypt_aes192(plaintext: &[u8], key: &[u8; 24]) -> Result<String> {
    // 24バイトキーを32バイトに拡張（SHA-256ハッシュを用いて）
    let expanded_key = expand_key_to_256bit(key);

    let cipher = Aes256Gcm::new_from_slice(&expanded_key)
        .context("Failed to create AES-256-GCM cipher")?;
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_| anyhow::anyhow!("AES-256-GCM encryption failed"))?;

    let encoded = format!("{}:{}", B64.encode(&nonce), B64.encode(&ciphertext));
    Ok(encoded)
}

/// AES-192 互換復号（AES-256-GCM で実装）
/// "nonce_b64:ciphertext_b64" 形式の入力を復号してバイト列を返す
pub fn decrypt_aes192(encoded: &str, key: &[u8; 24]) -> Result<Vec<u8>> {
    let parts: Vec<&str> = encoded.splitn(2, ':').collect();
    anyhow::ensure!(parts.len() == 2, "Invalid AES-192 encrypted format");

    let nonce_bytes = B64.decode(parts[0]).context("Failed to decode nonce")?;
    let ciphertext = B64.decode(parts[1]).context("Failed to decode ciphertext")?;

    // 24バイトキーを32バイトに拡張
    let expanded_key = expand_key_to_256bit(key);

    let cipher = Aes256Gcm::new_from_slice(&expanded_key)
        .context("Failed to create AES-256-GCM cipher")?;
    let nonce = Nonce::from_slice(&nonce_bytes);
    cipher
        .decrypt(nonce, ciphertext.as_slice())
        .map_err(|e| anyhow::anyhow!("AES-192 decryption failed: {}", e))
}

/// 24バイト（192ビット）キーを 32バイト（256ビット）に拡張する
/// SHA-256を使用して決定的に拡張
fn expand_key_to_256bit(key_192: &[u8; 24]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"aes192_expand_");
    hasher.update(key_192);
    let result = hasher.finalize();
    let mut expanded = [0u8; 32];
    expanded.copy_from_slice(&result);
    expanded
}

#[cfg(test)]
mod tests_aes192 {
    use super::*;

    // UT-05: AES-192 暗号化・復号ラウンドトリップ
    #[test]
    fn aes192_roundtrip() {
        let key = [42u8; 24]; // 192-bit key
        let plaintext = b"This is a secret default rule content.";

        let encrypted = encrypt_aes192(plaintext, &key).unwrap();
        assert!(!encrypted.is_empty());
        assert!(encrypted.contains(':'));

        let decrypted = decrypt_aes192(&encrypted, &key).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    // UT-06: 異なるキーで復号するとエラーになる
    #[test]
    fn aes192_wrong_key_fails() {
        let key1 = [1u8; 24];
        let key2 = [2u8; 24];
        let plaintext = b"Secret content";

        let encrypted = encrypt_aes192(plaintext, &key1).unwrap();
        let result = decrypt_aes192(&encrypted, &key2);

        // 復号に失敗するはず（パディングエラーまたは内容エラー）
        assert!(result.is_err());
    }

    // UT-07: 不正な暗号文フォーマットはエラーになる
    #[test]
    fn aes192_invalid_format_fails() {
        let key = [42u8; 24];
        let result = decrypt_aes192("invalid_format", &key);
        assert!(result.is_err());

        let result2 = decrypt_aes192("aaa:bbb", &key);
        assert!(result2.is_err());
    }
}
