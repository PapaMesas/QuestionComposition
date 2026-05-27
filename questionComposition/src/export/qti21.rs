// export/qti21.rs
// 目的: QTI 2.1 形式でのエクスポート機能を実装する。
// 構造定義: RequirementsSpecification/QTI 2.1.md に準拠。
// 参考: https://www.1edtech.org/standards/qti/index
// 変更点(Requirements3add):
//   - 設問ごとに独立した <assessmentItem> を生成する
//   - <responseDeclaration> と <outcomeDeclaration> を追加する
//   - shuffle="false" に変更
//   - 選択肢 ident を choice_1..N に統一し、正答の identifier を responseDeclaration で参照する
//   - responseProcessing をテンプレート形式に変更する
//   - schemaLocation を Moodle XML.md サンプルに合わせて修正する

use crate::model::QuestionWithChoices;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use anyhow::{Context, Result};

pub struct Qti21Exporter;

impl super::ExportHandler for Qti21Exporter {
    fn export(
        &self,
        questions: &[QuestionWithChoices],
        subject: &str,
        output_path: &PathBuf,
    ) -> Result<()> {
        let xml_content = generate_qti21_xml(questions, subject)?;

        let mut file = File::create(output_path)
            .with_context(|| format!("Failed to create file: {}", output_path.display()))?;

        file.write_all(xml_content.as_bytes())
            .with_context(|| format!("Failed to write to file: {}", output_path.display()))?;

        Ok(())
    }

    fn format_name(&self) -> &'static str {
        "QTI 2.1"
    }
}

/// QTI 2.1 形式でコンテンツを生成する
/// 複数問題はそれぞれ独立した <assessmentItem> として出力し、
/// XML 妥当性を保つため <assessmentItems> ルート要素で包む
fn generate_qti21_xml(questions: &[QuestionWithChoices], subject: &str) -> Result<String> {
    let mut xml = String::new();

    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    // 複数 assessmentItem を収容するルート要素
    xml.push_str("<assessmentItems>\n");
    xml.push_str(&format!("  <!-- Subject: {} -->\n", escape_xml(subject)));

    for (idx, question) in questions.iter().enumerate() {
        xml.push_str(&generate_qti21_item(question, idx)?);
    }

    xml.push_str("</assessmentItems>\n");

    Ok(xml)
}

/// 単一の設問を QTI 2.1 形式で生成する
/// 構造は QTI 2.1.md サンプルに準拠する
fn generate_qti21_item(question: &QuestionWithChoices, index: usize) -> Result<String> {
    let mut xml = String::new();

    let item_id = format!("item_{:03}", index + 1);

    // 正答に対応する choice_N ident を特定する
    let correct_choice_id = question
        .choices
        .iter()
        .position(|c| c == &question.correct_answer)
        .map(|pos| format!("choice_{}", pos + 1))
        .unwrap_or_else(|| "choice_1".to_string());

    // <assessmentItem> ルート要素 (QTI 2.1.md のスキーマに準拠)
    xml.push_str(&format!(
        "  <assessmentItem\n    xmlns=\"http://www.imsglobal.org/xsd/imsqti_v2p1\"\n"
    ));
    xml.push_str(
        "    xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"\n"
    );
    xml.push_str(
        "    xsi:schemaLocation=\"http://www.imsglobal.org/xsd/imsqti_v2p1 \n"
    );
    xml.push_str(
        "    http://www.imsglobal.org/xsd/qti/qtiv2p1/imsqti_v2p1.xsd\"\n"
    );
    xml.push_str(&format!(
        "    identifier=\"{}\"\n    title=\"{}\"\n    adaptive=\"false\"\n    timeDependent=\"false\">\n",
        item_id,
        escape_xml(&question.question_no)
    ));

    // 正答宣言
    xml.push_str(
        "    <responseDeclaration identifier=\"RESPONSE\" cardinality=\"single\" baseType=\"identifier\">\n"
    );
    xml.push_str("      <correctResponse>\n");
    xml.push_str(&format!(
        "        <value>{}</value>\n",
        correct_choice_id
    ));
    xml.push_str("      </correctResponse>\n");
    xml.push_str("    </responseDeclaration>\n");

    // スコア宣言
    xml.push_str(
        "    <outcomeDeclaration identifier=\"SCORE\" cardinality=\"single\" baseType=\"float\">\n"
    );
    xml.push_str("      <defaultValue>\n");
    xml.push_str("        <value>0</value>\n");
    xml.push_str("      </defaultValue>\n");
    xml.push_str("    </outcomeDeclaration>\n");

    // 問題本文と選択肢
    xml.push_str("    <itemBody>\n");
    xml.push_str("      <choiceInteraction responseIdentifier=\"RESPONSE\" shuffle=\"false\" maxChoices=\"1\">\n");
    xml.push_str(&format!(
        "        <prompt>{}</prompt>\n",
        escape_xml(&question.body)
    ));

    for (idx, choice) in question.choices.iter().enumerate() {
        xml.push_str(&format!(
            "        <simpleChoice identifier=\"choice_{}\">{}</simpleChoice>\n",
            idx + 1,
            escape_xml(choice)
        ));
    }

    xml.push_str("      </choiceInteraction>\n");
    xml.push_str("    </itemBody>\n");

    // テンプレートによる応答処理 (サンプルに準拠)
    xml.push_str(
        "    <responseProcessing\n        template=\"http://www.imsglobal.org/question/qti_v2p1/rptemplates/match_correct\"/>\n"
    );

    xml.push_str("  </assessmentItem>\n");

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
                "名古屋".to_string(),
            ],
            guideline: "21, 27".to_string(),
        }
    }

    #[test]
    fn qti21_has_response_declaration() {
        let q = sample_question();
        let xml = generate_qti21_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("<responseDeclaration identifier=\"RESPONSE\""));
    }

    #[test]
    fn qti21_has_outcome_declaration() {
        let q = sample_question();
        let xml = generate_qti21_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("<outcomeDeclaration identifier=\"SCORE\""));
    }

    #[test]
    fn qti21_correct_choice_id_in_response_declaration() {
        let q = sample_question();
        // 東京は choices[1] なので choice_2
        let xml = generate_qti21_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("<value>choice_2</value>"));
    }

    #[test]
    fn qti21_uses_shuffle_false() {
        let q = sample_question();
        let xml = generate_qti21_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("shuffle=\"false\""));
    }

    #[test]
    fn qti21_uses_template_response_processing() {
        let q = sample_question();
        let xml = generate_qti21_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("rptemplates/match_correct"));
    }

    #[test]
    fn qti21_uses_correct_namespace() {
        let q = sample_question();
        let xml = generate_qti21_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("http://www.imsglobal.org/xsd/imsqti_v2p1"));
    }
}
