use anyhow::Result;
use clap::Parser;
mod cli;
mod commands;
mod config;
mod error;
mod ai;
mod ui;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Parse command line
    let cli = cli::Cli::parse();
    
    // Execute command
    cli.execute().await
}
