use deduper_engine::{hashing::Algo, filter::Filter, report::JsonReport};
use crate::progress::bar_for_len;

#[derive(clap::Args)]
pub struct Args {
    #[arg(default_value = ".")] pub root: PathBuf,
    #[arg(long, default_value_t = 1024)] pub min_size: u64,
    #[arg(long, value_enum, default_value_t = Algo::Blake3)] pub algo: Algo,
    #[arg(long)] pub json_out: Option<PathBuf>,
}

pub fn run(a: Args) -> anyhow::Result<()> {
    let filter = Filter { min_size: a.min_size, ..Default::default() };
    let mut entries = Vec::new();

    // first pass: collect files with metadata
    let walker: Vec<_> = WalkDir::new(&a.root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .collect();

    let pb = bar_for_len(walker.len() as u64, "Hashing");
    entries = walker.par_iter()
        .filter(|e| filter.matches(&e.metadata().unwrap(), e.path()))
        .map(|e| {
            let h = hashing::hash_file(e.path(), a.algo).unwrap_or_default();
            pb.inc(1);
            FileEntry { path: e.path().to_path_buf(), hash: h }
        })
        .collect();
    pb.finish_with_message("done");

    if let Some(out) = a.json_out {
        JsonReport::new(entries.clone()).write(out)?;
    }
    println!("Hashed {} files", entries.len());
    Ok(())
}
/*{
let idx = Index::open(&a.root)?;
...
let walker = WalkDir::new(&a.root).into_iter().filter_map(Result::ok);
let filtered: Vec<_> = walker
    .filter(|e| e.file_type().is_file())
    .filter(|e| {
        !idx.is_fresh(
            e.path(),
            e.metadata().unwrap().modified().unwrap().elapsed().unwrap().as_secs() as i64,
        )
    })
    ...
for entry in &entries {
    idx.upsert(&entry.path, mtime_i64, &entry.hash);
}
}
*/