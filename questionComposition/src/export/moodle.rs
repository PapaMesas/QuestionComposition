// export/moodle.rs
// 目的: Moodle XML 形式でのエクスポート機能を実装する。
// Moodle のネイティブ形式で、複数選択肢問題（multichoice）をサポート。
// 構造定義: RequirementsSpecification/Moodle XML.md に準拠。
// 変更点(Requirements3add):
//   - <text> の内容を CDATA で囲む
//   - <defaultgrade>, <penalty>, <hidden>, <single>, <shuffleanswers>, <answernumbering> を追加
//   - answer の format を "html" に変更
//   - 選択肢を格納順に出力し、正答に fraction="100"、誤答に fraction="0" を設定

use crate::model::QuestionWithChoices;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use anyhow::{Context, Result};

pub struct MoodleExporter;

impl super::ExportHandler for MoodleExporter {
    fn export(
        &self,
        questions: &[QuestionWithChoices],
        subject: &str,
        output_path: &PathBuf,
    ) -> Result<()> {
        let xml_content = generate_moodle_xml(questions, subject)?;

        let mut file = File::create(output_path)
            .with_context(|| format!("Failed to create file: {}", output_path.display()))?;

        file.write_all(xml_content.as_bytes())
            .with_context(|| format!("Failed to write to file: {}", output_path.display()))?;

        Ok(())
    }

    fn format_name(&self) -> &'static str {
        "Moodle XML"
    }
}

/// Moodle XML フォーマットのコンテンツを生成する
fn generate_moodle_xml(questions: &[QuestionWithChoices], subject: &str) -> Result<String> {
    let mut xml = String::new();

    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<quiz>\n");

    // 科目名をコメントとして記載
    xml.push_str(&format!("  <!-- Subject: {} -->\n", escape_xml(subject)));

    for question in questions {
        xml.push_str(&generate_multichoice_question(question)?);
    }

    xml.push_str("</quiz>\n");

    Ok(xml)
}

/// 単一の複数選択肢問題を Moodle XML 形式で生成する
/// 構造は Moodle XML.md サンプルに準拠する
fn generate_multichoice_question(question: &QuestionWithChoices) -> Result<String> {
    let mut xml = String::new();

    xml.push_str("  <question type=\"multichoice\">\n");

    // 問題名
    xml.push_str("    <name>\n");
    xml.push_str(&format!(
        "      <text>{}</text>\n",
        escape_xml(&question.question_no)
    ));
    xml.push_str("    </name>\n");

    // 問題テキスト (CDATA でラップ)
    xml.push_str("    <questiontext format=\"html\">\n");
    xml.push_str(&format!(
        "      <text><![CDATA[{}]]></text>\n",
        question.body
    ));
    xml.push_str("    </questiontext>\n");

    // Moodle XML.md で定義された必須フィールド群
    xml.push_str("    <defaultgrade>1.0000000</defaultgrade>\n");
    xml.push_str("    <penalty>0.3333333</penalty>\n");
    xml.push_str("    <hidden>0</hidden>\n");
    xml.push_str("    <single>true</single>\n");
    xml.push_str("    <shuffleanswers>true</shuffleanswers>\n");
    xml.push_str("    <answernumbering>abc</answernumbering>\n");

    // ガイドライン番号をフィードバックとして記録する
    if !question.guideline.is_empty() {
        xml.push_str("    <generalfeedback format=\"html\">\n");
        xml.push_str(&format!(
            "      <text><![CDATA[ガイドライン: {}]]></text>\n",
            question.guideline
        ));
        xml.push_str("    </generalfeedback>\n");
    }

    // 全選択肢を格納順に出力する。正答は fraction="100"、誤答は fraction="0"
    for choice in &question.choices {
        let fraction = if choice == &question.correct_answer {
            "100"
        } else {
            "0"
        };
        xml.push_str(&format!(
            "    <answer fraction=\"{}\" format=\"html\">\n",
            fraction
        ));
        xml.push_str(&format!(
            "      <text><![CDATA[{}]]></text>\n",
            choice
        ));
        xml.push_str("      <feedback format=\"html\">\n");
        if fraction == "100" {
            xml.push_str("        <text><![CDATA[正解です。]]></text>\n");
        } else {
            xml.push_str("        <text></text>\n");
        }
        xml.push_str("      </feedback>\n");
        xml.push_str("    </answer>\n");
    }

    xml.push_str("  </question>\n");

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
    fn escape_xml_handles_special_chars() {
        assert_eq!(escape_xml("a & b"), "a &amp; b");
        assert_eq!(escape_xml("<tag>"), "&lt;tag&gt;");
        assert_eq!(escape_xml("quote\"test"), "quote&quot;test");
    }

    #[test]
    fn escape_xml_preserves_normal_text() {
        assert_eq!(escape_xml("hello world"), "hello world");
    }

    #[test]
    fn moodle_xml_contains_required_fields() {
        let q = sample_question();
        let xml = generate_moodle_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("<defaultgrade>1.0000000</defaultgrade>"));
        assert!(xml.contains("<penalty>0.3333333</penalty>"));
        assert!(xml.contains("<hidden>0</hidden>"));
        assert!(xml.contains("<single>true</single>"));
        assert!(xml.contains("<shuffleanswers>true</shuffleanswers>"));
        assert!(xml.contains("<answernumbering>abc</answernumbering>"));
    }

    #[test]
    fn moodle_xml_uses_cdata_for_text() {
        let q = sample_question();
        let xml = generate_moodle_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("<![CDATA[日本の首都はどこですか。]]>"));
        assert!(xml.contains("<![CDATA[東京]]>"));
    }

    #[test]
    fn moodle_xml_correct_answer_has_fraction_100() {
        let q = sample_question();
        let xml = generate_moodle_xml(&[q], "テスト科目").unwrap();
        // 正答（東京）が fraction="100" で出力されることを確認
        let correct_block_idx = xml.find("fraction=\"100\"").unwrap();
        assert!(xml[correct_block_idx..].contains("<![CDATA[東京]]>"));
    }

    #[test]
    fn moodle_xml_uses_html_format() {
        let q = sample_question();
        let xml = generate_moodle_xml(&[q], "テスト科目").unwrap();
        assert!(xml.contains("format=\"html\""));
        assert!(!xml.contains("format=\"moodle_auto_format\""));
    }
}
