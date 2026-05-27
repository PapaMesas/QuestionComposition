// excel/reader.rs
// 目的: calamine を使って .xlsx ファイルを読み込み、QuestionSheet へ変換する。
// フォーマット仕様:
//   1行目: 科目名 (A列)
//   2行目: ヘッダー行 (No, 問題番号, 問題本文, 選択肢数, 模範解答)
//   3行目以降: データ行

use anyhow::{Context, Result};
use calamine::{open_workbook, Data, DataType, Reader, Xlsx};
use std::path::Path;

use crate::model::{Question, QuestionSheet};

/// 指定パスの .xlsx を読み込んで QuestionSheet を返す
pub fn read_xlsx(path: &Path) -> Result<QuestionSheet> {
    let mut workbook: Xlsx<_> = open_workbook(path).context("Failed to open Excel file")?;

    // 最初のシートを対象とする
    let sheet_name = workbook
        .sheet_names()
        .first()
        .cloned()
        .context("Excel file has no sheets")?;

    let range = workbook
        .worksheet_range(&sheet_name)
        .context("Failed to read worksheet")?;

    let mut rows = range.rows();

    // 1行目: 科目名を取得する
    let first_row = rows.next().context("Excel file is empty")?;
    let subject = cell_to_string(first_row.first());

    // 2行目: ヘッダー行 — スキップする
    rows.next();

    // 3行目以降: データ行を読み込む
    let mut questions = Vec::new();
    for row in rows {
        // 空行はスキップする
        if row.iter().all(|c| c.is_empty()) {
            continue;
        }

        let no = cell_to_string(row.first()).parse::<u32>().unwrap_or(0);
        let question_no = cell_to_string(row.get(1));
        let body = cell_to_string(row.get(2));
        // 4列目 (選択肢数) は Excel 上では null のためスキップし、デフォルト 4 を使う
        let correct_answer = cell_to_string(row.get(4));

        if question_no.is_empty() && body.is_empty() {
            continue;
        }

        questions.push(Question {
            no,
            question_no,
            body,
            correct_answer,
            num_choices: 4, // デフォルト値; UI で変更可能
        });
    }

    Ok(QuestionSheet { subject, questions })
}

/// セルの値を文字列に変換するヘルパー
fn cell_to_string(cell: Option<&Data>) -> String {
    match cell {
        None => String::new(),
        Some(Data::String(s)) => s.trim().to_string(),
        Some(Data::Float(f)) => {
            // 整数として表示できる場合は ".0" を省く
            if f.fract() == 0.0 {
                (*f as i64).to_string()
            } else {
                f.to_string()
            }
        }
        Some(Data::Int(i)) => i.to_string(),
        Some(Data::Bool(b)) => b.to_string(),
        Some(Data::Empty) => String::new(),
        Some(other) => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_xlsxwriter::Workbook;
    use tempfile::NamedTempFile;

    /// テスト用 .xlsx をメモリ上に生成してパスを返すヘルパー
    fn create_test_xlsx(subject: &str, rows: &[(&str, &str, &str, &str)]) -> NamedTempFile {
        let tmp = NamedTempFile::with_suffix(".xlsx").unwrap();
        let mut wb = Workbook::new();
        let ws = wb.add_worksheet();

        // 1行目: 科目名
        ws.write_string(0, 0, subject).unwrap();
        // 2行目: ヘッダー
        for (col, h) in ["No", "問題番号", "問題本文", "選択肢数", "模範解答"]
            .iter()
            .enumerate()
        {
            ws.write_string(1, col as u16, *h).unwrap();
        }
        // 3行目以降: データ行 (no, question_no, body, correct_answer)
        for (i, (no, qno, body, ans)) in rows.iter().enumerate() {
            let row = (i + 2) as u32;
            ws.write_string(row, 0, *no).unwrap();
            ws.write_string(row, 1, *qno).unwrap();
            ws.write_string(row, 2, *body).unwrap();
            // col 3 は null (選択肢数)
            ws.write_string(row, 4, *ans).unwrap();
        }
        wb.save(tmp.path()).unwrap();
        tmp
    }

    // UT-05: 正常な .xlsx から科目名を取得できる
    #[test]
    fn reads_subject_correctly() {
        let tmp = create_test_xlsx("テスト科目", &[("1", "Q1", "問題本文1", "正答1")]);
        let sheet = read_xlsx(tmp.path()).unwrap();
        assert_eq!(sheet.subject, "テスト科目");
    }

    // UT-06: 設問数が正しく読み込まれる
    #[test]
    fn reads_correct_question_count() {
        let tmp = create_test_xlsx(
            "科目",
            &[
                ("1", "Q1", "問題1", "答1"),
                ("2", "Q2", "問題2", "答2"),
                ("3", "Q3", "問題3", "答3"),
            ],
        );
        let sheet = read_xlsx(tmp.path()).unwrap();
        assert_eq!(sheet.questions.len(), 3);
    }

    // UT-07: 各フィールドが正しく読み込まれる
    #[test]
    fn reads_question_fields_correctly() {
        let tmp = create_test_xlsx("科目", &[("1", "Q1-1", "これは問題本文です", "正答A")]);
        let sheet = read_xlsx(tmp.path()).unwrap();
        let q = &sheet.questions[0];
        assert_eq!(q.no, 1);
        assert_eq!(q.question_no, "Q1-1");
        assert_eq!(q.body, "これは問題本文です");
        assert_eq!(q.correct_answer, "正答A");
    }

    // UT-08: デフォルトの num_choices が 4 である
    #[test]
    fn default_num_choices_is_four() {
        let tmp = create_test_xlsx("科目", &[("1", "Q1", "問題", "答")]);
        let sheet = read_xlsx(tmp.path()).unwrap();
        assert_eq!(sheet.questions[0].num_choices, 4);
    }

    // UT-09: 空行が自動スキップされる (空行を含む xlsx でも設問数が正しい)
    #[test]
    fn skips_empty_rows() {
        // 空行は create_test_xlsx では追加しないが、question_no と body が空の行はスキップされる
        let tmp = create_test_xlsx(
            "科目",
            &[("1", "Q1", "問題", "答"), ("", "", "", "")],
        );
        let sheet = read_xlsx(tmp.path()).unwrap();
        assert_eq!(sheet.questions.len(), 1);
    }

    // UT-10: 存在しないパスを渡した場合にエラーを返す
    #[test]
    fn returns_error_for_nonexistent_file() {
        let result = read_xlsx(Path::new("/tmp/nonexistent_12345.xlsx"));
        assert!(result.is_err());
    }
}
