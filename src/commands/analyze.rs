use anyhow::Result;
use std::path::PathBuf;
use crate::{ai, utils};
use indicatif::{ProgressBar, ProgressStyle};
use tracing::info;

pub async fn execute(path: String, use_ai: bool) -> Result<()> {
    let path = PathBuf::from(path);
    info!("Analyzing {:?} with AI: {}", path, use_ai);
    
    // Setup progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    
    // Collect files
    pb.set_message("Collecting files...");
    let files = utils::collect_files(&path)?;
    
    // Initialize analysis
    let analyzer = if use_ai {
        ai::AIAnalyzer::new().await?
    } else {
        ai::AIAnalyzer::default()
    };
    
    // Run analysis
    pb.set_message("Analyzing files...");
    let results = analyzer.analyze_files(&files).await?;
    
    // Display results
    pb.finish_with_message("Analysis complete!");
    utils::display_results(&results);
    
    Ok(())
}
