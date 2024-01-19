use anyhow::{Context, Result};
use reqwest::{
    blocking::Client,
    header,
};
use serde_json;
use std::time::Duration;

use crate::config;

pub struct Gpt {
    client: Client
}

impl Gpt {
    pub fn try_create(api_key: String) -> Result<Self> {
        let mut default_headers = header::HeaderMap::new();
        let token_value = header::HeaderValue::from_str(&format!("Bearer {}", api_key))?;
        default_headers.insert("AUTHORIZATION", token_value);
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .default_headers(default_headers)
            .build()?;
        Ok(Gpt { client })
    }

    pub fn create_vectors(&mut self, input: &str) -> Result<Vec<f32>> {
        // return vector with dimension of 1536
        assert!(
            input.len() <= config::MAX_INPUT_TOKENS,
            "Max input tokens for V1 should be at most 2046"
        );
        let body = serde_json::json!({
            "input": input,
            "model": config::MODEL_NAME
        });
        let resp = self
            .client
            .post(config::EMBEDDING_API_URL)
            .json(&body)
            .send()?;
        let data: serde_json::Value = resp.json()?;
        log::debug!("resp: {data:?}");
        let data: &Vec<serde_json::Value> =
            data["data"].as_array().context("data returned is not array")?;
        let data: &serde_json::Value = data.first().context("array is empty")?;
        let vector: &Vec<serde_json::Value> = data["embedding"]
            .as_array()
            .context("data returned does not have vectors")?;
        let vector: Result<Vec<f32>, _> = vector
            .iter()
            .map(|value| {
                value
                    .as_f64()
                    .map(|f| f as f32)
                    .context("Value is not a numeric type")
            })
            .collect();
        let vector = vector?;
        Ok(vector)
    }
}
