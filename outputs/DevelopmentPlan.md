# Development Plan — Question Composition Application

Approved: 2026-04-30

---

## English Version

### Overview

A desktop GUI application built in **Rust** using **egui/eframe** that:
1. Manages LLM API credentials (encrypted)
2. Imports question data from `.xlsx`
3. Configures choices per question
4. Loads choice-generation rules from `.md` files
5. Generates distractor choices via LLM and exports to `.xlsx`

---

### Technology Stack

| Concern | Crate / Tool |
|---|---|
| GUI | `egui` + `eframe` |
| Excel read | `calamine` |
| Excel write | `rust_xlsxwriter` |
| HTTP / LLM calls | `reqwest` + `tokio` |
| JSON serialization | `serde` + `serde_json` |
| Config persistence | `serde` + `toml` |
| API key encryption | `aes-gcm` + `rand` |
| Markdown read | plain `std::fs` (raw text → LLM prompt) |

---

### Module Structure

```
questionComposition/
├── Cargo.toml
└── src/
    ├── main.rs              # Entry point, eframe bootstrap
    ├── app.rs               # Top-level App state + egui loop
    ├── config/
    │   ├── mod.rs           # API provider enum, ConfigStore
    │   └── crypto.rs        # AES-256-GCM encrypt/decrypt for API key
    ├── excel/
    │   ├── reader.rs        # calamine: parse .xlsx → Vec<Question>
    │   └── writer.rs        # rust_xlsxwriter: write output .xlsx
    ├── model.rs             # Question, ChoiceSet data types
    ├── rule_loader.rs       # Load .md files → String for LLM prompt
    ├── llm/
    │   ├── mod.rs           # LlmClient trait
    │   ├── gemini.rs        # Gemini REST implementation
    │   ├── openai.rs        # OpenAI (ChatGPT) REST implementation
    │   └── claude.rs        # Anthropic Claude REST implementation
    ├── generator.rs         # Orchestrate LLM call, parse response → choices
    └── ui/
        ├── settings_panel.rs  # Feature 1: API registration UI
        ├── import_panel.rs    # Feature 2: xlsx import UI
        ├── question_panel.rs  # Feature 3: per-question choice count + checkbox
        ├── rule_panel.rs      # Feature 4: rule .md load UI
        └── generate_panel.rs  # Feature 5: generate button + progress
```

---

### Implementation Phases

| Phase | Tasks |
|---|---|
| **P1** | Project scaffold, Cargo.toml, data model (`model.rs`) |
| **P2** | Config + crypto (API key store, AES-GCM) |
| **P3** | Excel reader (calamine) |
| **P4** | LLM client trait + 3 implementations |
| **P5** | Rule loader (default rules pre-loaded) |
| **P6** | Choice generator (prompt builder + LLM call + response parser) |
| **P7** | Excel writer (output .xlsx) |
| **P8** | GUI — settings panel |
| **P9** | GUI — import + question panel + rule panel |
| **P10** | GUI — generate panel, integration, end-to-end test |

---

### Key Design Decisions

- **API key encryption**: AES-256-GCM with a machine-derived key stored separately from the ciphertext. Config saved as `./outputs/config.enc`.
- **LLM prompt structure**: `[Rule content] + [Subject name] + [Question body] + [Model answer] + [N choices requested]` → ask LLM to return `N-1` distractors as JSON array.
- **Choice placement**: correct answer inserted at a random index among the N choices.
- **Default rules**: `./InputMaterials/test_development.md` and `./InputMaterials/test_guideline.md` are loaded at startup if no custom rules are set.

---

---

## 日本語版

### 概要

**Rust** + **egui/eframe** を使ったデスクトップGUIアプリケーションを構築します。以下の機能を実現します。

1. LLM APIキーの暗号化保存・管理
2. `.xlsx` ファイルからの設問データ取り込み
3. 設問ごとの選択肢数設定
4. `.md` ファイルからの選択肢生成ルール読み込み
5. LLMを使った誤答選択肢の生成と `.xlsx` への出力

