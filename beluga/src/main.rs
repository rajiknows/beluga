mod beluga;

use crate::beluga::cli as luga;
use anyhow::Context;
use clap::CommandFactory;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Beluga")]
#[command(about = "A cute squishy static site generator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new site
    Init {
        #[arg(default_value = "my-site")]
        name: String,
    },
    /// Watch for changes and rebuild
    Watch,
    /// Show help
    Help,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { name } => {
            println!("Initializing site: {}", name);
            luga::create(name)?;
            Ok(())
        }
        Commands::Watch => {
            luga::watch(".").unwrap();
            Ok(())
        }
        Commands::Help => {
            Cli::command().print_help().unwrap();
            println!();
            Ok(())
        }
    }
}
