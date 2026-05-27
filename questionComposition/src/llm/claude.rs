// llm/claude.rs
// 目的: Anthropic Claude Messages API を呼び出して選択肢を生成する。
// エンドポイント: api.anthropic.com/v1/messages

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::{json, Value};

use super::{LlmClient, LlmRequest};

pub struct ClaudeClient {
    api_key: String,
    http: Client,
}

impl ClaudeClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http: Client::new(),
        }
    }
}

impl LlmClient for ClaudeClient {
    fn complete(&self, request: &LlmRequest) -> Result<String> {
        let url = "https://api.anthropic.com/v1/messages";

        // Claude API のリクエスト本文を構築する
        let body = json!({
            "model": "claude-haiku-4-5-20251001",
            "max_tokens": 2048,
            "system": request.system_prompt,
            "messages": [
                { "role": "user", "content": request.user_message }
            ]
        });

        let resp: Value = self
            .http
            .post(url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .context("Claude API request failed")?
            .error_for_status()
            .context("Claude API returned error status")?
            .json()
            .context("Failed to parse Claude response")?;

        // レスポンスから生成テキストを抽出する
        let text = resp["content"][0]["text"]
            .as_str()
            .context("Unexpected Claude response structure")?
            .to_string();

        Ok(text)
    }
}
