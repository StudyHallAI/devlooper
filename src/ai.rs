use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use async_openai::{
    Client,
    types::{CreateCompletionRequestArgs, ChatCompletionRequestMessage},
};

#[async_trait]
pub trait Analyzer {
    async fn analyze(&self, content: &str) -> Result<Analysis>;
}

#[derive(Debug, Default)]
pub struct AIAnalyzer {
    client: Option<Client>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub suggestions: Vec<Suggestion>,
    pub metrics: Metrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Suggestion {
    pub category: String,
    pub message: String,
    pub line: Option<u32>,
    pub confidence: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    pub complexity: u32,
    pub maintainability: f32,
    pub safety_score: f32,
}

impl AIAnalyzer {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            client: Some(Client::new()),
        })
    }
    
    async fn get_completion(&self, prompt: &str) -> Result<String> {
        let client = self.client.as_ref()
            .expect("AI client not initialized");
            
        let request = CreateCompletionRequestArgs::default()
            .model("gpt-4")
            .prompt(prompt)
            .max_tokens(1000u16)
            .temperature(0.7)
            .build()?;
            
        let response = client.completions()
            .create(request)
            .await?;
            
        Ok(response.choices[0].text.clone())
    }
}

#[async_trait]
impl Analyzer for AIAnalyzer {
    async fn analyze(&self, content: &str) -> Result<Analysis> {
        if let Some(_) = self.client {
            // AI-powered analysis
            let prompt = format!(
                "Analyze this code for improvements:\n\n{}\n\n",
                content
            );
            
            let completion = self.get_completion(&prompt).await?;
            
            // Parse AI response
            let analysis = serde_json::from_str(&completion)
                .unwrap_or_else(|_| Analysis {
                    suggestions: vec![],
                    metrics: Metrics {
                        complexity: 0,
                        maintainability: 0.0,
                        safety_score: 0.0,
                    },
                });
                
            Ok(analysis)
        } else {
            // Basic analysis
            Ok(Analysis {
                suggestions: vec![],
                metrics: Metrics {
                    complexity: 0,
                    maintainability: 0.0,
                    safety_score: 0.0,
                },
            })
        }
    }
}
