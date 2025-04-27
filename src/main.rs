use clap::{Parser, Subcommand};
use magneta::TorrentSite;
use magneta::sites::TorrentTop;
use prettytable::{Table, row, format};

/// Magneta - Multi-site torrent search CLI
#[derive(Parser)]
#[command(name = "magneta")]
#[command(version = "0.1.0")]
#[command(about = "CLI tool to search torrents from multiple sites", long_about = None)]
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
                // add more sites here
            ];
            
            let mut results = Vec::new();
            for site in sites {
                let mut res = site.search(keyword).await?;
                results.append(&mut res);
            }

            if results.is_empty() {
                println!("No results found for keyword: {}", keyword);
                return Ok(());
            }

            // Create table
            let mut table = Table::new();
            table.set_titles(row!["Title", "Magnet"]);
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

            // Only add non-empty title/magnet rows
            for r in results {
                if !r.title.trim().is_empty() && !r.magnet.trim().is_empty() {
                    table.add_row(row![r.title, r.magnet]);
                }
            }

            // Print table
            table.printstd();
        }
    }

    Ok(())
}
