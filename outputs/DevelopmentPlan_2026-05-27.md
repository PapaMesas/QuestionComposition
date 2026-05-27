# Development Plan — Question Composition Application (Spec Revision: Requirements3add)

**Approval Date**: 2026-05-27  
**Base Plan**: DevelopmentPlan_2026-05-26.md

---

## English Version

### Overview of Changes

This plan covers the two spec additions introduced in `Requirements3add.md` relative to `Requirements2add.md`.

---

### Change 1 — Feature 5: Guideline Number Tracking

**Requirement addition**: For each question, explicitly state which numbered rule from `test_guideline.md` was used during choice generation.

**Current behavior**: The `guideline` field contains only the source file label (e.g., `"test_guideline.md"`).

**Required behavior**: The `guideline` field contains the specific guideline numbers that were applied (e.g., `"21, 27, 33"`).

#### Implementation

**File**: `src/generator.rs`

- Extend the LLM prompt to request a JSON object instead of a plain array:
  ```json
  {
    "distractors": ["誤答A", "誤答B", "誤答C"],
    "guideline_numbers": [21, 27, 33]
  }
  ```
- Update `parse_json_array` → replace with `parse_llm_response` that extracts both `distractors` and `guideline_numbers`
- Set `guideline` field to comma-separated numbers: `"21, 27, 33"`

**File**: `src/model.rs` — no structural change needed; `guideline: String` is sufficient.

---

### Change 2 — Feature 6: Export Structure Alignment

Each export handler must now strictly follow the sample structure defined in the corresponding `.md` file in `RequirementsSpecification/`.

#### 2-A: Moodle XML (`src/export/moodle.rs`)

Align with `Moodle XML.md`:

| Gap | Fix |
|---|---|
| Plain text in `<text>` | Wrap with `<![CDATA[...]]>` |
| Missing `<defaultgrade>` | Add `1.0000000` |
| Missing `<penalty>` | Add `0.3333333` |
| Missing `<hidden>` | Add `0` |
| Missing `<single>` | Add `true` |
| Missing `<shuffleanswers>` | Add `true` |
| Missing `<answernumbering>` | Add `abc` |
| `format="moodle_auto_format"` | Change to `format="html"` |
| Correct answer output separately | Output all choices in stored order; `fraction="100"` for correct, `"0"` for others |

#### 2-B: QTI 1.2 (`src/export/qti12.rs`)

Align with `QTI 1.2.md`:

| Gap | Fix |
|---|---|
| Outer `<questestinterop><assessment><section>` wrapper | Keep wrapper (correct for multi-item QTI 1.2) |
| `<presentation><flow>` | Change to `<presentation><material><mattext>` |
| `<response_lid>` → `<flow>` → `<response_label>` | Change to `<response_lid>` → `<render_choice shuffle="No">` → `<response_label>` |
| All choice idents same (`label_1`) | Assign A, B, C, D per choice |
| `respident="response_1"` in resprocessing | Change to `respident="RESPONSE"` |
| `maxvalue="100"`, score set to `100` | Change to `maxvalue="1"`, set to `1` |
| No `title` attribute on `<item>` | Add `title` using question body |

#### 2-C: QTI 2.1 (`src/export/qti21.rs`)

Align with `QTI 2.1.md`:

| Gap | Fix |
|---|---|
| All questions in one `<assessmentItem>` | One `<assessmentItem>` per question; wrap multiple in outer element |
| Missing `<responseDeclaration>` | Add with correct choice identifier |
| Missing `<outcomeDeclaration>` | Add with default value `0` |
| `<itemBody>` only | Add `<responseDeclaration>`, `<outcomeDeclaration>`, then `<itemBody>` |
| `shuffle="true"` | Change to `shuffle="false"` |
| Choice idents `choice_N` with correct answer as text | Assign `choice_1..N`; `<correctResponse><value>` contains the identifier of the correct choice |
| Custom `<responseProcessing>` block | Replace with template: `match_correct` |
| Wrong schemaLocation URL | Fix to match sample exactly |

#### 2-D: QTI 2.2 (`src/export/qti22.rs`)

Same structural fixes as QTI 2.1, with:
- Updated namespace: `http://www.imsglobal.org/xsd/imsqti_v2p2`
- Updated schemaLocation: `https://purl.imsglobal.org/spec/qti/v2p2/schema/xsd/imsqti_v2p2p4.xsd`
- Question text as `<p>` element before `<choiceInteraction>` inside `<itemBody>`
- Template URL: `https://www.imsglobal.org/question/qti_v2p2/rptemplates/match_correct`

