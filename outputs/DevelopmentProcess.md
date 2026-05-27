# 開発プロセス記録 — 設問作成支援ツール

完了日: 2026-04-30

---

## フェーズ一覧と実施内容

| フェーズ | 内容 | 状態 |
|---|---|---|
| P0 | 要件読み込み・InputMaterials確認 | ✅ 完了 |
| P1 | プロジェクト雛形、Cargo.toml、`model.rs` | ✅ 完了 |
| P2 | `config/crypto.rs` (AES-256-GCM)、`config/mod.rs` | ✅ 完了 |
| P3 | `excel/reader.rs` (calamine) | ✅ 完了 |
| P4 | `llm/mod.rs`, `gemini.rs`, `openai.rs`, `claude.rs` | ✅ 完了 |
| P5 | `rule_loader.rs` (デフォルトルール自動読み込み) | ✅ 完了 |
| P6 | `generator.rs` (プロンプト構築・LLM呼び出し・選択肢組み立て) | ✅ 完了 |
| P7 | `excel/writer.rs` (rust_xlsxwriter) | ✅ 完了 |
| P8 | `ui/settings_panel.rs` (API設定UI) | ✅ 完了 |
| P9 | `ui/import_panel.rs`, `ui/question_panel.rs`, `ui/rule_panel.rs` | ✅ 完了 |
| P10 | `ui/generate_panel.rs`, `app.rs`, `main.rs`, 統合 | ✅ 完了 |
| T1 | 単体テスト実装・実行 (19件 全PASS) | ✅ 完了 |
| T2 | 統合テスト実装・実行 (2件 全PASS) | ✅ 完了 |
| P11 | 日本語フォント対応: Noto Sans JP Medium をバイナリ埋め込み | ✅ 完了 |

---

## ファイル構成

```
questionComposition/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── app.rs
│   ├── model.rs
│   ├── generator.rs
│   ├── rule_loader.rs
│   ├── config/
│   │   ├── mod.rs
│   │   └── crypto.rs
│   ├── excel/
│   │   ├── mod.rs
│   │   ├── reader.rs
│   │   └── writer.rs
│   ├── llm/
│   │   ├── mod.rs
│   │   ├── gemini.rs
│   │   ├── openai.rs
│   │   └── claude.rs
│   └── ui/
│       ├── mod.rs
│       ├── settings_panel.rs
│       ├── import_panel.rs
│       ├── question_panel.rs
│       ├── rule_panel.rs
│       └── generate_panel.rs
└── tests/
    └── integration_test.rs
```

---

## ビルド・テスト実績

| コマンド | 結果 |
|---|---|
| `cargo build` | ✅ 成功 |
| `cargo build --release` | ✅ 成功 |
| `cargo clippy -- -D warnings` | ✅ エラーなし |
| `cargo fmt --check` | ✅ エラーなし |
| `cargo test` | ✅ 21/21 PASS |
| 日本語表示確認 (文字化け解消) | ✅ 確認済 |

---

## 主要な技術的決定事項

| 決定 | 理由 |
|---|---|
| `egui/eframe` を GUI フレームワークとして採用 | 純Rust・クロスプラットフォーム・依存が少ない |
| AES-256-GCM でAPIキーを暗号化 | 平文保存を避けつつ実装が単純 |
| ホスト名ベースの鍵導出 | ファイルを別PCにコピーしても復号不可 |
| `reqwest::blocking` を使用 | egui の同期描画ループと相性が良い |
| デフォルトルールをファイルから自動読み込み | 起動時に InputMaterials を参照し、なければ内蔵フォールバックを使用 |
| LLM レスポンスの JSON 配列を `[` `]` で抽出 | コードブロック付きレスポンスにも対応 |
| Noto Sans JP Medium をバイナリ埋め込み | OFL ライセンス・再配布可能・日本語文字化け解消 |

---

## 出力ドキュメント

| ファイル | 内容 |
|---|---|
| `outputs/DevelopmentPlan.md` | 開発計画 (英語版・日本語版) |
| `outputs/TestSpecification.md` | テスト仕様書 |
| `outputs/TestResults.md` | テスト結果 |
| `outputs/DevelopmentProcess.md` | 本ドキュメント |
| `outputs/manual.md` | 操作マニュアル |
