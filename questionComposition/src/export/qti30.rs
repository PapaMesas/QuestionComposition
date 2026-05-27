// export/qti30.rs
// 目的: QTI 3.0 形式でのエクスポート機能を実装する。
// 構造定義: RequirementsSpecification/QTI 3.0.md に準拠。
// 参考: https://www.1edtech.org/standards/qti/index
// 変更点(Requirements3add):
//   - ルート要素を <assessmentItem> から <qti-assessment-item> に変更する
//   - 名前空間を http://www.imsglobal.org/xsd/imsqtiasi_v3p0 に変更する
//   - 全要素に qti- プレフィックスを付与する
//     (qti-response-declaration, qti-outcome-declaration, qti-item-body,
//      qti-choice-interaction, qti-simple-choice, qti-response-processing)
//   - 属性名をハイフン区切りに変更 (response-identifier, max-choices 等)
//   - 選択肢 ident をアルファベット (A, B, C, D ...) に変更する
//   - xml:lang="ja-JP" をルート要素に追加する
//   - responseProcessing をテンプレート形式に変更する
//   - 非標準の <metadata> ブロックを削除する

use crate::model::QuestionWithChoices;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use anyhow::{Context, Result};

/// 選択肢インデックスをアルファベット識別子に変換する (0→A, 1→B, ...)
fn choice_ident(index: usize) -> String {
    let c = char::from(b'A' + index as u8);
    c.to_string()
}

pub struct Qti30Exporter;

impl super::ExportHandler for Qti30Exporter {
    fn export(
        &self,
        questions: &[QuestionWithChoices],
        subject: &str,
        output_path: &PathBuf,
    ) -> Result<()> {
        let xml_content = generate_qti30_xml(questions, subject)?;

        let mut file = File::create(output_path)
            .with_context(|| format!("Failed to create file: {}", output_path.display()))?;

        file.write_all(xml_content.as_bytes())
            .with_context(|| format!("Failed to write to file: {}", output_path.display()))?;

        Ok(())
    }

    fn format_name(&self) -> &'static str {
        "QTI 3.0"
    }
}

/// QTI 3.0 形式でコンテンツを生成する
/// 複数問題はそれぞれ独立した <qti-assessment-item> として出力し、
/// XML 妥当性を保つため <qti-assessment-items> ルート要素で包む
fn generate_qti30_xml(questions: &[QuestionWithChoices], subject: &str) -> Result<String> {
    let mut xml = String::new();

    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<qti-assessment-items>\n");
    xml.push_str(&format!("  <!-- Subject: {} -->\n", escape_xml(subject)));

    for (idx, question) in questions.iter().enumerate() {
        xml.push_str(&generate_qti30_item(question, idx)?);
    }

    xml.push_str("</qti-assessment-items>\n");

    Ok(xml)
}

