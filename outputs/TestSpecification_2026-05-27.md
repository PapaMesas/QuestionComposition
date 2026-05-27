# Test Specification — Question Composition Application (Spec Revision: Requirements3add)

**Date**: 2026-05-27  
**Base Spec**: TestSpecification_2026-05-26.md

---

## English Version

### Overview

Additional test cases covering the two specification changes in `Requirements3add.md`:
1. Feature 5 — The `guideline` column must contain specific guideline numbers from `test_guideline.md`
2. Feature 6 — All export formats must strictly conform to the structure defined in their respective `.md` sample files

---

### Feature 5 — Guideline Number Tracking

#### TS-5-3: Guideline Column Contains Specific Numbers
- **Objective**: Verify that the "ガイドライン" column contains numbered guideline rules (e.g., "21, 27, 33") not just a file name
- **Steps**:
  1. Import test questions from InputMaterials/テスト用.xlsx
  2. Configure choice count (default 4)
  3. Check the confirmation checkbox
  4. Click "作問開始"
  5. Export to Excel
- **Expected Result**: The "ガイドライン" column contains comma-separated integer guideline numbers from test_guideline.md

#### TS-5-4: LLM Response JSON Object Parsed Correctly (Unit)
- **Objective**: Verify `parse_llm_response` correctly extracts `distractors` and `guideline_numbers` from JSON object format
- **Test IDs**: UT-11, UT-12 (new tests in generator.rs)
- **Expected Result**: All unit tests in `generator::tests` pass

#### TS-5-5: Fallback to Array Format
- **Objective**: Verify that when LLM returns a plain JSON array (no guideline numbers), the system handles it gracefully
- **Test ID**: UT-13
- **Expected Result**: Choices are generated normally; guideline field is empty string; no error thrown

---

### Feature 6 — Export Format Structure Compliance

#### TS-6-1: Moodle XML Structure

##### TS-6-1-1: CDATA Wrapping
- **Objective**: Verify question text and answer text are wrapped in CDATA sections
- **Expected Result**: `<text><![CDATA[...]]></text>` pattern present for all text fields

##### TS-6-1-2: Required Moodle Fields
- **Objective**: Verify presence of all required fields per Moodle XML.md
- **Expected Result**: XML contains `<defaultgrade>1.0000000</defaultgrade>`, `<penalty>0.3333333</penalty>`, `<hidden>0</hidden>`, `<single>true</single>`, `<shuffleanswers>true</shuffleanswers>`, `<answernumbering>abc</answernumbering>`

##### TS-6-1-3: Answer Format Attribute
- **Objective**: Verify answer elements use `format="html"` not `format="moodle_auto_format"`
- **Expected Result**: All `<answer>` elements have `format="html"`

##### TS-6-1-4: Correct Answer Fraction
- **Objective**: Verify correct answer has `fraction="100"` and distractors have `fraction="0"`
- **Expected Result**: Exactly one answer with `fraction="100"` per question; that answer matches the correct_answer value

#### TS-6-2: QTI 1.2 Structure

##### TS-6-2-1: Render Choice Wrapper
- **Objective**: Verify `<render_choice shuffle="No">` is used inside `<response_lid>`
- **Expected Result**: XML contains `<render_choice shuffle="No">` element

##### TS-6-2-2: Alphabetic Choice Identifiers
- **Objective**: Verify choices use A, B, C, D identifiers not `label_1`
- **Expected Result**: `<response_label ident="A">`, `<response_label ident="B">` etc.

##### TS-6-2-3: Correct Answer by Alphabetic Ident
- **Objective**: Verify `<varequal respident="RESPONSE">` references the alphabetic ident of the correct choice
- **Expected Result**: `<varequal respident="RESPONSE">B</varequal>` (or correct letter) matches correct answer position

##### TS-6-2-4: Score Range 0-1
- **Objective**: Verify SCORE range is 0 to 1 per sample
- **Expected Result**: `maxvalue="1"` and `<setvar>1</setvar>`

#### TS-6-3: QTI 2.1 Structure

##### TS-6-3-1: responseDeclaration and outcomeDeclaration Present
- **Objective**: Verify both declarations exist per question
- **Expected Result**: `<responseDeclaration identifier="RESPONSE">` and `<outcomeDeclaration identifier="SCORE">` present

