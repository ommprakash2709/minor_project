use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum, Args};
use deduper_engine::{filtering::Filter, hashing, quarantine, scan_directory, FileEntry};
use regex::Regex;
use std::{collections::HashMap, fs, path::Path};

#[derive(Parser)]
#[command(author, version, about = "Intelligent File Deduplicator")]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all files in a directory.
    Find(FindArgs),
    /// Hash matching files and (optionally) dump JSON report.
    Scan(ScanArgs),
    /// Detect duplicates and move them to quarantine.
    Quarantine(QuarantineArgs),
    /// Restore a single file from quarantine.
    Recover(RecoverArgs),
}

#[derive(Args)]
struct FindArgs {
    path: Option<String>,
}

#[derive(Args)]
struct ScanArgs {
    path: Option<String>,
    #[arg(long, default_value_t = 0)]
    min_size: u64,
    #[arg(long, default_value = "txt")]
    ext: String,
    #[arg(long, default_value = ".*")]
    pattern: String,
    #[arg(long, value_enum, default_value_t = HashAlgo::Sha256)]
    algo: HashAlgo,
    #[arg(long)]
    output: Option<String>,
}

#[derive(Args)]
struct QuarantineArgs {
    path: Option<String>,
    #[arg(long, default_value_t = 0)]
    min_size: u64,
    #[arg(long, default_value = "txt")]
    ext: String,
    #[arg(long, default_value = ".*")]
    pattern: String,
}

#[derive(Args)]
struct RecoverArgs {
    file: String,
}

#[derive(ValueEnum, Clone, Copy)]
enum HashAlgo {
    Sha256,
    Blake3,
    Xxh3,
}

impl From<HashAlgo> for hashing::Algo {
    fn from(a: HashAlgo) -> Self {
        match a {
            HashAlgo::Sha256 => hashing::Algo::Sha256,
            HashAlgo::Blake3 => hashing::Algo::Blake3,
            HashAlgo::Xxh3 => hashing::Algo::Xxh3,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        // ---------------- find ----------------
        Commands::Find(args) => {
            let root = args.path.unwrap_or_else(|| ".".to_string());
            for e in walkdir::WalkDir::new(&root)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                println!("{}", e.path().display());
            }
        }

        // ---------------- scan ----------------
        Commands::Scan(args) => {
            let root = args.path.unwrap_or_else(|| ".".to_string());
            let filter = Filter {
                min_size: args.min_size,
                max_size: None,
                ext: Some(args.ext),
                pattern: Regex::new(&args.pattern)?,
                since: None,
            };
            let entries = scan_directory(Path::new(&root), &filter, args.algo.into())?;
            println!("Hashed {} files", entries.len());

            if let Some(out) = args.output {
                fs::write(&out, serde_json::to_string_pretty(&entries)?)?;
                println!("Report written to {}", out);
            }
        }

        // --------------- quarantine -----------
        Commands::Quarantine(args) => {
            let root = args.path.unwrap_or_else(|| ".".to_string());
            let filter = Filter {
                min_size: args.min_size,
                max_size: None,
                ext: Some(args.ext),
                pattern: Regex::new(&args.pattern)?,
                since: None,
            };
            let entries = scan_directory(Path::new(&root), &filter, hashing::Algo::Sha256)?;
            move_duplicates(&entries)?;
        }

        // ---------------- recover -------------
        Commands::Recover(args) => {
            let dest = quarantine::recover(&args.file)?;
            println!("Recovered to {}", dest.display());
        }
    }
    Ok(())
}

/// Move every duplicate (same hash) to quarantine dir.
fn move_duplicates(entries: &[FileEntry]) -> Result<()> {
    let mut seen: HashMap<&str, &str> = HashMap::new();
    for e in entries {
        if let Some(orig) = seen.get(e.hash.as_str()) {
            let dest = quarantine::quarantine(Path::new(&e.path))?;
            println!("Duplicate of {} quarantined as {}", orig, dest.display());
        } else {
            seen.insert(e.hash.as_str(), e.path.as_str());
        }
    }
    Ok(())
}
