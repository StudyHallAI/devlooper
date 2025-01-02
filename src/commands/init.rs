use anyhow::Result;
use std::path::PathBuf;
use crate::config::{Config, ProjectConfig, AIConfig, AnalysisConfig};
use tracing::{info, warn};

pub async fn execute(path: Option<String>) -> Result<()> {
    let path = path.map(PathBuf::from).unwrap_or_else(|| {
        std::env::current_dir().expect("Failed to get current directory")
    });
    
    info!("Initializing project at {:?}", path);
    
    // Create config
    let config = Config {
        project: ProjectConfig {
            name: path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("project")
                .to_string(),
            path: path.clone(),
            ignore: vec![".git", "target", "node_modules"].into_iter()
                .map(String::from)
                .collect(),
        },
        ai: AIConfig {
            model: "gpt-4".to_string(),
            temperature: 0.7,
            max_tokens: 1000,
        },
        analysis: AnalysisConfig {
            parallel: true,
            depth: 5,
            extensions: vec![".rs", ".toml", ".md"].into_iter()
                .map(String::from)
                .collect(),
        },
    };
    
    // Save config
    let config_path = path.join(".modern-cli.json");
    let config_str = serde_json::to_string_pretty(&config)?;
    std::fs::write(&config_path, config_str)?;
    
    info!("Created config at {:?}", config_path);
    
    Ok(())
}
