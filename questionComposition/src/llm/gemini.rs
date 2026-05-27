// llm/gemini.rs
// 目的: Google Gemini REST API を呼び出して選択肢を生成する。
// エンドポイント: generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::{json, Value};

use super::{LlmClient, LlmRequest};

pub struct GeminiClient {
    api_key: String,
    http: Client,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http: Client::new(),
        }
    }
}

impl LlmClient for GeminiClient {
    fn complete(&self, request: &LlmRequest) -> Result<String> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
            self.api_key
        );

        // Gemini API のリクエスト本文を構築する
        let body = json!({
            "system_instruction": {
                "parts": [{ "text": request.system_prompt }]
            },
            "contents": [{
                "parts": [{ "text": request.user_message }]
            }],
            "generationConfig": {
                "temperature": 0.7,
                "maxOutputTokens": 2048
            }
        });

        let resp: Value = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .context("Gemini API request failed")?
            .error_for_status()
            .context("Gemini API returned error status")?
            .json()
            .context("Failed to parse Gemini response")?;

        // レスポンスから生成テキストを抽出する
        let text = resp["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .context("Unexpected Gemini response structure")?
            .to_string();

        Ok(text)
    }
}
