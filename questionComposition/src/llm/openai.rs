// llm/openai.rs
// 目的: OpenAI Chat Completions API を呼び出して選択肢を生成する。
// エンドポイント: api.openai.com/v1/chat/completions

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::{json, Value};

use super::{LlmClient, LlmRequest};

pub struct OpenAiClient {
    api_key: String,
    http: Client,
}

impl OpenAiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http: Client::new(),
        }
    }
}

impl LlmClient for OpenAiClient {
    fn complete(&self, request: &LlmRequest) -> Result<String> {
        let url = "https://api.openai.com/v1/chat/completions";

        // OpenAI API のリクエスト本文を構築する
        let body = json!({
            "model": "gpt-4o-mini",
            "messages": [
                { "role": "system", "content": request.system_prompt },
                { "role": "user",   "content": request.user_message }
            ],
            "temperature": 0.7,
            "max_tokens": 2048
        });

        let resp: Value = self
            .http
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .context("OpenAI API request failed")?
            .error_for_status()
            .context("OpenAI API returned error status")?
            .json()
            .context("Failed to parse OpenAI response")?;

        // レスポンスから生成テキストを抽出する
        let text = resp["choices"][0]["message"]["content"]
            .as_str()
            .context("Unexpected OpenAI response structure")?
            .to_string();

        Ok(text)
    }
}
