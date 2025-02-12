// src/config.rs
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub git: GitConfig,
    pub llm: LLMConfig,
    pub display: DisplayConfig,
}

#[derive(Debug, Deserialize)]
pub struct GitConfig {
    pub repo_path: String,
    pub ignore_patterns: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct LLMConfig {
    pub provider: String,
    pub api_key: String,
    pub rate_limit: usize,
    pub window_ms: u64,
    pub timeout: Duration,
}

#[derive(Debug, Deserialize)]
pub struct DisplayConfig {
    pub color_enabled: bool,
    pub line_numbers: bool,
    pub context_lines: usize,
}