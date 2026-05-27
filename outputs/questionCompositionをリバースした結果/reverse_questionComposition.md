# questionComposition — リバース分析レポート

**対象パス**: `VibeCoding/RUST/QuestionComposition/questionComposition`  
**分析日**: 2026-05-11
**概要**: code-explorer (agent) を使用して、対象パスをリバース

---

## 1. アプリケーション概要

**試験問題の選択肢自動生成ツール**。Excel ファイルから問題本文・正解を読み込み、LLM（Gemini / OpenAI / Claude）に誤答選択肢（ディストラクター）を生成させ、結果を Excel に書き出すデスクトップ GUI アプリケーション。

### 5ステップのワークフロー（タブ構成に対応）

| ステップ | タブ名 | 内容 |
|---------|--------|------|
| 1 | API設定 | LLM プロバイダーと API キーを設定・暗号化保存 |
| 2 | 設問取り込み | .xlsx ファイルから問題データを読み込む |
| 3 | 選択肢数設定 | 問題ごとに生成する選択肢の数（2〜8）を調整 |
| 4 | ルール設定 | 選択肢生成ガイドライン（.md ファイル）を読み込む |
| 5 | 作問生成 | LLM を呼び出して誤答を生成し、.xlsx として保存 |

---

## 2. ディレクトリ構造

```
questionComposition/
├── Cargo.toml
├── assets/
│   └── NotoSansJP-Medium.ttf      # バイナリに埋め込まれる日本語フォント
├── src/
│   ├── main.rs                    # エントリポイント
│   ├── lib.rs                     # クレートルート（統合テスト用）
│   ├── app.rs                     # AppState・App・Tab 定義、egui メインループ
│   ├── model.rs                   # データ型定義
│   ├── generator.rs               # LLM 呼び出し・選択肢生成ロジック
│   ├── rule_loader.rs             # ルール .md ファイルの読み込み
│   ├── config/
│   │   ├── mod.rs                 # 設定の保存・読み込み
│   │   └── crypto.rs              # AES-256-GCM 暗号化・復号
│   ├── excel/
│   │   ├── mod.rs
│   │   ├── reader.rs              # calamine で .xlsx 読み込み
│   │   └── writer.rs              # rust_xlsxwriter で .xlsx 書き出し
│   ├── llm/
│   │   ├── mod.rs                 # LlmClient トレイト・ファクトリ関数
│   │   ├── gemini.rs              # Google Gemini 1.5 Flash
│   │   ├── openai.rs              # OpenAI gpt-4o-mini
│   │   └── claude.rs              # Anthropic claude-haiku-4-5
│   └── ui/
│       ├── mod.rs
│       ├── settings_panel.rs      # ① API設定パネル
│       ├── import_panel.rs        # ② 設問取り込みパネル
│       ├── question_panel.rs      # ③ 選択肢数設定パネル
│       ├── rule_panel.rs          # ④ ルール設定パネル
│       └── generate_panel.rs      # ⑤ 作問生成パネル
└── tests/
    └── integration_test.rs        # 統合テスト（IT-01, IT-02）
```

親ディレクトリ `/QuestionComposition/` には `InputMaterials/`（デフォルトルール .md）と `outputs/`（config.toml・出力 .xlsx）が配置される。

---

## 3. 主要モジュールと役割

| モジュール | 役割 |
|-----------|------|
| `app` | アプリ全体の状態（`AppState`）と egui の描画ループ。タブ切り替えと各パネルへのディスパッチ |
| `model` | `Question`、`QuestionWithChoices`、`QuestionSheet`、`LlmProvider` の定義。アプリ全体のデータ契約 |
| `generator` | LLM へのプロンプト構築・呼び出し・JSON レスポンス解析・正答のランダム挿入 |
| `rule_loader` | ルール .md ファイルの読み込み。デフォルトルール（`../InputMaterials/`）とカスタムルールの両対応 |
| `config/mod` | `AppConfig` の TOML シリアライズ・デシリアライズ。`../outputs/config.toml` へ永続化 |
| `config/crypto` | AES-256-GCM による API キーの暗号化・復号。機械固有のキーを SHA-256 で導出 |
| `excel/reader` | calamine を使った .xlsx 読み込み。行フォーマット仕様に従い `QuestionSheet` へ変換 |
| `excel/writer` | rust_xlsxwriter を使った .xlsx 書き出し。選択肢列数を動的に生成 |
| `llm/mod` | `LlmClient` トレイトと `create_client` ファクトリ。プロバイダーの切り替えを一元管理 |
| `llm/{gemini,openai,claude}` | 各 REST API の具体実装。`reqwest::blocking::Client` でブロッキング HTTP |
| `ui/*_panel` | egui を使った各タブパネルの描画。`AppState` への参照を受け取り状態を直接変更 |

