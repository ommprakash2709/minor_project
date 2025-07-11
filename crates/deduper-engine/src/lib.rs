//! Intelligent File Deduplicator Engine

pub mod hashing;
pub mod filtering;
pub mod quarantine;

use serde::{Deserialize, Serialize};
// Remove this unused import: use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileEntry {
    pub path: String,
    pub hash: String,
}

/// Recursively scan directory and hash matching files
pub fn scan_directory(
    root: &std::path::Path,
    filter: &filtering::Filter,
    algo: hashing::Algo,
) -> anyhow::Result<Vec<FileEntry>> {
    use rayon::prelude::*;
    use walkdir::WalkDir;

    let files: Vec<_> = WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .collect();

    let entries: Vec<_> = files
        .par_iter()
        .filter(|e| filter.matches(&e.metadata().unwrap(), e.path()))
        .filter_map(|e| {
            hashing::hash_file(e.path(), algo)
                .ok()
                .map(|digest| FileEntry {
                    path: e.path().to_string_lossy().into_owned(),
                    hash: digest,
                })
        })
        .collect();

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_scan_directory_empty() {
        let temp_dir = TempDir::new().unwrap();
        let filter = filtering::Filter {
            min_size: 0,
            max_size: None,
            ext: None,
            pattern: regex::Regex::new(".*").unwrap(),
            since: None,
        };
        
        let result = scan_directory(temp_dir.path(), &filter, hashing::Algo::Sha256).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_scan_directory_with_files() {
        let temp_dir = TempDir::new().unwrap();
        
        let file1_path = temp_dir.path().join("test1.txt");
        let mut file1 = File::create(&file1_path).unwrap();
        writeln!(file1, "Hello World").unwrap();
        
        let file2_path = temp_dir.path().join("test2.txt");
        let mut file2 = File::create(&file2_path).unwrap();
        writeln!(file2, "Hello World").unwrap();
        
        let filter = filtering::Filter {
            min_size: 0,
            max_size: None,
            ext: Some("txt".to_string()),
            pattern: regex::Regex::new(".*").unwrap(),
            since: None,
        };
        
        let result = scan_directory(temp_dir.path(), &filter, hashing::Algo::Sha256).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].hash, result[1].hash);
    }
}