##### TS-6-3-2: Correct Choice ID in responseDeclaration
- **Objective**: Verify `<value>` in `<correctResponse>` references the correct `choice_N` identifier
- **Expected Result**: If correct answer is at position index 1, `<value>choice_2</value>`

##### TS-6-3-3: shuffle="false"
- **Objective**: Verify shuffle is false per sample
- **Expected Result**: `<choiceInteraction ... shuffle="false">`

##### TS-6-3-4: Template responseProcessing
- **Objective**: Verify template URL is used
- **Expected Result**: `template="http://www.imsglobal.org/question/qti_v2p1/rptemplates/match_correct"`

#### TS-6-4: QTI 2.2 Structure

##### TS-6-4-1: Correct Namespace and Schema Location
- **Objective**: Verify QTI 2.2 namespace and purl.imsglobal.org schema location
- **Expected Result**: `xmlns="http://www.imsglobal.org/xsd/imsqti_v2p2"` and `purl.imsglobal.org/spec/qti/v2p2` in xsi:schemaLocation

##### TS-6-4-2: Question Text as `<p>` Element
- **Objective**: Verify question body is in `<p>` tag before `<choiceInteraction>` inside `<itemBody>`
- **Expected Result**: `<p>` element appears before `<choiceInteraction>` in the XML

##### TS-6-4-3: QTI 2.2 Template URL
- **Objective**: Verify QTI 2.2-specific template URL
- **Expected Result**: `template="https://www.imsglobal.org/question/qti_v2p2/rptemplates/match_correct"`

#### TS-6-5: QTI 3.0 Structure

##### TS-6-5-1: qti- Prefixed Root Element
- **Objective**: Verify root element is `<qti-assessment-item>` not `<assessmentItem>`
- **Expected Result**: XML starts with `<qti-assessment-item`

##### TS-6-5-2: QTI 3.0 Namespace
- **Objective**: Verify namespace is `http://www.imsglobal.org/xsd/imsqtiasi_v3p0`
- **Expected Result**: namespace present in root element

##### TS-6-5-3: qti- Prefixed Elements
- **Objective**: Verify all QTI elements use qti- prefix
- **Expected Result**: `<qti-response-declaration>`, `<qti-outcome-declaration>`, `<qti-item-body>`, `<qti-choice-interaction>`, `<qti-simple-choice>`, `<qti-response-processing>` present

##### TS-6-5-4: Alphabetic Choice Identifiers
- **Objective**: Verify choices use A, B, C, D identifiers
- **Expected Result**: `identifier="A"`, `identifier="B"` etc.

##### TS-6-5-5: Hyphenated Attributes
- **Objective**: Verify QTI 3.0 attribute names use hyphens not camelCase
- **Expected Result**: `response-identifier="RESPONSE"`, `max-choices="1"`

##### TS-6-5-6: xml:lang Attribute
- **Objective**: Verify `xml:lang="ja-JP"` on root element
- **Expected Result**: `xml:lang="ja-JP"` present

##### TS-6-5-7: QTI 3.0 Template URL
- **Objective**: Verify purl.imsglobal.org QTI 3.0 template URL
- **Expected Result**: `template="https://purl.imsglobal.org/spec/qti/v3p0/rptemplates/match_correct"`

---

### Unit Test Coverage

All new test cases are implemented in the respective module test blocks:

| File | Tests Added |
|---|---|
| `src/generator.rs` | UT-11 to UT-17 (7 tests) |
| `src/export/moodle.rs` | 4 structural compliance tests |
| `src/export/qti12.rs` | 6 structural compliance tests |
| `src/export/qti21.rs` | 6 structural compliance tests |
| `src/export/qti22.rs` | 5 structural compliance tests |
| `src/export/qti30.rs` | 8 structural compliance tests |

**Total tests after this revision**: 53 (51 unit + 2 integration) — all passing.

---

## 日本語版

### 概要

`Requirements3add.md` の2点の仕様変更に対応する追加テスト仕様です。
1. 機能5 — `guideline` 列に `test_guideline.md` の具体的なガイドライン番号を格納すること
2. 機能6 — 各エクスポート形式が対応するサンプルファイルで定義された構造に準拠すること

---

### 機能5 — ガイドライン番号の明示

