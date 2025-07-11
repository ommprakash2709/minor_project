#[derive(clap::Args)]
pub struct Args {
    #[arg(default_value = ".")]
    pub root: PathBuf,
}

pub fn run(args: Args) -> anyhow::Result<()> {
    for entry in WalkDir::new(args.root).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            println!("{}", entry.path().display());
        }
    }
    Ok(())
}
