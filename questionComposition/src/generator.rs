// generator.rs
// 目的: LLM を呼び出して各設問の誤答選択肢を生成し、模範解答をランダムな位置に挿入して返す。
// プロンプト構造: [ルール] + [科目名・問題本文・模範解答・必要数] → JSON オブジェクトで誤答と
//                使用ガイドライン番号を返してもらう。
// 変更点(Requirements3add): guideline フィールドをファイル名ラベルから
//                           実際に適用したガイドライン番号の文字列("21, 27, 33")に変更。

use anyhow::{Context, Result};
use rand::Rng;
use serde_json::Value;

use crate::llm::{LlmClient, LlmRequest};
use crate::model::{Question, QuestionWithChoices};
use crate::rule_loader::RuleSet;

/// LLM レスポンスを解析した結果
struct LlmResponse {
    distractors: Vec<String>,
    guideline_numbers: Vec<u32>,
}

/// 1設問分の誤答選択肢を LLM に生成させ、QuestionWithChoices を返す
/// ガイドライン番号も同時に抽出して guideline フィールドに格納する
pub fn generate_choices(
    client: &dyn LlmClient,
    subject: &str,
    question: &Question,
    rules: &RuleSet,
) -> Result<QuestionWithChoices> {
    let num_distractors = (question.num_choices as usize).saturating_sub(1);

    // システムプロンプト: 選択肢生成ルールと回答形式を指定する
    let system_prompt = format!(
        "あなたは試験問題の誤答選択肢を生成する専門家です。\n\
         以下の選択肢生成ガイドラインに厳密に従って誤答を作成してください。\n\n\
         # ガイドライン\n\n{}\n\n\
         必ず以下の JSON オブジェクト形式のみで返してください（説明文不要）:\n\
         {{\n\
           \"distractors\": [\"誤答1\", \"誤答2\", ...],\n\
           \"guideline_numbers\": [適用したガイドライン番号のリスト (整数)]\n\
         }}",
        rules.content
    );

    // ユーザーメッセージ: 問題情報を渡して誤答と適用番号を要求する
    let user_message = format!(
        "科目名: {subject}\n\
         問題番号: {question_no}\n\
         問題本文: {body}\n\
         模範解答: {correct}\n\n\
         上記の問題に対して、もっともらしい誤答（ディストラクター）を {n} 個生成してください。\n\
         また、選択肢を生成する際に参照したガイドライン番号をすべて列挙してください。",
        subject = subject,
        question_no = question.question_no,
        body = question.body,
        correct = question.correct_answer,
        n = num_distractors,
    );

    let response = client
        .complete(&LlmRequest {
            system_prompt,
            user_message,
        })
        .context("LLM call failed")?;

    // LLM レスポンスから誤答リストとガイドライン番号を抽出する
    let parsed =
        parse_llm_response(&response).context("Failed to parse LLM response as JSON object")?;

    // 模範解答をランダムな位置に挿入して選択肢リストを完成させる
    let choices = insert_correct_at_random(
        parsed.distractors,
        &question.correct_answer,
        question.num_choices as usize,
    );

    // ガイドライン番号をカンマ区切りの文字列に変換する
    let guideline = format_guideline_numbers(&parsed.guideline_numbers);

    Ok(QuestionWithChoices {
        no: question.no,
        question_no: question.question_no.clone(),
        body: question.body.clone(),
        correct_answer: question.correct_answer.clone(),
        choices,
        guideline,
    })
}

/// LLM レスポンスから JSON オブジェクトを解析して distractors と guideline_numbers を返す
/// フォールバック: JSON オブジェクトが見つからない場合は配列形式として解析する
fn parse_llm_response(response: &str) -> Result<LlmResponse> {
    // レスポンス中の "{" から "}" を含む部分を抽出する (コードブロック対応)
    if let Some(obj_result) = try_parse_as_object(response) {
        return obj_result;
    }

    // フォールバック: 配列形式の場合はガイドライン番号なしで処理する
    let distractors = parse_json_array_from(response)?;
    Ok(LlmResponse {
        distractors,
        guideline_numbers: vec![],
    })
}

/// JSON オブジェクト形式として解析を試みる
fn try_parse_as_object(response: &str) -> Option<Result<LlmResponse>> {
    let start = response.find('{')?;
    let end = response.rfind('}')?;
    if start >= end {
        return None;
    }
    let json_str = &response[start..=end];

    let value: Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => return None,
    };

    let obj = value.as_object()?;

    let distractors = obj
        .get("distractors")?
        .as_array()?
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();

    let guideline_numbers = obj
        .get("guideline_numbers")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_u64().map(|n| n as u32))
                .collect()
        })
        .unwrap_or_default();

    Some(Ok(LlmResponse {
        distractors,
        guideline_numbers,
    }))
}

/// レスポンスから JSON 配列を抽出して Vec<String> を返す (フォールバック用)
fn parse_json_array_from(response: &str) -> Result<Vec<String>> {
    let start = response
        .find('[')
        .context("No JSON array found in response")?;
    let end = response
        .rfind(']')
        .context("No closing bracket found in response")?;
    let json_str = &response[start..=end];

    let value: Value = serde_json::from_str(json_str).context("Invalid JSON array")?;
    let arr = value.as_array().context("JSON value is not an array")?;

    let strings: Vec<String> = arr
        .iter()
        .filter_map(|v| v.as_str().map(|s| s.to_string()))
        .collect();

    Ok(strings)
}

