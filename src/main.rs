use clap::{Parser, Subcommand};
use anyhow::Result;

mod cli;
mod tui;
mod cleanup;
mod discovery;
mod utils;

#[derive(Parser)]
#[command(name = "safe-clean")]
#[command(about = "Safe disk cleanup CLI/TUI tool")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch interactive TUI mode
    Tui,
    /// Cleanup Docker resources safely
    Docker {
        /// Show what would be cleaned without actually removing
        #[arg(long)]
        dry_run: bool,
    },
    /// Cleanup system temporary folders
    Temp {
        /// Show what would be cleaned without actually removing
        #[arg(long)]
        dry_run: bool,
    },
    /// List directories with sizes for selective cleanup
    List {
        /// Path to analyze (default: current directory)
        path: Option<String>,
        /// Show top N largest items
        #[arg(short, long, default_value = "20")]
        top: usize,
    },
    /// Find large files and directories
    Large {
        /// Path to search (default: current directory)
        path: Option<String>,
        /// Minimum size threshold (e.g., "100MB", "1GB")
        #[arg(short, long, default_value = "100MB")]
        size: String,
    },
    /// Discover and cleanup development artifacts (node_modules, .venv)
    DevClean {
        /// Path to search (default: current directory)
        path: Option<String>,
        /// Show what would be cleaned without actually removing
        #[arg(long)]
        dry_run: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Tui) => {
            tui::run().await?;
        }
        Some(Commands::Docker { dry_run }) => {
            cleanup::docker::cleanup(dry_run).await?;
        }
        Some(Commands::Temp { dry_run }) => {
            cleanup::temp::cleanup(dry_run).await?;
        }
        Some(Commands::List { path, top }) => {
            cli::list::run(path, top).await?;
        }
        Some(Commands::Large { path, size }) => {
            cli::large::run(path, size).await?;
        }
        Some(Commands::DevClean { path, dry_run }) => {
            cleanup::dev::cleanup(path, dry_run).await?;
        }
        None => {
            // No subcommand provided, launch TUI by default
            tui::run().await?;
        }
    }

    Ok(())
}
