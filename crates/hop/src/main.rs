mod proto;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::{fs, io::Read, path::PathBuf};

/// The hop toolchain
#[derive(Debug, Parser)]
#[command(name = "hop")]
#[command(about = "Tools for working with SCIP", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Prints SCIP indices or documents as json
    Print {
        /// Read the scip data from here, if not provided, read from stdin
        input: Option<PathBuf>,
        #[arg(long)]
        path: Option<String>,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Print { input, path } => {
            let input = match input {
                Some(file) => fs::read(file)?,
                None => {
                    let mut buffer: Vec<u8> = Vec::new();
                    std::io::stdin().read_to_end(&mut buffer)?;
                    buffer
                }
            };
            if let Some(index) = proto::read_index(&input) {
                if let Some(path) = path {
                    let document = index
                        .documents
                        .iter()
                        .find(|d| d.relative_path == path)
                        .context(format!("Failed to find document at {path}"))?;
                    println!("{}", serde_json::to_string(document)?);
                } else {
                    println!("{}", serde_json::to_string(&index)?);
                }
            } else if let Some(document) = proto::read_document(&input) {
                if let Some(_) = path {
                    bail!("Cannot filter by --path when searching a single encoded document");
                }
                println!("{}", serde_json::to_string(&document)?);
            } else {
                bail!("Could not parse input as either an index or a document");
            }
            Ok(())
        }
    }
}
