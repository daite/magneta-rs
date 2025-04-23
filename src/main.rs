use clap::{Parser, Subcommand};
use magneta::{TorrentSite, TorrentResult};
use magneta::sites::TorrentTop;
use prettytable::{Table, row, cell};

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
            let mut table = Table::new();
            table.add_row(row!["TITLE", "MAGNET"]);
            for r in results {
                table.add_row(row![r.title, r.magnet]);
            }
            table.printstd();
        }
    }
    Ok(())
}