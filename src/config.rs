use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub project: ProjectConfig,
    pub ai: AIConfig,
    pub analysis: AnalysisConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub path: PathBuf,
    pub ignore: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIConfig {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub parallel: bool,
    pub depth: u32,
    pub extensions: Vec<String>,
}
