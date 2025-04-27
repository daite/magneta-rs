use clap::{Parser, Subcommand};
use prettytable::{Table, format, row};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use config::Config;
use std::env;
use std::path::PathBuf;

mod site_registry;

/// CLI argument parsing
#[derive(Parser)]
#[command(name = "magneta")]
#[command(version = "0.1.0")]
#[command(about = "Multi-site torrent search CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for a torrent using a keyword
    Search {
        /// The keyword to search for
        keyword: String,
    },
}

/// Load configuration from config.toml located next to the binary
fn get_config() -> anyhow::Result<Config> {
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path.parent()
        .map(|p| p.to_path_buf())  
        .unwrap_or_else(|| PathBuf::from(".")); 

    let config_path = exe_dir.join("config.toml");

    Config::builder()
        .add_source(config::File::from(config_path))
        .build()
        .map_err(Into::into)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let settings = get_config()?;

    match &cli.command {
        Commands::Search { keyword } => {
            // Create sites based on config
            let sites = site_registry::create_sites(&settings);

            // Create a spinner
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} {msg}")
                    .unwrap()
                    .tick_strings(&[
                        "⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"
                    ])
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            pb.set_message("Searching torrents...");

            let mut results = Vec::new();
            for site in sites {
                let mut res = site.search(keyword).await?;
                results.append(&mut res);
            }

            pb.finish_with_message("✅ Search completed!");

            if results.is_empty() {
                println!("No results found for keyword: {}", keyword);
                return Ok(());
            }

            let mut table = Table::new();
            table.set_titles(row!["Title", "Magnet"]);
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

            for r in results {
                if !r.title.trim().is_empty() && !r.magnet.trim().is_empty() {
                    table.add_row(row![r.title, r.magnet]);
                }
            }

            table.printstd();
        }
    }

    Ok(())
}