---

## 4. データモデル

| 型 | 概要 |
|----|------|
| `Question` | Excel から読み込んだ1問分のデータ（no, question_no, body, correct_answer, num_choices） |
| `QuestionWithChoices` | LLM 生成後の1問分（`Question` の各フィールド + `choices: Vec<String>`） |
| `QuestionSheet` | 科目名（`subject`）と `Vec<Question>` のコンテナ |
| `LlmProvider` | `Gemini` / `OpenAI` / `Claude` の列挙型。`Default = Gemini`。`Display` 実装でUI表示文字列を持つ |
| `AppConfig` | 設定ファイル対応構造体。`provider: LlmProvider` と `encrypted_api_key: Option<String>` |
| `RuleSet` | ルール本文（`content`）とソースラベル（`source_label`）のペア |
| `AppState` | 全パネル横断の共有状態。設定・取り込み・生成それぞれの進捗・メッセージを保持 |

---

## 5. 主要な処理フロー

### エントリポイント → GUI 起動

```
main()
  → eframe::run_native("設問作成支援ツール", 1024x768)
  → App::new(cc)
      → NotoSansJP フォントをバイナリから egui に登録
      → AppState::new()
            → config::load() で ../outputs/config.toml を読む
            → RuleSet::default() で InputMaterials/ のルールを読む
  → eframe::App::update() ループ
```

### タブ ⑤「作問開始」クリック時

```
generate_panel::show()
  → run_generation(&mut state)
      → config::load_api_key(&state.config)   // AES-256-GCM 復号
      → llm::create_client(provider, api_key) // Box<dyn LlmClient> を生成
      → 各 Question に対してループ:
            generator::generate_choices(client, subject, question, rules)
              → システムプロンプト構築（rules.content を埋め込む）
              → ユーザーメッセージ構築（問題情報 + JSON 配列指定）
              → client.complete(request)       // 同期 HTTP 呼び出し
              → parse_json_array(response)     // "[" から "]" を抽出してパース
              → insert_correct_at_random()     // 正答をランダム位置に挿入
              → QuestionWithChoices を返す
      → state.generated_questions に蓄積
  → save_output(&mut state)
      → rfd::FileDialog でファイル保存ダイアログ
      → excel::writer::write_xlsx(path, subject, questions)
```

---

## 6. 依存クレート

| クレート | 用途 |
|---------|------|
| `eframe` / `egui` 0.27 | デスクトップ GUI フレームワーク |
| `calamine` 0.24 | .xlsx 読み込み |
| `rust_xlsxwriter` 0.68 | .xlsx 書き出し |
| `tokio` 1 | 非同期ランタイム（`features = ["full"]`、実際には未使用） |
| `reqwest` 0.12 | HTTP クライアント（ブロッキング API を使用） |
| `serde` / `serde_json` | JSON シリアライズ・デシリアライズ |
| `toml` 0.8 | TOML 設定ファイル読み書き |
| `aes-gcm` 0.10 | AES-256-GCM 暗号化 |
| `rand` 0.8 | 正答挿入位置のランダム生成 |
| `base64` 0.22 | 暗号文の TOML 保存用エンコード |
| `anyhow` 1 | アプリケーションエラー処理 |
| `sha2` 0.10 | ホスト名から暗号化キーを SHA-256 で導出 |
| `rfd` 0.14 | ネイティブファイル選択ダイアログ |
| `chrono` 0.4 | 出力ファイル名の日時フォーマット |
| `egui-file-dialog` 0.5 | ファイルダイアログ（未使用、`rfd` で代替） |
| `tempfile` 3 (dev) | テスト用一時ファイル |