---

### 技術スタック

| 用途 | クレート／ツール |
|---|---|
| GUI | `egui` + `eframe` |
| Excel読み込み | `calamine` |
| Excel書き出し | `rust_xlsxwriter` |
| HTTP / LLM通信 | `reqwest` + `tokio` |
| JSONシリアライズ | `serde` + `serde_json` |
| 設定ファイル保存 | `serde` + `toml` |
| APIキー暗号化 | `aes-gcm` + `rand` |
| Markdownファイル読み込み | 標準ライブラリ `std::fs`（テキストとしてLLMプロンプトへ渡す） |

---

### モジュール構成

```
questionComposition/
├── Cargo.toml
└── src/
    ├── main.rs              # エントリポイント、eframe起動
    ├── app.rs               # アプリ全体の状態管理 + egui描画ループ
    ├── config/
    │   ├── mod.rs           # APIプロバイダーの列挙型、設定ストア
    │   └── crypto.rs        # AES-256-GCMによるAPIキーの暗号化・復号
    ├── excel/
    │   ├── reader.rs        # calamine: .xlsx → Vec<Question> への変換
    │   └── writer.rs        # rust_xlsxwriter: 出力 .xlsx の生成
    ├── model.rs             # Question、ChoiceSet などのデータ型定義
    ├── rule_loader.rs       # .mdファイルの読み込み → LLMプロンプト用文字列
    ├── llm/
    │   ├── mod.rs           # LlmClientトレイト定義
    │   ├── gemini.rs        # Gemini REST実装
    │   ├── openai.rs        # OpenAI（ChatGPT）REST実装
    │   └── claude.rs        # Anthropic Claude REST実装
    ├── generator.rs         # LLM呼び出しの統括、レスポンス解析 → 選択肢生成
    └── ui/
        ├── settings_panel.rs  # 機能1: API登録画面
        ├── import_panel.rs    # 機能2: xlsx取り込み画面
        ├── question_panel.rs  # 機能3: 設問ごとの選択肢数設定 + チェックボックス
        ├── rule_panel.rs      # 機能4: ルール.md読み込み画面
        └── generate_panel.rs  # 機能5: 作問開始ボタン + 進捗表示
```

---

### 実装フェーズ

| フェーズ | 内容 |
|---|---|
| **P1** | プロジェクト雛形作成、Cargo.toml、データモデル定義（`model.rs`） |
| **P2** | 設定管理・暗号化（APIキー保存、AES-GCM実装） |
| **P3** | Excel読み込み（calamine） |
| **P4** | LLMクライアントトレイト定義 + 3プロバイダー実装 |
| **P5** | ルールローダー（デフォルトルール自動読み込み） |
| **P6** | 選択肢ジェネレーター（プロンプト構築 + LLM呼び出し + レスポンス解析） |
| **P7** | Excel書き出し（出力 .xlsx 生成） |
| **P8** | GUI — API設定パネル |
| **P9** | GUI — 取り込み・設問設定・ルール読み込みパネル |
| **P10** | GUI — 作問生成パネル、全体統合、エンドツーエンド確認 |

---

### 主要設計方針

- **APIキー暗号化**: AES-256-GCMを使用し、暗号文とは別にキーを管理。設定は `./outputs/config.enc` に保存
- **LLMプロンプト構造**: `[ルール内容] + [科目名] + [問題本文] + [模範解答] + [必要選択肢数]` → LLMに「N-1個の誤答選択肢をJSON配列で返せ」と指示
- **正答の配置**: N個の選択肢のうちランダムな位置に模範解答を挿入
- **デフォルトルール**: 起動時にカスタムルールが未設定であれば `./InputMaterials/test_development.md` と `./InputMaterials/test_guideline.md` を自動読み込み
