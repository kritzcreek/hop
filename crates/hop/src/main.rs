mod proto;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};

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

    /// Lists the paths of documents contained within the given index as json
    ListFiles { input: Option<PathBuf> },
}

fn read_input(input: Option<&Path>) -> Result<Vec<u8>> {
    match input {
        Some(file) => {
            fs::read(file).context(format!("Failed to read input file at {}", file.display()))
        }
        None => {
            let mut buffer: Vec<u8> = Vec::new();
            std::io::stdin().read_to_end(&mut buffer)?;
            Ok(buffer)
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Print { input, path } => {
            let bytes = read_input(input.as_deref())?;
            if let Some(index) = proto::read_index(&bytes) {
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
            } else if let Some(document) = proto::read_document(&bytes) {
                if let Some(_) = path {
                    bail!("Cannot filter by --path when searching a single encoded document");
                }
                println!("{}", serde_json::to_string(&document)?);
            } else {
                bail!("Could not parse input as either an index or a document");
            }
            Ok(())
        }
        Commands::ListFiles { input } => {
            let bytes = read_input(input.as_deref())?;
            if let Some(index) = proto::read_index(&bytes) {
                let mut paths: Vec<_> = index.documents.iter().map(|d| &d.relative_path).collect();
                paths.sort();
                println!("{}", serde_json::to_string(&paths)?);
            } else if let Some(document) = proto::read_document(&bytes) {
                println!("{}", serde_json::to_string(&vec![document.relative_path])?);
            } else {
                bail!("Could not parse input as either an index or a document")
            }
            Ok(())
        }
    }
}
