# Test Specification — Question Composition Application (v2)

**Date**: 2026-05-26

---

## English Version

### Overview

Test specification for Feature 5 enhancement (guideline tracking in Excel output) and Feature 6 implementation (output format selection: Moodle XML, QTI 1.2, QTI 2.1, QTI 2.2, QTI 3.0).

---

### Feature 5 Enhancements — Guideline Tracking

#### TS-5-1: Excel Output Includes Guideline Column
- **Objective**: Verify that Excel output includes the "ガイドライン" column after all choice columns
- **Steps**:
  1. Import test questions from InputMaterials/テスト用.xlsx
  2. Configure choices (default 4)
  3. Confirm settings and select a guideline rule
  4. Generate choices
  5. Export to Excel (original format)
- **Expected Result**: Excel file has column "ガイドライン" after choice columns, showing which rule was used

#### TS-5-2: Guideline Name is Accurately Recorded
- **Objective**: Verify that the correct guideline name is recorded for each question
- **Steps**:
  1. Generate choices with multiple guideline rules
  2. Export to Excel
  3. Inspect "ガイドライン" column
- **Expected Result**: Each row shows the correct rule name (from rules.source_label)

---

### Feature 6 — Output Format Selection

#### TS-6-1: Export Tab is Accessible
- **Objective**: Verify that the Export tab appears in the tab bar and is navigable
- **Steps**:
  1. Launch application
  2. Generate test questions
  3. Click "⑥ エクスポート" tab
- **Expected Result**: Export panel appears with format selection UI

#### TS-6-2: Format Selection Works
- **Objective**: Verify that all output formats can be selected
- **Steps**:
  1. Navigate to Export tab
  2. Click each radio button: Moodle XML, QTI 1.2, QTI 2.1, QTI 2.2, QTI 3.0
- **Expected Result**: Each format is selectable; filename updates appropriately

#### TS-6-3: File Selection Dialog Works
- **Objective**: Verify that file selection dialog opens and saves path
- **Steps**:
  1. Click "📂 出力先を選択" button
  2. Navigate to a test directory
  3. Confirm selection
- **Expected Result**: Path is displayed; export button becomes enabled

#### TS-6-4: Moodle XML Export Succeeds
- **Objective**: Verify that Moodle XML format exports correctly
- **Steps**:
  1. Generate test questions (3-5 questions)
  2. Select Moodle XML format
  3. Choose output path
  4. Click "📤 エクスポート"
- **Expected Result**: 
  - XML file is created
  - Success message appears
  - File contains valid XML with quiz element
  - Guideline info is included as metadata

#### TS-6-5: QTI 1.2 Export Succeeds
- **Objective**: Verify that QTI 1.2 format exports correctly
- **Steps**:
  1. Select QTI 1.2 format
  2. Choose output path
  3. Click "📤 エクスポート"
- **Expected Result**:
  - XML file is created with QTI 1.2 namespace
  - Contains questestinterop element
  - Each question has item element with proper metadata

#### TS-6-6: QTI 2.1 Export Succeeds
- **Objective**: Verify that QTI 2.1 format exports correctly
- **Steps**:
  1. Select QTI 2.1 format
  2. Choose output path
  3. Click "📤 エクスポート"
- **Expected Result**:
  - XML file is created with QTI 2.1 namespace
  - Contains assessmentItem element
  - Has itemBody with choiceInteraction elements

#### TS-6-7: QTI 2.2 Export Succeeds
- **Objective**: Verify that QTI 2.2 format exports correctly
- **Steps**:
  1. Select QTI 2.2 format
  2. Choose output path
  3. Click "📤 エクスポート"
- **Expected Result**:
  - XML file is created with QTI 2.2 namespace (different from 2.1)
  - Structure is compatible with QTI 2.2 specification

#### TS-6-8: QTI 3.0 Export Succeeds
- **Objective**: Verify that QTI 3.0 format exports correctly
- **Steps**:
  1. Select QTI 3.0 format
  2. Choose output path
  3. Click "📤 エクスポート"
- **Expected Result**:
  - XML file is created with QTI 3.0 namespace
  - Contains assessmentItem element
  - Guideline information is included

#### TS-6-9: XML Special Characters are Escaped
- **Objective**: Verify that XML special characters in questions/choices are properly escaped
- **Steps**:
  1. Create test data with special characters: <, >, &, ", '
  2. Export to any XML format (e.g., Moodle)
  3. Open XML file and verify content
- **Expected Result**: All special characters are properly XML-escaped (&lt;, &gt;, &amp;, etc.)

#### TS-6-10: Error Handling on Invalid Path
- **Objective**: Verify that export fails gracefully if output path is invalid
- **Steps**:
  1. Set output path to a read-only or non-existent directory
  2. Click "📤 エクスポート"
- **Expected Result**: Error message appears describing the issue

#### TS-6-11: File Overwrite Protection
- **Objective**: Verify behavior when exporting to an existing file path
- **Steps**:
  1. Create a dummy XML file at the export path
  2. Select the same path for export
  3. Click "📤 エクスポート"