#### 2-E: QTI 3.0 (`src/export/qti30.rs`)

Align with `QTI 3.0.md`:

| Gap | Fix |
|---|---|
| Root element `<assessmentItem>` with wrong namespace | Change root to `<qti-assessment-item>` with namespace `http://www.imsglobal.org/xsd/imsqtiasi_v3p0` |
| No `<qti-response-declaration>` | Add with correct identifier (A/B/C/D) |
| No `<qti-outcome-declaration>` | Add with default `0` |
| Element names without `qti-` prefix | Rename all: `qti-item-body`, `qti-choice-interaction`, `qti-simple-choice`, `qti-response-processing` |
| `response-identifier` not `responseIdentifier` | Use hyphenated attribute names per QTI 3.0 spec |
| Choice idents `choice_N` | Change to alphabetic: A, B, C, D |
| Custom responseProcessing | Replace with template URL |
| Wrong metadata block | Remove non-standard `<metadata>`; add `xml:lang="ja-JP"` to root |

---

### Implementation Phases

| Phase | File | Task |
|---|---|---|
| **P-A** | `src/generator.rs` | Extend prompt; add `parse_llm_response`; set guideline to numbers string |
| **P-B** | `src/export/moodle.rs` | CDATA wrapping; add missing elements; fix format attr; fix choice output order |
| **P-C** | `src/export/qti12.rs` | Fix presentation structure; fix response_lid; A-D idents; fix score range |
| **P-D** | `src/export/qti21.rs` | Rebuild per-item structure; add declarations; use template responseProcessing |
| **P-E** | `src/export/qti22.rs` | Same as P-D with QTI 2.2 namespace/URLs |
| **P-F** | `src/export/qti30.rs` | Rebuild with `qti-` prefix elements; A-D idents; template responseProcessing |
| **P-G** | All changed files | Update unit tests; run `cargo test` |

---

## 日本語版

### 変更の概要

`Requirements2add.md` に対する `Requirements3add.md` の差分として追加された2点の仕様変更を実装します。

---

### 変更1 — 機能5: ガイドライン番号の明示

**仕様追加内容**: 設問ごとに、`test_guideline.md` の何番のルールを使用して選択肢を生成したかを明示する。

**現状**: `guideline` フィールドにはソースファイルのラベル（例: `"test_guideline.md"`）のみ格納される。

**変更後**: `guideline` フィールドに適用したガイドライン番号を格納する（例: `"21, 27, 33"`）。

#### 実装内容

**ファイル**: `src/generator.rs`

- LLMプロンプトを変更し、JSON配列ではなくJSONオブジェクトで応答を要求する:
  ```json
  {
    "distractors": ["誤答A", "誤答B", "誤答C"],
    "guideline_numbers": [21, 27, 33]
  }
  ```
- `parse_json_array` を `parse_llm_response` に置き換え、`distractors` と `guideline_numbers` の両方を抽出する
- `guideline` フィールドをカンマ区切りの番号文字列で設定: `"21, 27, 33"`

**ファイル**: `src/model.rs` — 構造変更不要。`guideline: String` で対応可能。

---

### 変更2 — 機能6: エクスポート構造の仕様定義ファイルへの準拠

各エクスポートハンドラーは、`RequirementsSpecification/` に配置された対応する `.md` ファイルで定義されたサンプル構造に厳密に準拠する。

#### 2-A: Moodle XML (`src/export/moodle.rs`)

`Moodle XML.md` に準拠するための修正:

| 現状の問題 | 修正内容 |
|---|---|
| `<text>` 内がプレーンテキスト | `<![CDATA[...]]>` で囲む |
| `<defaultgrade>` が欠落 | `1.0000000` を追加 |
| `<penalty>` が欠落 | `0.3333333` を追加 |
| `<hidden>` が欠落 | `0` を追加 |
| `<single>` が欠落 | `true` を追加 |
| `<shuffleanswers>` が欠落 | `true` を追加 |
| `<answernumbering>` が欠落 | `abc` を追加 |
| `format="moodle_auto_format"` | `format="html"` に変更 |
| 正答と誤答を分けて出力 | すべての選択肢を格納順に出力。正答は `fraction="100"`、誤答は `fraction="0"` |

#### 2-B: QTI 1.2 (`src/export/qti12.rs`)

