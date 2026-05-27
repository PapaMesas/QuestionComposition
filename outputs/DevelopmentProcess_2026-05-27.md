# Development Process — Question Composition Application (Spec Revision: Requirements3add)

**Date**: 2026-05-27  
**Base Process**: DevelopmentProcess_2026-05-26.md

---

## English Version

### Overview

This document records the development process for implementing the specification changes introduced in `Requirements3add.md`.

---

### Phase 1: Spec Diff Analysis

1. Read `Requirements2add.md` (current spec) and `Requirements3add.md` (new spec)
2. Compared both documents line by line
3. Identified two change areas:
   - **Feature 5**: Added requirement to record specific guideline numbers from `test_guideline.md`
   - **Feature 6**: Added structure definition files per export format in `RequirementsSpecification/`
4. Read all five format definition files (`Moodle XML.md`, `QTI 1.2.md`, `QTI 2.1.md`, `QTI 2.2.md`, `QTI 3.0.md`)
5. Compared existing export implementations against the sample structures
6. Documented gaps per format

---

### Phase 2: Development Plan

1. Drafted revised development plan with both English and Japanese versions
2. Presented plan to user for approval
3. Plan approved; `DevelopmentPlan_2026-05-27.md` written to `outputs/`

---

### Phase 3: Implementation

#### P-A: generator.rs — Guideline Number Tracking

- Changed LLM prompt to request JSON object with `distractors` and `guideline_numbers`
- Replaced `parse_json_array` with `parse_llm_response` + `try_parse_as_object`
- Added `format_guideline_numbers` to convert `[21, 27, 33]` → `"21, 27, 33"`
- Added fallback: if LLM returns plain array, guideline field becomes empty string (no error)

#### P-B: export/moodle.rs — Moodle XML Structural Alignment

- Wrapped all text content in `<![CDATA[...]]>`
- Added `<defaultgrade>1.0000000</defaultgrade>`, `<penalty>0.3333333</penalty>`, `<hidden>0</hidden>`, `<single>true</single>`, `<shuffleanswers>true</shuffleanswers>`, `<answernumbering>abc</answernumbering>`
- Changed answer format attribute from `moodle_auto_format` to `html`
- Changed output logic: all choices in stored order; `fraction="100"` for correct, `"0"` for distractors

#### P-C: export/qti12.rs — QTI 1.2 Structural Alignment

- Changed `<presentation><flow>` to `<presentation><material><mattext>`
- Changed `<response_lid>` inner structure to `<render_choice shuffle="No">`
- Replaced uniform `label_1` idents with per-choice alphabetic idents (A, B, C, D)
- Changed `respident` to `"RESPONSE"`
- Changed score range from 0-100 to 0-1
- Added `title` attribute to `<item>` using question body

#### P-D: export/qti21.rs — QTI 2.1 Structural Alignment

- Rebuilt to generate one `<assessmentItem>` per question; wrapped in `<assessmentItems>` root
- Added `<responseDeclaration>` with correct `choice_N` identifier
- Added `<outcomeDeclaration>` with default value 0
- Changed `shuffle` to `"false"`
- Replaced custom `<responseProcessing>` with template reference

#### P-E: export/qti22.rs — QTI 2.2 Structural Alignment

- Applied same structural changes as QTI 2.1
- Updated namespace to `http://www.imsglobal.org/xsd/imsqti_v2p2`
- Updated `xsi:schemaLocation` to `purl.imsglobal.org` URL
- Added `<p>` tag for question body before `<choiceInteraction>` in `<itemBody>`
- Updated template URL to QTI 2.2 variant

#### P-F: export/qti30.rs — QTI 3.0 Structural Alignment

- Changed root element from `<assessmentItem>` to `<qti-assessment-item>`
- Updated namespace to `http://www.imsglobal.org/xsd/imsqtiasi_v3p0`
- Renamed all elements with `qti-` prefix
- Changed attribute names to hyphenated form (`response-identifier`, `max-choices`)
- Changed choice identifiers to alphabetic (A, B, C, D)
- Added `xml:lang="ja-JP"` to root element
- Removed non-standard `<metadata>` block
- Updated template URL to purl.imsglobal.org QTI 3.0 variant

---

### Phase 4: Testing

1. Wrote test specification — `TestSpecification_2026-05-27.md`
2. Presented for approval; approved by user
3. Ran `cargo test -- --nocapture`
4. **Result**: 53 tests passed, 0 failed
5. Wrote test results — `TestResults_2026-05-27.md`

---

### Files Modified

| File | Change Type |
|---|---|
| `src/generator.rs` | Modified — guideline number extraction |
| `src/export/moodle.rs` | Modified — structural alignment to Moodle XML.md |
| `src/export/qti12.rs` | Modified — structural alignment to QTI 1.2.md |
| `src/export/qti21.rs` | Modified — structural alignment to QTI 2.1.md |
| `src/export/qti22.rs` | Modified — structural alignment to QTI 2.2.md |
| `src/export/qti30.rs` | Modified — structural alignment to QTI 3.0.md |

### Files Created

| File | Purpose |
|---|---|
| `outputs/DevelopmentPlan_2026-05-27.md` | Revised development plan (EN + JA) |
| `outputs/TestSpecification_2026-05-27.md` | Revised test specification (EN + JA) |
| `outputs/TestResults_2026-05-27.md` | Test results (EN + JA) |
| `outputs/DevelopmentProcess_2026-05-27.md` | This document |

---

## 日本語版

### 概要

`Requirements3add.md` で追加された仕様変更の実装プロセスを記録します。

---

### フェーズ1: 仕様差分の分析

1. `Requirements2add.md`（現行仕様）と `Requirements3add.md`（新仕様）を読み込んだ
2. 2文書を行単位で比較した
3. 2つの変更箇所を特定した:
   - **機能5**: `test_guideline.md` の具体的なガイドライン番号を記録する要件が追加された
   - **機能6**: 各エクスポート形式に対応する構造定義ファイルが `RequirementsSpecification/` に配置された
4. 5つの形式定義ファイルをすべて読み込んだ
5. 既存の各エクスポート実装をサンプル構造と比較した
6. 形式ごとの差分を文書化した

---

### フェーズ2: 開発計画

1. 英語版・日本語版を含む修正開発計画を作成した
2. ユーザーに提示して承認を得た
3. `DevelopmentPlan_2026-05-27.md` を `outputs/` に出力した

---

### フェーズ3: 実装

**P-A**: `generator.rs` — LLM プロンプトを JSON オブジェクト形式に変更し、ガイドライン番号を抽出・フォーマットするロジックを実装した。

**P-B**: `export/moodle.rs` — CDATA ラップ・必須フィールド追加・`format="html"` 変更・選択肢出力ロジックを修正した。

**P-C**: `export/qti12.rs` — presentation 構造・render_choice・A-D ident・スコア範囲・respident を修正した。

**P-D**: `export/qti21.rs` — 設問ごとの assessmentItem 構造・宣言要素・テンプレート responseProcessing に再構築した。

**P-E**: `export/qti22.rs` — P-D と同様の修正に加えて QTI 2.2 固有の名前空間・URL・`<p>` タグを適用した。

**P-F**: `export/qti30.rs` — `qti-` プレフィックス要素・アルファベット ident・ハイフン属性・`xml:lang` で完全に再構築した。

---

### フェーズ4: テスト

1. `TestSpecification_2026-05-27.md` を作成し、ユーザーの承認を得た
2. `cargo test -- --nocapture` を実行した
3. **結果**: 53件全テスト通過・失敗0件
4. `TestResults_2026-05-27.md` を出力した
