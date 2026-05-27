# Development Plan — Question Composition Application (v2)

**Base Approval Date**: 2026-04-30  
**Updated**: 2026-05-26

---

## English Version

### Overview

A desktop GUI application built in **Rust** using **egui/eframe** that:
1. Manages LLM API credentials (encrypted)
2. Imports question data from `.xlsx`
3. Configures choices per question
4. Loads choice-generation rules from `.md` files
5. Generates distractor choices via LLM and exports with **guideline tracking**
6. **[NEW]** Exports to multiple formats: Moodle XML, QTI 1.2, QTI 2.1, QTI 2.2, QTI 3.0

---

### Key Changes from Original Plan

#### Feature 5 — Enhanced Output
**Before**: Output columns: `No, 問題番号, 問題本文, 模範解答, 選択肢数, 選択肢1, 選択肢2, ...`

**After**: Output columns: `No, 問題番号, 問題本文, 模範解答, 選択肢数, 選択肢1, 選択肢2, ..., ガイドライン`
- **Implementation**: Track which guideline rule was used during generation and include in output

#### Feature 6 — Output Format Selection [NEW]
- User selects export format:
  - Moodle XML
  - QTI 1.2
  - QTI 2.1
  - QTI 2.2
  - QTI 3.0
- User specifies save location and filename
- Each format handler validates data before output

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
| Markdown read | `std::fs` |
| XML generation | `quick-xml` |
| File dialog | `rfd` (native file picker) |

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
    ├── model.rs             # Question, ChoiceSet, GuidelineInfo data types
    ├── rule_loader.rs       # Load .md files → String for LLM prompt
    ├── llm/
    │   ├── mod.rs           # LlmClient trait
    │   ├── gemini.rs        # Gemini REST implementation
    │   ├── openai.rs        # OpenAI (ChatGPT) REST implementation
    │   └── claude.rs        # Anthropic Claude REST implementation
    ├── generator.rs         # Orchestrate LLM call, parse response → choices + guideline
    ├── export/              # [NEW] Output format handlers
    │   ├── mod.rs           # ExportFormat enum, ExportHandler trait
    │   ├── moodle.rs        # Moodle XML export
    │   ├── qti12.rs         # QTI 1.2 export
    │   ├── qti21.rs         # QTI 2.1 export
    │   ├── qti22.rs         # QTI 2.2 export
    │   └── qti30.rs         # QTI 3.0 export
    └── ui/
        ├── settings_panel.rs  # Feature 1: API registration UI
        ├── import_panel.rs    # Feature 2: xlsx import UI
        ├── question_panel.rs  # Feature 3: per-question choice count + checkbox
        ├── rule_panel.rs      # Feature 4: rule .md load UI
        ├── generate_panel.rs  # Feature 5: generate button + progress + guideline display
        └── export_panel.rs    # [NEW] Feature 6: format selection + file save dialog
```

---

### Implementation Phases

| Phase | Tasks |
|---|---|
| **P1** | Project scaffold, Cargo.toml, data model with `GuidelineInfo` field |
| **P2** | Config + crypto (API key store, AES-GCM) |
| **P3** | Excel reader (calamine) |
| **P4** | LLM client trait + 3 implementations |
| **P5** | Rule loader (default rules pre-loaded) |
| **P6** | Choice generator + guideline tracking (return which rule was used) |
| **P7** | **[UPDATE]** Excel writer with "ガイドライン" column |
| **P8** | Export module: base trait + format enum |
| **P9** | Moodle XML export implementation |
| **P10** | QTI 1.2 export implementation |
| **P11** | QTI 2.1 export implementation |
| **P12** | QTI 2.2 export implementation |
| **P13** | QTI 3.0 export implementation |
| **P14** | GUI — settings panel |
| **P15** | GUI — import + question panel + rule panel |
| **P16** | GUI — generate panel with guideline display |
| **P17** | GUI — export panel with format selection + file dialog |
| **P18** | Full integration + E2E testing |

---

### Key Design Decisions

- **Guideline tracking**: `generator.rs` returns a tuple `(choices, guideline_name)` to track which rule was applied
- **Export architecture**: Abstract `ExportHandler` trait allowing new formats to be added independently
- **File dialog**: Use `rfd` crate for native OS file picker (cross-platform support)
- **Output validation**: Each format handler validates question data before attempting export
- **Backwards compatibility**: Original `.xlsx` export remains available in Feature 5
- **QTI compliance**: Reference standard documentation at `https://www.1edtech.org/standards/qti/index` for correct XML structure per version

---

---

## 日本語版

### 概要

**Rust** + **egui/eframe** を使ったデスクトップGUIアプリケーションを構築します。以下の機能を実現します。

1. LLM APIキーの暗号化保存・管理
2. `.xlsx` ファイルからの設問データ取り込み
3. 設問ごとの選択肢数設定
4. `.md` ファイルからの選択肢生成ルール読み込み
5. LLMを使った誤答選択肢の生成と `.xlsx` への出力（**ガイドライン情報を含む**）
6. **[新規]** 複数フォーマットへのエクスポート: Moodle XML、QTI 1.2、QTI 2.1、QTI 2.2、QTI 3.0