- **Expected Result**: File is overwritten (or warning is shown, based on OS behavior)

---

### Integration Tests

#### TS-INT-1: Full Workflow with Excel + Export
- **Objective**: End-to-end test from Excel import to multiple format exports
- **Steps**:
  1. Import questions from Excel
  2. Configure choices
  3. Generate with guideline rules
  4. Export to Excel (Feature 5)
  5. Export to all QTI formats (Feature 6)
- **Expected Result**: 
  - All 5 files are created
  - Excel includes guideline column
  - All XML files are valid

#### TS-INT-2: Multiple Export Formats from Same Generation
- **Objective**: Verify that the same generated questions can be exported to multiple formats
- **Steps**:
  1. Generate questions once
  2. Export to Moodle XML
  3. Change format to QTI 1.2 and export again
  4. Repeat for QTI 2.1, 2.2, 3.0
- **Expected Result**: All 5 files are created from single generation

#### TS-INT-3: Guideline Consistency Across Formats
- **Objective**: Verify that guideline info is preserved in all export formats
- **Steps**:
  1. Generate questions with specific rule
  2. Export to all formats
  3. Inspect each file for guideline information
- **Expected Result**: Guideline appears in all formats (in metadata, comments, or small text)

---

### Regression Tests

#### TS-REG-1: Existing Excel Export Still Works
- **Objective**: Verify that Feature 5 original Excel export hasn't been broken
- **Steps**:
  1. Generate questions
  2. Export to Excel (from generate_panel or export_panel)
  3. Verify structure and content
- **Expected Result**: Excel export works as before with added guideline column

#### TS-REG-2: All Previous Features Work
- **Objective**: Quick smoke test of existing functionality
- **Steps**:
  1. API registration (Feature 1)
  2. Import questions (Feature 2)
  3. Configure choices (Feature 3)
  4. Load rules (Feature 4)
  5. Generate choices (Feature 5)
- **Expected Result**: No regressions; all features work as expected

---

---

## 日本語版

### 概要

機能5の拡張（Excel出力にガイドライン情報を含める）および機能6の実装（複数の出力形式選択: Moodle XML、QTI 1.2、QTI 2.1、QTI 2.2、QTI 3.0）に対するテスト仕様書です。

---

### 機能5の拡張 — ガイドライン追跡

#### TS-5-1: Excelの出力にガイドライン列を含む
- **目的**: Excel出力に「ガイドライン」列が全選択肢列の後に含まれることを確認
- **手順**:
  1. InputMaterials/テスト用.xlsxから設問を取り込む
  2. 選択肢数を設定（デフォルト4）
  3. 設定完了を確認し、ガイドラインルールを選択
  4. 選択肢を生成
  5. Excelへエクスポート（元の形式）
- **期待結果**: Excelファイルが選択肢列の後に「ガイドライン」列を持ち、使用されたルールが表示される

#### TS-5-2: ガイドライン名が正確に記録される
- **目的**: 各設問に対して正しいガイドライン名が記録されることを確認
- **手順**:
  1. 複数のガイドラインルールで選択肢を生成
  2. Excelへエクスポート
  3. 「ガイドライン」列を検査
- **期待結果**: 各行が正しいルール名を表示（rules.source_labelから）

---

### 機能6 — 出力形式の選択

#### TS-6-1: エクスポートタブがアクセス可能
- **目的**: エクスポートタブがタブバーに表示され、ナビゲート可能であることを確認
- **手順**:
  1. アプリケーションを起動
  2. テスト設問を生成
  3. 「⑥ エクスポート」タブをクリック
- **期待結果**: エクスポートパネルが形式選択UIと共に表示される

#### TS-6-2: 形式選択が機能する
- **目的**: すべての出力形式が選択可能であることを確認
- **手順**:
  1. エクスポートタブへナビゲート
  2. 各ラジオボタンをクリック: Moodle XML、QTI 1.2、QTI 2.1、QTI 2.2、QTI 3.0
- **期待結果**: 各形式が選択可能; ファイル名が適切に更新される

#### TS-6-3: ファイル選択ダイアログが機能する
- **目的**: ファイル選択ダイアログが開き、パスが保存されることを確認
- **手順**:
  1. 「📂 出力先を選択」ボタンをクリック
  2. テストディレクトリへナビゲート
  3. 選択を確認
- **期待結果**: パスが表示される; エクスポートボタンが有効になる

#### TS-6-4: Moodle XMLエクスポートが成功する
- **目的**: Moodle XML形式が正しくエクスポートされることを確認
- **手順**:
  1. テスト設問を生成（3-5問題）
  2. Moodle XML形式を選択
  3. 出力パスを選択
  4. 「📤 エクスポート」をクリック
- **期待結果**:
  - XMLファイルが作成される
  - 成功メッセージが表示される
  - ファイルがquiz要素を含む有効なXMLを含む
  - ガイドライン情報がメタデータとして含まれる

#### TS-6-5: QTI 1.2エクスポートが成功する
- **目的**: QTI 1.2形式が正しくエクスポートされることを確認
- **手順**:
  1. QTI 1.2形式を選択
  2. 出力パスを選択
  3. 「📤 エクスポート」をクリック
