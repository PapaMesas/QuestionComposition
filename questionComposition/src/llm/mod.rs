// llm/mod.rs
// 目的: 各 LLM プロバイダーへの共通インターフェースを定義する。
// 3プロバイダー (Gemini, OpenAI, Claude) に同一の呼び出し方法を提供する。

pub mod claude;
pub mod gemini;
pub mod openai;

use anyhow::Result;

/// LLM への生成リクエストと結果
pub struct LlmRequest {
    /// LLM に渡すシステムプロンプト
    pub system_prompt: String,
    /// LLM に渡すユーザーメッセージ
    pub user_message: String,
}

/// 各プロバイダーが実装するトレイト
pub trait LlmClient: Send + Sync {
    /// プロンプトを送り、レスポンステキストを返す (同期ブロッキング)
    fn complete(&self, request: &LlmRequest) -> Result<String>;
}

/// プロバイダー種別と APIキーから適切な LlmClient を生成するファクトリ
pub fn create_client(provider: &crate::model::LlmProvider, api_key: &str) -> Box<dyn LlmClient> {
    match provider {
        crate::model::LlmProvider::Gemini => {
            Box::new(gemini::GeminiClient::new(api_key.to_string()))
        }
        crate::model::LlmProvider::OpenAI => {
            Box::new(openai::OpenAiClient::new(api_key.to_string()))
        }
        crate::model::LlmProvider::Claude => {
            Box::new(claude::ClaudeClient::new(api_key.to_string()))
        }
    }
}
