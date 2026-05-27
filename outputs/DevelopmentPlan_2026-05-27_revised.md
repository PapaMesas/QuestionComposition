# Development Plan — Question Composition Application (Spec Revision: Requirements4add)

**Approval Date**: 2026-05-27 (Revised)  
**Base Plan**: DevelopmentPlan_2026-05-27.md  
**Spec**: Requirements4add.md

---

## English Version

### Overview of Changes

This revised plan covers the spec additions from `Requirements4add.md`, building upon `Requirements3add.md`. Two major additions are identified:

---

### Change 1 — Feature 1: API Deletion Capability

**Requirement addition**: Users must be able to delete stored API configuration.

**Current behavior**: API information can be set and retrieved, but cannot be deleted.

**Required behavior**: A delete/remove button in the API configuration UI allows users to clear the stored API credentials securely.

#### Implementation

**File**: `src/ui/config.rs` (or equivalent UI module)

- Add a "Delete API" or "Remove API" button in the API configuration panel
- Implement a callback that:
  1. Clears the encrypted API key from secure storage
  2. Resets the UI state (button to disabled, input field to empty)
  3. Provides user feedback (success message or confirmation dialog)

**File**: `src/crypto.rs` (or secure storage module)

- Add a `delete_api_key()` function that:
  1. Removes the stored encrypted API key from persistent storage
  2. Returns `Result<(), Error>` indicating success/failure

**File**: `src/main.rs` or equivalent state management

- Wire the delete callback to the storage layer

**Test coverage**:
- Verify delete removes the key from storage
- Verify subsequent reads return an error or empty state
- Verify UI is properly disabled after deletion

---

### Change 2 — Feature 4: Default Rules Encryption & Hidden Display

**Requirement additions**:
1. Default rules (`test_development.md`, `test_guideline.md`) must not be displayed on screen
2. Default rules must be encrypted using AES-192

**Current behavior**: Default rules are loaded and displayed in the UI; no encryption is applied.

**Required behavior**:
- Default rules are encrypted at build time or on first load using AES-192
- Default rules are not rendered in the UI
- Default rules are decrypted only when passed to the LLM

#### Implementation

**File**: `src/crypto.rs`

Add AES-192 encryption/decryption functions:

```rust
pub fn encrypt_aes192(plaintext: &[u8], key: &[u8; 24]) -> Result<Vec<u8>, Error> {
    // Implement AES-192-CBC encryption with PKCS7 padding
    // Return Base64-encoded ciphertext
}

pub fn decrypt_aes192(ciphertext_b64: &str, key: &[u8; 24]) -> Result<String, Error> {
    // Decode from Base64
    // Decrypt AES-192-CBC
    // Return plaintext as String
}
```

- Use a crate like `aes`, `cipher`, or `openssl` for AES-192 (check Cargo.toml compatibility)
- 192-bit key = 24 bytes
- Use a fixed key for decryption (or derive from a master key if desired)

**File**: `src/model.rs`

Update the rule loading structure:

```rust
pub struct DefaultRules {
    pub development_rule_encrypted: String,  // Base64-encoded
    pub guideline_rule_encrypted: String,    // Base64-encoded
}

pub fn load_default_rules() -> Result<DefaultRules, Error> {
    // Load encrypted rule content from file or embedded constant
    // Return without decryption
}

pub fn get_decrypted_rules(key: &[u8; 24]) -> Result<(String, String), Error> {
    // Decrypt both rules only when needed
    // Return plaintext rules
}
```

**File**: `src/ui/rules.rs` (or equivalent UI module)

- Remove the default rules from the rule display panel
- Ensure custom uploaded rules are still displayed normally

**File**: `src/generator.rs`

Update the LLM prompt building:

```rust
// Decrypt rules only when generating choices
let decrypted_development = crypto::decrypt_aes192(&rules.development_rule_encrypted, &key)?;
let decrypted_guideline = crypto::decrypt_aes192(&rules.guideline_rule_encrypted, &key)?;

// Include decrypted rules in LLM prompt
let prompt = format!(
    "Using these rules:\n\nDevelopment Rule:\n{}\n\nGuideline Rule:\n{}\n\n...",
    decrypted_development,
    decrypted_guideline
);
```

**Build integration**:
- Define the 192-bit AES key as a build-time constant or environment variable
- Pre-encrypt `test_development.md` and `test_guideline.md` during build
- Store encrypted versions in the binary or a config file

**Test coverage**:
- Verify encryption/decryption roundtrip
- Verify encrypted rules are not displayed in UI
- Verify decrypted rules are correctly passed to LLM
- Verify decryption fails with an incorrect key

---

### Implementation Phases

| Phase | File(s) | Task |
|---|---|---|
| **P-1** | `src/crypto.rs` | Implement AES-192 encrypt/decrypt functions |
| **P-2** | `src/model.rs` | Add `DefaultRules` struct and loader |
| **P-3** | Build process | Pre-encrypt default rules and embed in binary |
| **P-4** | `src/ui/config.rs` | Add API deletion UI and callback |
| **P-5** | `src/ui/rules.rs` | Hide default rules from display |
| **P-6** | `src/generator.rs` | Decrypt rules at generation time |
| **P-7** | All changed files | Add unit tests; run `cargo test` |

---

## 日本語版

### 変更の概要

このプランは `Requirements4add.md` で追加された仕様を実装します。`Requirements3add.md` の実装に加えて、以下の2つの重要な変更に対応します。

---

### 変更1 — 機能1: API削除機能

**仕様追加内容**: ユーザーは保存されたAPI設定を削除できなければならない。