- **期待結果**:
  - QTI 1.2ネームスペースを持つXMLファイルが作成される
  - questestinterop要素を含む
  - 各設問が適切なメタデータを持つitem要素を持つ

#### TS-6-6: QTI 2.1エクスポートが成功する
- **目的**: QTI 2.1形式が正しくエクスポートされることを確認
- **手順**:
  1. QTI 2.1形式を選択
  2. 出力パスを選択
  3. 「📤 エクスポート」をクリック
- **期待結果**:
  - QTI 2.1ネームスペースを持つXMLファイルが作成される
  - assessmentItem要素を含む
  - choiceInteraction要素を持つitemBodyを持つ

#### TS-6-7: QTI 2.2エクスポートが成功する
- **目的**: QTI 2.2形式が正しくエクスポートされることを確認
- **手順**:
  1. QTI 2.2形式を選択
  2. 出力パスを選択
  3. 「📤 エクスポート」をクリック
- **期待結果**:
  - QTI 2.2ネームスペース（2.1と異なる）を持つXMLファイルが作成される
  - QTI 2.2仕様と互換性のある構造を持つ

#### TS-6-8: QTI 3.0エクスポートが成功する
- **目的**: QTI 3.0形式が正しくエクスポートされることを確認
- **手順**:
  1. QTI 3.0形式を選択
  2. 出力パスを選択
  3. 「📤 エクスポート」をクリック
- **期待結果**:
  - QTI 3.0ネームスペースを持つXMLファイルが作成される
  - assessmentItem要素を含む
  - ガイドライン情報が含まれる

#### TS-6-9: XML特殊文字がエスケープされる
- **目的**: 問題や選択肢内のXML特殊文字が適切にエスケープされることを確認
- **手順**:
  1. 特殊文字を含むテストデータを作成: <、>、&、"、'
  2. XML形式（例：Moodle）にエクスポート
  3. XMLファイルを開いて内容を確認
- **期待結果**: すべての特殊文字が適切にXMLエスケープされる（&lt;、&gt;、&amp;等）

#### TS-6-10: 無効なパスでのエラー処理
- **目的**: 出力パスが無効な場合、エクスポートが優雅に失敗することを確認
- **手順**:
  1. 出力パスを読み取り専用または存在しないディレクトリに設定
  2. 「📤 エクスポート」をクリック
- **期待結果**: 問題を説明するエラーメッセージが表示される

#### TS-6-11: ファイル上書き保護
- **目的**: 既存ファイルパスにエクスポートする場合の動作を確認
- **手順**:
  1. ダミーXMLファイルをエクスポートパスに作成
  2. エクスポートに同じパスを選択
  3. 「📤 エクスポート」をクリック
- **期待結果**: ファイルが上書きされる（またはOSの動作に基づいて警告が表示される）

---

### 統合テスト

#### TS-INT-1: Excel + Exportを使用した全体ワークフロー
- **目的**: Excelインポートから複数形式エクスポートまでのエンドツーエンドテスト
- **手順**:
  1. Excelから設問を取り込む
  2. 選択肢数を設定
  3. ガイドラインルールで生成
  4. Excelへエクスポート（機能5）
  5. すべてのQTI形式にエクスポート（機能6）
- **期待結果**:
  - 5つのファイルがすべて作成される
  - Excelがガイドライン列を含む
  - すべてのXMLファイルが有効

#### TS-INT-2: 同じ生成から複数の形式でエクスポート
- **目的**: 同じ生成設問を複数の形式にエクスポート可能であることを確認
- **手順**:
  1. 設問を1回生成
  2. Moodle XMLにエクスポート
  3. 形式をQTI 1.2に変更してエクスポート
  4. QTI 2.1、2.2、3.0について繰り返す
- **期待結果**: 単一の生成から5つのファイルがすべて作成される

#### TS-INT-3: 形式全体でのガイドライン一貫性
- **目的**: ガイドライン情報がすべてのエクスポート形式で保持されることを確認
- **手順**:
  1. 特定のルールで設問を生成
  2. すべての形式にエクスポート
  3. ガイドライン情報について各ファイルを検査
- **期待結果**: ガイドラインが全形式に表示される（メタデータ、コメント、小さいテキストとして）

---

### リグレッションテスト

#### TS-REG-1: 既存のExcelエクスポートが動作する
- **目的**: 機能5の元のExcelエクスポートが破損していないことを確認
- **手順**:
  1. 設問を生成
  2. Excelへエクスポート（generate_panelまたはexport_panelから）
  3. 構造と内容を確認
- **期待結果**: Excelエクスポートが以前どおり機能し、ガイドライン列が追加される

#### TS-REG-2: すべての既存機能が機能する
- **目的**: 既存機能の簡単なスモークテスト
- **手順**:
  1. API登録（機能1）
  2. 設問の取り込み（機能2）
  3. 選択肢数の設定（機能3）
  4. ルールの読み込み（機能4）
  5. 選択肢の生成（機能5）
- **期待結果**: リグレッションなし; すべての機能が期待どおり動作
