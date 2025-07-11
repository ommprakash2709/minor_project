#[derive(clap::Args)]
pub struct Args {
    pub filename: PathBuf,
    #[arg(long, default_value = ".")] pub dest: PathBuf,
}

pub fn run(a: Args) -> anyhow::Result<()> {
    let recovered = deduper_engine::quarantine::recover(&a.filename, &a.dest)?;
    println!("Recovered to {}", recovered.display());
    Ok(())
}
