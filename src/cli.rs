use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize new project
    Init {
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Run analysis
    Analyze {
        #[arg(short, long)]
        path: String,
        #[arg(short, long)]
        ai: bool,
    },
    /// Watch for changes
    Watch {
        #[arg(short, long)]
        path: String,
    },
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            Commands::Init { path } => {
                commands::init::execute(path).await
            }
            Commands::Analyze { path, ai } => {
                commands::analyze::execute(path, ai).await
            }
            Commands::Watch { path } => {
                commands::watch::execute(path).await
            }
        }
    }
}