**現状**: APIキーは設定・取得できるが、削除できない。

**変更後**: API設定パネルに「削除」ボタンを備え、ユーザーがAPIクレデンシャルを安全に削除できる。

#### 実装内容

**ファイル**: `src/ui/config.rs` （またはUI モジュール相当）

- API設定パネルに「API削除」または「API削除」ボタンを追加
- 削除コールバック実装：
  1. 暗号化されたAPIキーをセキュアストレージから削除
  2. UI状態をリセット（ボタンを無効化、入力欄を空に）
  3. ユーザーへフィードバック提供（成功メッセージまたは確認ダイアログ）

**ファイル**: `src/crypto.rs` （またはセキュアストレージモジュール）

- `delete_api_key()` 関数の実装：
  1. 保存されたAPIキーをストレージから削除
  2. `Result<(), Error>` を返却

**ファイル**: `src/main.rs` または状態管理相当

- 削除コールバックをストレージレイヤーにワイヤリング

**テストカバレッジ**:
- 削除がストレージからキーを削除することを確認
- 削除後の読み込みがエラーまたは空状態を返すことを確認
- 削除後にUIが適切に無効化されることを確認

---

### 変更2 — 機能4: デフォルトルール暗号化と非表示

**仕様追加内容**:
1. デフォルトルール（`test_development.md`、`test_guideline.md`）を画面上に表示しない
2. デフォルトルールはAES-192を使用して暗号化する

**現状**: デフォルトルールはUIに表示され、暗号化されていない。

**変更後**:
- デフォルトルールをビルド時または初回読み込み時にAES-192で暗号化
- デフォルトルールをUIに描画しない
- デフォルトルールはLLMに渡す際のみ復号

#### 実装内容

**ファイル**: `src/crypto.rs`

AES-192暗号化・復号関数の追加:

```rust
pub fn encrypt_aes192(plaintext: &[u8], key: &[u8; 24]) -> Result<Vec<u8>, Error> {
    // AES-192-CBC暗号化（PKCS7パディング対応）
    // Base64エンコード結果を返却
}

pub fn decrypt_aes192(ciphertext_b64: &str, key: &[u8; 24]) -> Result<String, Error> {
    // Base64デコード
    // AES-192-CBC復号
    // プレーンテキストを文字列として返却
}
```

- `aes`, `cipher`, `openssl` などのクレートを使用（Cargo.toml 互換性確認）
- 192ビット鍵 = 24バイト
- 復号用に固定鍵を使用（または必要に応じてマスター鍵から派生）

**ファイル**: `src/model.rs`

ルール読み込み構造の更新:

```rust
pub struct DefaultRules {
    pub development_rule_encrypted: String,  // Base64エンコード
    pub guideline_rule_encrypted: String,    // Base64エンコード
}

pub fn load_default_rules() -> Result<DefaultRules, Error> {
    // 暗号化されたルール内容をファイルまたは埋め込み定数から読み込み
    // 復号なしで返却
}

pub fn get_decrypted_rules(key: &[u8; 24]) -> Result<(String, String), Error> {
    // 必要な時のみ両ルールを復号
    // プレーンテキストルールを返却
}
```

**ファイル**: `src/ui/rules.rs` （またはUI モジュール相当）

- デフォルトルールをルール表示パネルから削除
- カスタムアップロードルールは通常通り表示

**ファイル**: `src/generator.rs`

LLMプロンプト作成の更新:

```rust
// 選択肢生成時のみルール復号
let decrypted_development = crypto::decrypt_aes192(&rules.development_rule_encrypted, &key)?;
let decrypted_guideline = crypto::decrypt_aes192(&rules.guideline_rule_encrypted, &key)?;

// LLMプロンプトに復号ルールを組み込む
let prompt = format!(
    "Using these rules:\n\nDevelopment Rule:\n{}\n\nGuideline Rule:\n{}\n\n...",
    decrypted_development,
    decrypted_guideline
);
```

**ビルド統合**:
- 192ビットAES鍵をビルド時定数または環境変数として定義
- ビルド中に `test_development.md` と `test_guideline.md` を事前暗号化
- 暗号化版をバイナリまたは設定ファイルに格納

**テストカバレッジ**:
- 暗号化・復号ラウンドトリップを確認
- 暗号化されたルールがUIに表示されないことを確認
- 復号されたルールがLLMに正しく渡されることを確認
- 誤った鍵で復号が失敗することを確認

---

### 実装フェーズ

| フェーズ | ファイル | 内容 |
|---|---|---|
| **P-1** | `src/crypto.rs` | AES-192 暗号化・復号関数の実装 |
| **P-2** | `src/model.rs` | `DefaultRules` 構造体とローダー追加 |
| **P-3** | ビルドプロセス | デフォルトルール事前暗号化とバイナリ埋め込み |
| **P-4** | `src/ui/config.rs` | API削除UI とコールバック追加 |
| **P-5** | `src/ui/rules.rs` | デフォルトルール表示非表示 |
| **P-6** | `src/generator.rs` | 生成時ルール復号 |
| **P-7** | 変更ファイル全体 | ユニットテスト追加・`cargo test` 実行 |

---

## Additional Notes / 補足

- **Dependencies**: Verify that `aes` and `cipher` crates are compatible with the current Rust toolchain. Consider `aes-gcm` for authenticated encryption if security is critical.
- **Key Management**: The 192-bit key should be treated as a secret. For production, consider using `dotenv` or a secure key derivation function (KDF).
- **User Communication**: Consider a confirmation dialog before deleting API keys to prevent accidental removal.

