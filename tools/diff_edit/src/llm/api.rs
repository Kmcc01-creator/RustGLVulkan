
// src/llm/api.rs
use async_trait::async_trait;
use reqwest::{Client, header};
use serde_json::Value;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct LLMConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub timeout: Duration,
    pub max_retries: u32,
    pub batch_size: usize,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.openai.com/v1".to_string(),
            model: "gpt-4".to_string(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            batch_size: 4096,
        }
    }
}

#[derive(Debug)]
pub struct LLMAPIClient {
    config: LLMConfig,
    client: Client,
    templates: PromptTemplates,
}

impl LLMAPIClient {
    pub fn new(config: LLMConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("Bearer {}", config.api_key))?,
        );

        let client = Client::builder()
            .timeout(config.timeout)
            .default_headers(headers)
            .build()?;

        Ok(Self {
            config,
            client,
            templates: PromptTemplates::default(),
        })
    }

    pub async fn analyze_diff(
        &self,
        diff_analysis: &DiffAnalysis,
    ) -> Result<LLMResponse, Box<dyn std::error::Error>> {
        let template = self.templates.get_template("diff_analysis")?;
        let context = self.build_template_context(diff_analysis)?;
        
        let prompt = template.render(&context)?;
        
        self.make_api_request(&prompt).await
    }

    async fn make_api_request(
        &self,
        prompt: &str,
    ) -> Result<LLMResponse, Box<dyn std::error::Error>> {
        let mut retries = 0;
        let mut last_error = None;

        while retries < self.config.max_retries {
            match self.execute_request(prompt).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    last_error = Some(e);
                    retries += 1;
                    tokio::time::sleep(Duration::from_secs(2u64.pow(retries))).await;
                }
            }
        }

        Err(last_error.unwrap())
    }

    async fn execute_request(
        &self,
        prompt: &str,
    ) -> Result<LLMResponse, Box<dyn std::error::Error>> {
        let response = self.client
            .post(&format!("{}/chat/completions", self.config.base_url))
            .json(&serde_json::json!({
                "model": self.config.model,
                "messages": [
                    {
                        "role": "system",
                        "content": "You are an expert code reviewer analyzing code changes."
                    },
                    {
                        "role": "user",
                        "content": prompt
                    }
                ],
                "temperature": 0.3,
                "max_tokens": self.config.batch_size
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!(
                "API request failed: {} - {}",
                response.status(),
                response.text().await?
            ).into());
        }

        let response_data: Value = response.json().await?;
        self.parse_llm_response(&response_data)
    }
}
