// excel/writer.rs
// 目的: rust_xlsxwriter を使って生成結果を .xlsx ファイルへ出力する。
// 出力フォーマット仕様:
//   1行目: 科目名
//   2行目: ヘッダー (No, 問題番号, 問題本文, 模範解答, 選択肢数, 選択肢1, 選択肢2, ..., ガイドライン)
//   3行目以降: データ行

use anyhow::{Context, Result};
use rust_xlsxwriter::Workbook;
use std::path::Path;

use crate::model::QuestionWithChoices;

/// 生成された選択肢データを .xlsx ファイルへ書き出す
pub fn write_xlsx(path: &Path, subject: &str, questions: &[QuestionWithChoices]) -> Result<()> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // 1行目: 科目名
    worksheet
        .write_string(0, 0, subject)
        .context("Failed to write subject")?;

    // 2行目: ヘッダー行
    // 最大選択肢数を求めてヘッダーを動的に生成する
    let max_choices = questions.iter().map(|q| q.choices.len()).max().unwrap_or(0);

    let fixed_headers = ["No", "問題番号", "問題本文", "模範解答", "選択肢数"];
    for (col, header) in fixed_headers.iter().enumerate() {
        worksheet
            .write_string(1, col as u16, *header)
            .context("Failed to write header")?;
    }
    for i in 1..=max_choices {
        worksheet
            .write_string(
                1,
                (fixed_headers.len() + i - 1) as u16,
                format!("選択肢{}", i).as_str(),
            )
            .context("Failed to write choice header")?;
    }
    // ガイドライン列のヘッダー
    let guideline_col = (fixed_headers.len() + max_choices) as u16;
    worksheet
        .write_string(1, guideline_col, "ガイドライン")
        .context("Failed to write guideline header")?;

    // 3行目以降: データ行
    for (row_idx, q) in questions.iter().enumerate() {
        let row = (row_idx + 2) as u32;

        worksheet.write_number(row, 0, q.no as f64).ok();
        worksheet.write_string(row, 1, &q.question_no).ok();
        worksheet.write_string(row, 2, &q.body).ok();
        worksheet.write_string(row, 3, &q.correct_answer).ok();
        worksheet.write_number(row, 4, q.choices.len() as f64).ok();

        for (col_offset, choice) in q.choices.iter().enumerate() {
            worksheet
                .write_string(row, (5 + col_offset) as u16, choice)
                .ok();
        }

        // ガイドライン情報を記載
        worksheet
            .write_string(row, guideline_col, &q.guideline)
            .ok();
    }

    workbook.save(path).context("Failed to save Excel file")?;

    Ok(())
}
