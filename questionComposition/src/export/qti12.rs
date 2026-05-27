// export/qti12.rs
// 目的: QTI 1.2 形式でのエクスポート機能を実装する。
// QTI (Question and Test Interoperability) は 1EdTech の標準フォーマット。
// 構造定義: RequirementsSpecification/QTI 1.2.md に準拠。
// 参考: https://www.1edtech.org/standards/qti/index
// 変更点(Requirements3add):
//   - <presentation><flow> を <presentation><material><mattext> + <response_lid><render_choice> に変更
//   - 選択肢 ident をアルファベット (A, B, C, D ...) に変更
//   - respident を "RESPONSE" に統一
//   - score 範囲を 0-1 に変更 (maxvalue="1", setvar=1)
//   - <item> に title 属性を追加

use crate::model::QuestionWithChoices;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use anyhow::{Context, Result};

/// 選択肢インデックスをアルファベット識別子に変換する (0→A, 1→B, ...)
fn choice_ident(index: usize) -> String {
    // A-Z の範囲を前提とする（選択肢数が 26 を超えることはない）
    let c = char::from(b'A' + index as u8);
    c.to_string()
}

pub struct Qti12Exporter;

impl super::ExportHandler for Qti12Exporter {
    fn export(
        &self,
        questions: &[QuestionWithChoices],
        subject: &str,
        output_path: &PathBuf,
    ) -> Result<()> {
        let xml_content = generate_qti12_xml(questions, subject)?;

        let mut file = File::create(output_path)
            .with_context(|| format!("Failed to create file: {}", output_path.display()))?;

        file.write_all(xml_content.as_bytes())
            .with_context(|| format!("Failed to write to file: {}", output_path.display()))?;

        Ok(())
    }

    fn format_name(&self) -> &'static str {
        "QTI 1.2"
    }
}

/// QTI 1.2 フォーマットのコンテンツを生成する
/// 複数問題は <questestinterop><assessment><section> で包む
fn generate_qti12_xml(questions: &[QuestionWithChoices], subject: &str) -> Result<String> {
    let mut xml = String::new();

    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<!DOCTYPE questestinterop SYSTEM \"http://www.imsglobal.org/ims_qtiasiv1p2p1.dtd\">\n");
    xml.push_str("<questestinterop>\n");

    xml.push_str("  <assessment ident=\"assessment_1\">\n");
    xml.push_str("    <qtimetadata>\n");
    xml.push_str("      <qtimetadatafield>\n");
    xml.push_str("        <fieldlabel>Subject</fieldlabel>\n");
    xml.push_str(&format!(
        "        <fieldentry>{}</fieldentry>\n",
        escape_xml(subject)
    ));
    xml.push_str("      </qtimetadatafield>\n");
    xml.push_str("    </qtimetadata>\n");

    xml.push_str("    <section ident=\"section_1\">\n");

    for (idx, question) in questions.iter().enumerate() {
        xml.push_str(&generate_qti12_item(question, idx)?);
    }

    xml.push_str("    </section>\n");
    xml.push_str("  </assessment>\n");
    xml.push_str("</questestinterop>\n");

    Ok(xml)
}