#### TS-5-3: ガイドライン列に具体的な番号が格納される
- **目的**: 「ガイドライン」列にファイル名ではなく番号（例: "21, 27, 33"）が格納されることを確認する
- **手順**:
  1. InputMaterials/テスト用.xlsx から設問を取り込む
  2. 選択肢数を設定する（デフォルト4）
  3. 設定完了チェックボックスにチェックを入れる
  4. 「作問開始」ボタンをクリックする
  5. Excel 形式でエクスポートする
- **期待結果**: 「ガイドライン」列に test_guideline.md のガイドライン番号がカンマ区切りで格納されること

#### TS-5-4: LLM レスポンスの JSON オブジェクト解析（ユニットテスト）
- **目的**: `parse_llm_response` が JSON オブジェクト形式から `distractors` と `guideline_numbers` を正しく抽出することを確認する
- **テスト ID**: UT-11, UT-12（generator.rs 内の新規テスト）
- **期待結果**: `generator::tests` の全ユニットテストが通過する

#### TS-5-5: 配列形式へのフォールバック
- **目的**: LLM がプレーンな JSON 配列（ガイドライン番号なし）を返した場合に正常動作することを確認する
- **テスト ID**: UT-13
- **期待結果**: 選択肢は正常に生成され、guideline フィールドは空文字列になり、エラーが発生しないこと

---

### 機能6 — エクスポート形式の構造準拠

#### TS-6-1: Moodle XML 構造

- **TS-6-1-1**: 問題テキストと選択肢テキストが CDATA で囲まれていること
- **TS-6-1-2**: Moodle XML.md で定義された必須フィールド（defaultgrade, penalty, hidden, single, shuffleanswers, answernumbering）が全て存在すること
- **TS-6-1-3**: answer 要素が `format="html"` を使用していること（`format="moodle_auto_format"` は不可）
- **TS-6-1-4**: 正答が `fraction="100"`、誤答が `fraction="0"` で出力されること

#### TS-6-2: QTI 1.2 構造

- **TS-6-2-1**: `<render_choice shuffle="No">` が `<response_lid>` 内に存在すること
- **TS-6-2-2**: 選択肢の ident がアルファベット（A, B, C, D）であること（`label_1` は不可）
- **TS-6-2-3**: `<varequal respident="RESPONSE">` が正答のアルファベット ident を参照していること
- **TS-6-2-4**: SCORE 範囲が 0-1 であること（`maxvalue="1"`, `<setvar>1</setvar>`）

#### TS-6-3: QTI 2.1 構造

- **TS-6-3-1**: `<responseDeclaration>` と `<outcomeDeclaration>` が設問ごとに存在すること
- **TS-6-3-2**: `<correctResponse><value>` が正答の `choice_N` 識別子を参照していること
- **TS-6-3-3**: `shuffle="false"` であること
- **TS-6-3-4**: テンプレート形式の responseProcessing が使用されていること

#### TS-6-4: QTI 2.2 構造

- **TS-6-4-1**: QTI 2.2 の名前空間と purl.imsglobal.org のスキーマ定義が正しいこと
- **TS-6-4-2**: `<p>` タグで問題本文が `<choiceInteraction>` より前に配置されていること
- **TS-6-4-3**: QTI 2.2 用のテンプレート URL が使用されていること

#### TS-6-5: QTI 3.0 構造

- **TS-6-5-1**: ルート要素が `<qti-assessment-item>` であること
- **TS-6-5-2**: 名前空間が `http://www.imsglobal.org/xsd/imsqtiasi_v3p0` であること
- **TS-6-5-3**: 全要素に `qti-` プレフィックスが付いていること
- **TS-6-5-4**: 選択肢識別子がアルファベット（A, B, C, D）であること
- **TS-6-5-5**: 属性名がハイフン区切りであること（`response-identifier`, `max-choices`）
- **TS-6-5-6**: `xml:lang="ja-JP"` がルート要素に存在すること
- **TS-6-5-7**: QTI 3.0 用の purl.imsglobal.org テンプレート URL が使用されていること

---

### ユニットテストカバレッジ

| ファイル | 追加テスト数 |
|---|---|
| `src/generator.rs` | 7件（UT-11〜UT-17） |
| `src/export/moodle.rs` | 4件 |
| `src/export/qti12.rs` | 6件 |
| `src/export/qti21.rs` | 6件 |
| `src/export/qti22.rs` | 5件 |
| `src/export/qti30.rs` | 8件 |

**本改訂後の総テスト数**: 53件（ユニット51件 + 統合2件）— 全件通過確認済み。