/// 単一の設問を QTI 3.0 形式で生成する
/// 構造は QTI 3.0.md サンプルに準拠する
fn generate_qti30_item(question: &QuestionWithChoices, index: usize) -> Result<String> {
    let mut xml = String::new();

    let item_id = format!("item{:03}", index + 1);

    // 正答のアルファベット識別子を特定する
    let correct_ident = question
        .choices
        .iter()
        .position(|c| c == &question.correct_answer)
        .map(choice_ident)
        .unwrap_or_else(|| "A".to_string());

    // <qti-assessment-item> ルート要素 (QTI 3.0.md のスキーマに準拠)
    xml.push_str("  <qti-assessment-item\n");
    xml.push_str("    xmlns=\"http://www.imsglobal.org/xsd/imsqtiasi_v3p0\"\n");
    xml.push_str("    xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"\n");
    xml.push_str("    xsi:schemaLocation=\"http://www.imsglobal.org/xsd/imsqtiasi_v3p0\n");
    xml.push_str("    https://purl.imsglobal.org/spec/qti/v3p0/schema/xsd/imsqti_asiv3p0p1_v1p0.xsd\"\n");
    xml.push_str(&format!(
        "    identifier=\"{}\"\n    title=\"{}\"\n    time-dependent=\"false\"\n    xml:lang=\"ja-JP\">\n",
        item_id,
        escape_xml(&question.question_no)
    ));

    // 正答宣言 (qti- プレフィックス付き)
    xml.push_str(
        "    <qti-response-declaration\n      identifier=\"RESPONSE\"\n      cardinality=\"single\"\n      base-type=\"identifier\">\n"
    );
    xml.push_str("      <qti-correct-response>\n");
    xml.push_str(&format!(
        "        <qti-value>{}</qti-value>\n",
        correct_ident
    ));
    xml.push_str("      </qti-correct-response>\n");
    xml.push_str("    </qti-response-declaration>\n");

    // スコア宣言 (qti- プレフィックス付き)
    xml.push_str(
        "    <qti-outcome-declaration\n      identifier=\"SCORE\"\n      cardinality=\"single\"\n      base-type=\"float\">\n"
    );
    xml.push_str("      <qti-default-value>\n");
    xml.push_str("        <qti-value>0</qti-value>\n");
    xml.push_str("      </qti-default-value>\n");
    xml.push_str("    </qti-outcome-declaration>\n");

    // 問題本文と選択肢 (qti- プレフィックス付き)
    xml.push_str("    <qti-item-body>\n");
    xml.push_str(&format!(
        "      <p>{}</p>\n",
        escape_xml(&question.body)
    ));
    xml.push_str(
        "      <qti-choice-interaction\n        response-identifier=\"RESPONSE\"\n        max-choices=\"1\">\n"
    );

    for (idx, choice) in question.choices.iter().enumerate() {
        let ident = choice_ident(idx);
        xml.push_str(&format!(
            "        <qti-simple-choice identifier=\"{}\">{}</qti-simple-choice>\n",
            ident,
            escape_xml(choice)
        ));
    }

    xml.push_str("      </qti-choice-interaction>\n");
    xml.push_str("    </qti-item-body>\n");

    // テンプレートによる応答処理 (QTI 3.0 用 URL に準拠)
    xml.push_str(
        "    <qti-response-processing\n      template=\"https://purl.imsglobal.org/spec/qti/v3p0/rptemplates/match_correct\"/>\n"
    );

    xml.push_str("  </qti-assessment-item>\n");

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
            body: "『吾輩は猫である』の作者は誰ですか。".to_string(),
            correct_answer: "夏目漱石".to_string(),
            choices: vec![
                "夏目漱石".to_string(),
                "森鴎外".to_string(),
                "芥川龍之介".to_string(),
                "太宰治".to_string(),
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
    fn qti30_uses_qti_prefixed_root_element() {
        let q = sample_question();
        let xml = generate_qti30_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("<qti-assessment-item"));
        assert!(!xml.contains("<assessmentItem"));
    }

    #[test]
    fn qti30_uses_correct_namespace() {
        let q = sample_question();
        let xml = generate_qti30_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("http://www.imsglobal.org/xsd/imsqtiasi_v3p0"));
    }

    #[test]
    fn qti30_has_qti_response_declaration() {
        let q = sample_question();
        let xml = generate_qti30_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("<qti-response-declaration"));
        assert!(xml.contains("<qti-correct-response>"));
        assert!(xml.contains("<qti-value>A</qti-value>"));
    }

    #[test]
    fn qti30_uses_alphabetic_choice_idents() {
        let q = sample_question();
        let xml = generate_qti30_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("identifier=\"A\""));
        assert!(xml.contains("identifier=\"B\""));
        assert!(!xml.contains("identifier=\"choice_1\""));
    }

    #[test]
    fn qti30_uses_qti30_template_url() {
        let q = sample_question();
        let xml = generate_qti30_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("purl.imsglobal.org/spec/qti/v3p0/rptemplates/match_correct"));
    }

    #[test]
    fn qti30_has_xml_lang() {
        let q = sample_question();
        let xml = generate_qti30_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("xml:lang=\"ja-JP\""));
    }

    #[test]
    fn qti30_uses_hyphenated_attributes() {
        let q = sample_question();
        let xml = generate_qti30_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("response-identifier=\"RESPONSE\""));
        assert!(xml.contains("max-choices=\"1\""));
    }
}