---

### 元の計画からの主要な変更点

#### 機能5 — 出力形式の拡張
**変更前**: 出力列: `No, 問題番号, 問題本文, 模範解答, 選択肢数, 選択肢1, 選択肢2, ...`

**変更後**: 出力列: `No, 問題番号, 問題本文, 模範解答, 選択肢数, 選択肢1, 選択肢2, ..., ガイドライン`
- **実装方針**: 生成時にどのガイドラインルールが使用されたかを追跡し、出力に含める

#### 機能6 — 出力形式の選択機能 [新規]
- ユーザーが以下のエクスポート形式から選択:
  - Moodle XML
  - QTI 1.2
  - QTI 2.1
  - QTI 2.2
  - QTI 3.0
- 保存先フォルダとファイル名を指定できる
- 各形式のハンドラーは出力前にデータを検証

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
| Markdownファイル読み込み | `std::fs` |
| XML生成 | `quick-xml` |
| ファイルダイアログ | `rfd` (ネイティブファイルピッカー) |

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
    ├── model.rs             # Question、ChoiceSet、GuidelineInfo などのデータ型定義
    ├── rule_loader.rs       # .mdファイルの読み込み → LLMプロンプト用文字列
    ├── llm/
    │   ├── mod.rs           # LlmClientトレイト定義
    │   ├── gemini.rs        # Gemini REST実装
    │   ├── openai.rs        # OpenAI（ChatGPT）REST実装
    │   └── claude.rs        # Anthropic Claude REST実装
    ├── generator.rs         # LLM呼び出しの統括、レスポンス解析 → 選択肢+ガイドライン生成
    ├── export/              # [新規] 出力形式ハンドラー
    │   ├── mod.rs           # ExportFormat列挙型、ExportHandlerトレイト
    │   ├── moodle.rs        # Moodle XML エクスポート
    │   ├── qti12.rs         # QTI 1.2 エクスポート
    │   ├── qti21.rs         # QTI 2.1 エクスポート
    │   ├── qti22.rs         # QTI 2.2 エクスポート
    │   └── qti30.rs         # QTI 3.0 エクスポート
    └── ui/
        ├── settings_panel.rs  # 機能1: API登録画面
        ├── import_panel.rs    # 機能2: xlsx取り込み画面
        ├── question_panel.rs  # 機能3: 設問ごとの選択肢数設定 + チェックボックス
        ├── rule_panel.rs      # 機能4: ルール.md読み込み画面
        ├── generate_panel.rs  # 機能5: 作問開始ボタン + 進捗表示 + ガイドライン表示
        └── export_panel.rs    # [新規] 機能6: 形式選択 + ファイル保存ダイアログ
```

---

### 実装フェーズ

| フェーズ | 内容 |
|---|---|
| **P1** | プロジェクト雛形作成、Cargo.toml、データモデル定義（`GuidelineInfo`フィールド追加） |
| **P2** | 設定管理・暗号化（APIキー保存、AES-GCM実装） |
| **P3** | Excel読み込み（calamine） |
| **P4** | LLMクライアントトレイト定義 + 3プロバイダー実装 |
| **P5** | ルールローダー（デフォルトルール自動読み込み） |
| **P6** | 選択肢ジェネレーター + ガイドライン追跡（使用されたルール情報を返す） |
| **P7** | **[更新]** Excel書き出し（「ガイドライン」列を追加） |
| **P8** | エクスポートモジュール: 基本トレイト + 形式列挙型 |
| **P9** | Moodle XML エクスポート実装 |
| **P10** | QTI 1.2 エクスポート実装 |
| **P11** | QTI 2.1 エクスポート実装 |
| **P12** | QTI 2.2 エクスポート実装 |
| **P13** | QTI 3.0 エクスポート実装 |
| **P14** | GUI — API設定パネル |
| **P15** | GUI — 取り込み・設問設定・ルール読み込みパネル |
| **P16** | GUI — 作問生成パネル（ガイドライン表示を含む） |
| **P17** | GUI — エクスポートパネル（形式選択 + ファイルダイアログ） |
| **P18** | 全体統合 + エンドツーエンド確認 |

---

### 主要設計方針

- **ガイドライン追跡**: `generator.rs` は `(choices, guideline_name)` のタプルを返し、どのルールが適用されたかを追跡
- **エクスポート・アーキテクチャ**: 抽象的な `ExportHandler` トレイトにより、新しい形式を独立して追加可能
- **ファイルダイアログ**: `rfd` クレートを使用してネイティブOSのファイルピッカーを実装（クロスプラットフォーム対応）
- **出力検証**: 各形式のハンドラーはエクスポート前に設問データを検証
- **後方互換性**: 元の `.xlsx` エクスポートは機能5で継続利用可能
- **QTI準拠**: `https://www.1edtech.org/standards/qti/index` の標準ドキュメントを参照し、バージョン別に正確なXML構造を実装
