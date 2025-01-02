use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("AI error: {0}")]
    AI(String),
    
    #[error("Analysis error: {0}")]
    Analysis(String),
}
