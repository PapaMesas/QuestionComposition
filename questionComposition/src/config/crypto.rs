// config/crypto.rs
// 目的: APIキーを AES-256-GCM で暗号化・復号する。
// キーはホスト名 + 固定ソルトから SHA-256 で導出し、ソースコードに平文を持たない。

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

    // UT-04: 異なる入力値を暗号化すると異なる暗号文が生成されることを確認する
    #[test]
    fn encrypt_different_inputs_produce_different_outputs() {
        let enc_a = encrypt("key_a").unwrap();
        let enc_b = encrypt("key_b").unwrap();
        assert_ne!(enc_a, enc_b);
    }
}