/// 単一の設問を QTI 1.2 形式で生成する
/// 構造は QTI 1.2.md サンプルに準拠する
fn generate_qti12_item(question: &QuestionWithChoices, index: usize) -> Result<String> {
    let mut xml = String::new();

    let question_ident = format!("ITEM{:03}", index + 1);

    // 問題本文を title に使用する（50文字で切り詰め）
    let title = if question.body.chars().count() > 50 {
        question
            .body
            .chars()
            .take(50)
            .collect::<String>()
            + "..."
    } else {
        question.body.clone()
    };

    xml.push_str(&format!(
        "      <item ident=\"{}\" title=\"{}\">\n",
        question_ident,
        escape_xml(&title)
    ));

    // 問題テキスト: <presentation><material><mattext>
    xml.push_str("        <presentation>\n");
    xml.push_str("          <material>\n");
    xml.push_str(&format!(
        "            <mattext texttype=\"text/plain\">{}</mattext>\n",
        escape_xml(&question.body)
    ));
    xml.push_str("          </material>\n");

    // 選択肢: <response_lid><render_choice shuffle="No"><response_label>
    xml.push_str("          <response_lid ident=\"RESPONSE\" rcardinality=\"Single\">\n");
    xml.push_str("            <render_choice shuffle=\"No\">\n");

    // 正答のアルファベット識別子を先に特定する
    let correct_ident = question
        .choices
        .iter()
        .position(|c| c == &question.correct_answer)
        .map(choice_ident)
        .unwrap_or_else(|| "A".to_string());

    for (idx, choice) in question.choices.iter().enumerate() {
        let ident = choice_ident(idx);
        xml.push_str(&format!(
            "              <response_label ident=\"{}\">\n",
            ident
        ));
        xml.push_str("                <material>\n");
        xml.push_str(&format!(
            "                  <mattext texttype=\"text/plain\">{}</mattext>\n",
            escape_xml(choice)
        ));
        xml.push_str("                </material>\n");
        xml.push_str("              </response_label>\n");
    }

    xml.push_str("            </render_choice>\n");
    xml.push_str("          </response_lid>\n");
    xml.push_str("        </presentation>\n");

    // 正答処理: スコア範囲は 0-1 (サンプルに準拠)
    xml.push_str("        <resprocessing>\n");
    xml.push_str("          <outcomes>\n");
    xml.push_str("            <decvar vartype=\"Decimal\" minvalue=\"0\" maxvalue=\"1\" varname=\"SCORE\"/>\n");
    xml.push_str("          </outcomes>\n");
    xml.push_str("          <respcondition continue=\"No\">\n");
    xml.push_str("            <conditionvar>\n");
    xml.push_str(&format!(
        "              <varequal respident=\"RESPONSE\">{}</varequal>\n",
        correct_ident
    ));
    xml.push_str("            </conditionvar>\n");
    xml.push_str("            <setvar varname=\"SCORE\" action=\"Set\">1</setvar>\n");
    xml.push_str("          </respcondition>\n");
    xml.push_str("        </resprocessing>\n");

    xml.push_str("      </item>\n");

    Ok(xml)
}

/// XML の特殊文字をエスケープする
fn escape_xml(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::QuestionWithChoices;

    fn sample_question() -> QuestionWithChoices {
        QuestionWithChoices {
            no: 1,
            question_no: "Q1".to_string(),
            body: "日本の首都はどこですか。".to_string(),
            correct_answer: "東京".to_string(),
            choices: vec![
                "大阪".to_string(),
                "東京".to_string(),
                "京都".to_string(),
                "福岡".to_string(),
            ],
            guideline: "21, 27".to_string(),
        }
    }

    #[test]
    fn choice_ident_maps_correctly() {
        assert_eq!(choice_ident(0), "A");
        assert_eq!(choice_ident(1), "B");
        assert_eq!(choice_ident(3), "D");
    }

    #[test]
    fn qti12_uses_render_choice() {
        let q = sample_question();
        let xml = generate_qti12_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("<render_choice shuffle=\"No\">"));
    }

    #[test]
    fn qti12_uses_alphabetic_idents() {
        let q = sample_question();
        let xml = generate_qti12_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("ident=\"A\""));
        assert!(xml.contains("ident=\"B\""));
        assert!(!xml.contains("ident=\"label_1\""));
    }

    #[test]
    fn qti12_correct_answer_referenced_by_ident() {
        let q = sample_question();
        let xml = generate_qti12_xml(&[q], "テスト科目").unwrap();
        // 東京は choices[1] なので ident="B"
        assert!(xml.contains("<varequal respident=\"RESPONSE\">B</varequal>"));
    }

    #[test]
    fn qti12_score_range_is_zero_to_one() {
        let q = sample_question();
        let xml = generate_qti12_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("maxvalue=\"1\""));
        assert!(xml.contains("<setvar varname=\"SCORE\" action=\"Set\">1</setvar>"));
        assert!(!xml.contains("maxvalue=\"100\""));
    }

    #[test]
    fn qti12_uses_response_ident_response() {
        let q = sample_question();
        let xml = generate_qti12_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("ident=\"RESPONSE\""));
        assert!(xml.contains("respident=\"RESPONSE\""));
    }
}
