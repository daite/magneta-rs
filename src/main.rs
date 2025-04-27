use clap::{Parser, Subcommand};
use prettytable::{Table, format, row};
use magneta::TorrentSite;
use magneta::sites::TorrentTop;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Search { keyword } => {
            let sites: Vec<Box<dyn TorrentSite>> = vec![
                Box::new(TorrentTop),
                // Add more sites here
            ];

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