### 内部依存関係の方向

```
main / ui/* → app::AppState
app → config, model, rule_loader
generator → llm::LlmClient (トレイト), model, rule_loader
llm/mod → llm/{gemini,openai,claude}
config/mod → config/crypto
excel/reader, writer → model
```

---

## 7. 設計上の特徴

### LlmClient トレイト（ストラテジーパターン）

`llm/mod.rs` にて `LlmClient: Send + Sync` トレイトを定義し、`create_client` ファクトリで `Box<dyn LlmClient>` を返す。`generator.rs` はプロバイダー実装を一切知らず、トレイト境界だけに依存している。テストで `MockClient` をインライン定義して差し替えられる設計になっている。

### 機械固有の暗号化キー導出

`config/crypto.rs` はホスト名 + 固定ソルトを SHA-256 にかけて 32 バイトの AES キーを導出する。`HOSTNAME` 環境変数 → `/etc/hostname` → フォールバック文字列の順でホスト名を取得。config.toml を別マシンにコピーしても復号できない設計になっている。

### フォント埋め込み

```rust
static NOTO_SANS_JP: &[u8] = include_bytes!("../assets/NotoSansJP-Medium.ttf");
```

日本語フォントをコンパイル時にバイナリに埋め込む。配布時に外部フォントファイルが不要。

### LLM レスポンスのロバスト解析

`generator::parse_json_array` は `find("[")` / `rfind("]")` で抽出するため、LLM が説明文やコードブロック（` ```json ``` `）を付加しても JSON 配列を正しく取り出せる。

### テスト設計

ユニットテストはモジュール内 `#[cfg(test)]` に配置（UT-01〜UT-18）、統合テストは `tests/integration_test.rs`（IT-01〜IT-02）と明確に分離。テスト ID が `TestSpecification.md` と対応する形で管理されている。

---

## 8. 改善余地のある箇所

### 🔴 重大: UI フリーズ（最優先）

`generate_panel.rs` の `run_generation` は egui の描画スレッドで全設問に対してループしながら同期 HTTP 呼び出しを行う。問題数が増えると UI が完全にフリーズする。コメント内にも「スレッド化を検討」と明記されている。`tokio` は依存に含まれているが活用されていない。

**推奨対応**: `std::thread::spawn` + `mpsc::channel` で生成処理をバックグラウンドスレッドに移し、進捗を `Sender` で描画スレッドに通知する。

### 🟡 中: 不要な依存クレート

- `egui-file-dialog`（未使用、`rfd` で代替済み）
- `tokio`（`features = ["full"]` で依存しているが実際には未使用）

どちらもビルド時間とバイナリサイズを増加させている。

### 🟡 中: 進捗表示が実際には更新されない

`state.generation_progress` はループ内でインクリメントされるが、egui はフレームが返らないと再描画しない。プログレスカウンターは生成完了後にしか反映されない。UI フリーズの解消と合わせて対処が必要。

### 🟡 中: Excel writer のエラー無視

`excel/writer.rs` の多くの `write_*` 呼び出しで `.ok()` を使っており、セルの書き込みエラーが黙って無視される。`?` 演算子で伝播させ、呼び出し元でユーザーにエラー表示すべき。

### 🟡 中: ルールファイルパスのハードコード

`rule_loader.rs` の `DEFAULT_RULE_PATHS` が `"../InputMaterials/..."` という相対パスで固定。実行カレントディレクトリ依存になっており、異なる場所から実行するとデフォルトルールが読めず内蔵フォールバックに切り替わる。

### 🟢 軽微: Excel writer のテスト未整備

`excel/writer.rs` にユニットテストが存在しない。統合テスト IT-02 でファイルサイズが 0 バイトでないことのみ確認しているが、書き出した内容（科目名・選択肢テキスト）の値検証は行われていない。