/// ガイドライン番号リストをカンマ区切りの文字列に変換する
/// 番号が空の場合は空文字列を返す
fn format_guideline_numbers(numbers: &[u32]) -> String {
    if numbers.is_empty() {
        return String::new();
    }
    numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// 誤答リストの中にランダムな位置で模範解答を挿入する
fn insert_correct_at_random(
    mut distractors: Vec<String>,
    correct: &str,
    total: usize,
) -> Vec<String> {
    // 要求数より誤答が少ない場合は必要数に切り詰める (total - 1 個)
    distractors.truncate(total.saturating_sub(1));

    let mut rng = rand::thread_rng();
    let insert_pos = rng.gen_range(0..=distractors.len());
    distractors.insert(insert_pos, correct.to_string());
    distractors
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    // UT-11: JSON オブジェクト形式のレスポンスを正しく解析できる
    #[test]
    fn parses_json_object_response() {
        let response = r#"{"distractors":["誤答A","誤答B","誤答C"],"guideline_numbers":[21,27,33]}"#;
        let result = parse_llm_response(response).unwrap();
        assert_eq!(result.distractors, vec!["誤答A", "誤答B", "誤答C"]);
        assert_eq!(result.guideline_numbers, vec![21u32, 27, 33]);
    }

    // UT-12: コードブロック内の JSON オブジェクトを解析できる
    #[test]
    fn parses_json_object_inside_code_block() {
        let response = "以下が結果です:\n```json\n{\"distractors\":[\"誤答1\",\"誤答2\"],\"guideline_numbers\":[21,22]}\n```";
        let result = parse_llm_response(response).unwrap();
        assert_eq!(result.distractors, vec!["誤答1", "誤答2"]);
        assert_eq!(result.guideline_numbers, vec![21u32, 22]);
    }

    // UT-13: 配列形式のフォールバック解析が動作する
    #[test]
    fn falls_back_to_array_format() {
        let response = r#"["誤答A","誤答B","誤答C"]"#;
        let result = parse_llm_response(response).unwrap();
        assert_eq!(result.distractors, vec!["誤答A", "誤答B", "誤答C"]);
        assert!(result.guideline_numbers.is_empty());
    }

    // UT-14: ガイドライン番号リストをカンマ区切り文字列に変換できる
    #[test]
    fn formats_guideline_numbers_correctly() {
        assert_eq!(format_guideline_numbers(&[21, 27, 33]), "21, 27, 33");
        assert_eq!(format_guideline_numbers(&[5]), "5");
        assert_eq!(format_guideline_numbers(&[]), "");
    }

    // UT-15: 模範解答がランダム位置に挿入される (100回試行して位置が複数パターンになる)
    #[test]
    fn correct_answer_inserted_at_varying_positions() {
        let correct = "正答";
        let mut positions = std::collections::HashSet::new();
        for _ in 0..100 {
            let distractors = vec!["誤答1".to_string(), "誤答2".to_string(), "誤答3".to_string()];
            let choices = insert_correct_at_random(distractors, correct, 4);
            let pos = choices.iter().position(|c| c == correct).unwrap();
            positions.insert(pos);
        }
        assert!(positions.len() > 1, "正答の挿入位置がランダムになっていない");
    }

    // UT-16: 挿入後の選択肢数が num_choices と一致する
    #[test]
    fn choices_count_matches_num_choices() {
        for total in [2usize, 3, 4, 5, 6] {
            let distractors: Vec<String> =
                (0..total - 1).map(|i| format!("誤答{}", i)).collect();
            let choices = insert_correct_at_random(distractors, "正答", total);
            assert_eq!(choices.len(), total, "num_choices={} で選択肢数が一致しない", total);
        }
    }

    // UT-17: モック LLM クライアントを使った generate_choices の統合確認 (JSON オブジェクト形式)
    #[test]
    fn generate_choices_uses_llm_json_object_response() -> Result<()> {
        struct MockClient;
        impl crate::llm::LlmClient for MockClient {
            fn complete(&self, _req: &crate::llm::LlmRequest) -> Result<String> {
                Ok(r#"{"distractors":["誤答A","誤答B","誤答C"],"guideline_numbers":[21,27]}"#.to_string())
            }
        }

        let question = crate::model::Question {
            no: 1,
            question_no: "Q1".to_string(),
            body: "問題本文".to_string(),
            correct_answer: "正答".to_string(),
            num_choices: 4,
        };
        let rules = crate::rule_loader::RuleSet {
            content: "テストルール".to_string(),
            source_label: "test_guideline.md".to_string(),
        };

        let result = generate_choices(&MockClient, "テスト科目", &question, &rules)?;
        assert_eq!(result.choices.len(), 4);
        assert!(result.choices.contains(&"正答".to_string()));
        assert_eq!(result.guideline, "21, 27");
        Ok(())
    }
}
