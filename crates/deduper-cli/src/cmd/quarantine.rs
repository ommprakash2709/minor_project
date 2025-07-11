#[derive(clap::Args)]
pub struct Args {
    #[arg(default_value = ".")] pub root: PathBuf,
    #[arg(long, default_value = "quarantine")] pub quarantine_dir: PathBuf,
}

pub fn run(a: Args) -> anyhow::Result<()> {
    // reuse scan logic to obtain hashes
    let entries = super::scan::scan_to_vec(&a.root)?;
    let mut seen = HashMap::new();
    for entry in &entries {
        if let Some(orig) = seen.get(&entry.hash) {
            deduper_engine::quarantine::quarantine(
                &entry.path,
                &a.quarantine_dir
            )?;
            println!("→ quarantined duplicate of {}", orig.display());
        } else {
            seen.insert(entry.hash.clone(), entry.path.clone());
        }
    }
    Ok(())
}
