// model.rs
// 目的: アプリ全体で使用するデータ型を定義する。
// Excel から読み込んだ設問情報と、LLM が生成した選択肢情報を表現する。

use serde::{Deserialize, Serialize};

/// Excel から取り込んだ1設問分のデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    /// 通し番号 (1始まり)
    pub no: u32,
    /// 問題番号 (例: "1-1")
    pub question_no: String,
    /// 問題本文
    pub body: String,
    /// 模範解答
    pub correct_answer: String,
    /// 画面上で設定した生成選択肢数 (デフォルト: 4)
    pub num_choices: u32,
}

/// LLM が生成した選択肢を含む1設問分の出力データ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionWithChoices {
    /// 通し番号
    pub no: u32,
    /// 問題番号
    pub question_no: String,
    /// 問題本文
    pub body: String,
    /// 模範解答
    pub correct_answer: String,
    /// 生成された選択肢 (模範解答を含む、num_choices 個)
    pub choices: Vec<String>,
    /// 生成に使用されたガイドライン名
    pub guideline: String,
}

/// 取り込んだ Excel 全体のデータ (科目名 + 設問リスト)
#[derive(Debug, Clone, Default)]
pub struct QuestionSheet {
    /// 科目名 (Excel 1行目)
    pub subject: String,
    /// 取り込んだ設問リスト
    pub questions: Vec<Question>,
}

/// LLM プロバイダーの種別
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum LlmProvider {
    #[default]
    Gemini,
    OpenAI,
    Claude,
}

impl std::fmt::Display for LlmProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LlmProvider::Gemini => write!(f, "Gemini"),
            LlmProvider::OpenAI => write!(f, "ChatGPT (OpenAI)"),
            LlmProvider::Claude => write!(f, "Claude (Anthropic)"),
        }
    }
}