`QTI 1.2.md` に準拠するための修正:

| 現状の問題 | 修正内容 |
|---|---|
| 外部ラッパー構造 | `<questestinterop><assessment><section>` は保持（QTI 1.2 の複数問題対応として正しい構造） |
| `<presentation><flow>` | `<presentation><material><mattext>` に変更 |
| `<response_lid>` 内が `<flow>` | `<render_choice shuffle="No">` に変更 |
| 全選択肢の ident が同じ (`label_1`) | 選択肢ごとに A, B, C, D を割り当て |
| `respident="response_1"` | `respident="RESPONSE"` に変更 |
| `maxvalue="100"`、スコア `100` | `maxvalue="1"`、スコア `1` に変更 |
| `<item>` に `title` 属性がない | 問題本文を使って `title` 属性を追加 |

#### 2-C: QTI 2.1 (`src/export/qti21.rs`)

`QTI 2.1.md` に準拠するための修正:

| 現状の問題 | 修正内容 |
|---|---|
| 全問題が1つの `<assessmentItem>` | 設問ごとに `<assessmentItem>` を生成 |
| `<responseDeclaration>` が欠落 | 正答選択肢の識別子を参照する `<responseDeclaration>` を追加 |
| `<outcomeDeclaration>` が欠落 | デフォルト値 `0` の `<outcomeDeclaration>` を追加 |
| `shuffle="true"` | `shuffle="false"` に変更 |
| `<responseProcessing>` を独自実装 | `match_correct` テンプレートに変更 |
| schemaLocation の URL が不正確 | サンプルに合わせて修正 |

#### 2-D: QTI 2.2 (`src/export/qti22.rs`)

QTI 2.1 と同様の構造修正に加え:
- 名前空間: `http://www.imsglobal.org/xsd/imsqti_v2p2`
- schemaLocation: `https://purl.imsglobal.org/spec/qti/v2p2/schema/xsd/imsqti_v2p2p4.xsd`
- `<itemBody>` 内で `<choiceInteraction>` の前に `<p>` で問題本文を記載
- テンプレート URL: `https://www.imsglobal.org/question/qti_v2p2/rptemplates/match_correct`

#### 2-E: QTI 3.0 (`src/export/qti30.rs`)

`QTI 3.0.md` に準拠するための修正:

| 現状の問題 | 修正内容 |
|---|---|
| ルート要素が `<assessmentItem>` で名前空間が誤り | `<qti-assessment-item>` に変更し名前空間を `http://www.imsglobal.org/xsd/imsqtiasi_v3p0` に修正 |
| `<qti-response-declaration>` が欠落 | 正答識別子 (A/B/C/D) を参照する要素を追加 |
| `<qti-outcome-declaration>` が欠落 | デフォルト値 `0` の要素を追加 |
| 要素名に `qti-` プレフィックスがない | `qti-item-body`、`qti-choice-interaction`、`qti-simple-choice`、`qti-response-processing` に変更 |
| 属性名がキャメルケース | ハイフン区切り属性名に変更（例: `response-identifier`、`max-choices`） |
| 選択肢 ident が `choice_N` | アルファベット (A, B, C, D) に変更 |
| 独自 responseProcessing | テンプレート URL に変更 |
| 非標準の `<metadata>` ブロック | 削除。代わりにルートに `xml:lang="ja-JP"` を追加 |

---

### 実装フェーズ

| フェーズ | ファイル | 内容 |
|---|---|---|
| **P-A** | `src/generator.rs` | プロンプト変更・`parse_llm_response` 実装・ガイドライン番号文字列の設定 |
| **P-B** | `src/export/moodle.rs` | CDATA 対応・欠落要素の追加・フォーマット属性修正・選択肢出力順の修正 |
| **P-C** | `src/export/qti12.rs` | presentation 構造修正・response_lid 修正・A-D ident・スコア範囲修正 |
| **P-D** | `src/export/qti21.rs` | 設問ごとの assessmentItem 構造に再構築・宣言要素の追加・テンプレート responseProcessing |
| **P-E** | `src/export/qti22.rs` | P-D と同様（QTI 2.2 名前空間・URL 適用） |
| **P-F** | `src/export/qti30.rs` | `qti-` プレフィックス要素で再構築・A-D ident・テンプレート responseProcessing |
| **P-G** | 変更した全ファイル | ユニットテスト更新・`cargo test` 実行 |
