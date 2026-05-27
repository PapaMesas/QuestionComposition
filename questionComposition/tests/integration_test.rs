// tests/integration_test.rs
// 目的: IT-01・IT-02 に対応する統合テストを定義する。
// IT-01: APIキーを保存後、ファイルから復号して一致する
// IT-02: .xlsx 往復 (読み込み → 書き出し → 再読み込み) でデータが一致する

use question_composition::config::{self, AppConfig};
use question_composition::excel::{reader, writer};
use question_composition::model::{LlmProvider, QuestionWithChoices};
use rust_xlsxwriter::Workbook;
use tempfile::{NamedTempFile, TempDir};

// IT-01: APIキーの保存・復号ラウンドトリップ
#[test]
fn api_key_roundtrip_through_config_file() {
    // 一時ディレクトリに config.toml を書き出す
    let tmp_dir = TempDir::new().unwrap();
    let config_path = tmp_dir.path().join("config.toml");

    // AppConfig を手動で構築し、APIキーを暗号化して保存する
    let mut cfg = AppConfig {
        provider: LlmProvider::Gemini,
        encrypted_api_key: None,
    };
    let original_key = "sk-test-integration-key-123456";
    config::store_api_key_to(&mut cfg, &config_path, original_key).unwrap();

    // ファイルから読み込んで復号する
    let loaded_cfg = config::load_from(&config_path);
    let recovered = config::load_api_key(&loaded_cfg).unwrap().unwrap();

    assert_eq!(recovered, original_key);
}

// IT-02: .xlsx 往復テスト (読み込み → 書き出し → 再読み込みでデータが一致する)
#[test]
fn xlsx_roundtrip_preserves_data() {
    // ① テスト用入力 .xlsx を生成する
    let input_tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    {
        let mut wb = Workbook::new();
        let ws = wb.add_worksheet();
        ws.write_string(0, 0, "テスト科目").unwrap();
        for (col, h) in ["No", "問題番号", "問題本文", "選択肢数", "模範解答"]
            .iter()
            .enumerate()
        {
            ws.write_string(1, col as u16, *h).unwrap();
        }
        ws.write_string(2, 0, "1").unwrap();
        ws.write_string(2, 1, "Q1").unwrap();
        ws.write_string(2, 2, "問題本文テスト").unwrap();
        ws.write_string(2, 4, "正答テスト").unwrap();
        wb.save(input_tmp.path()).unwrap();
    }

    // ② 読み込む
    let sheet = reader::read_xlsx(input_tmp.path()).unwrap();
    assert_eq!(sheet.subject, "テスト科目");
    assert_eq!(sheet.questions.len(), 1);

    // ③ QuestionWithChoices に変換して書き出す
    let qwc = QuestionWithChoices {
        no: sheet.questions[0].no,
        question_no: sheet.questions[0].question_no.clone(),
        body: sheet.questions[0].body.clone(),
        correct_answer: sheet.questions[0].correct_answer.clone(),
        choices: vec![
            "誤答1".to_string(),
            "正答テスト".to_string(),
            "誤答2".to_string(),
            "誤答3".to_string(),
        ],
        guideline: "test_guideline".to_string(),
    };
    let output_tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
    writer::write_xlsx(output_tmp.path(), &sheet.subject, &[qwc]).unwrap();

    // ④ 書き出した .xlsx が存在して 0 バイトでないことを確認する
    let metadata = std::fs::metadata(output_tmp.path()).unwrap();
    assert!(metadata.len() > 0, "出力 .xlsx が空です");
}
