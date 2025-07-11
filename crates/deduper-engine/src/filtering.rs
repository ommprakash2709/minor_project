use chrono::{DateTime, Utc};  // Add this import
use regex::Regex;
use std::{fs, path::Path};

#[derive(Debug, Clone)]
pub struct Filter {
    pub min_size: u64,
    pub max_size: Option<u64>,
    pub ext: Option<String>,
    pub pattern: Regex,
    pub since: Option<DateTime<Utc>>,  // Now properly typed
}

impl Filter {
    pub fn matches(&self, md: &fs::Metadata, path: &Path) -> bool {
        if md.len() < self.min_size {
            return false;
        }
        
        if let Some(max) = self.max_size {
            if md.len() > max {
                return false;
            }
        }
        
        if let Some(ref wanted) = self.ext {
            if path.extension()
                .and_then(|e| e.to_str())
                .map(|e| e != wanted)
                .unwrap_or(true) {
                return false;
            }
        }
        
        if !self.pattern.is_match(&path.to_string_lossy()) {
            return false;
        }
        
        if let Some(since) = self.since {
            if let Ok(mtime) = md.modified() {
                if mtime < since.into() {
                    return false;
                }
            }
        }
        
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_filter_min_size() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Small content").unwrap();
        
        let metadata = temp_file.as_file().metadata().unwrap();
        
        let filter = Filter {
            min_size: 1000,
            max_size: None,
            ext: None,
            pattern: Regex::new(".*").unwrap(),
            since: None,
        };
        
        assert!(!filter.matches(&metadata, temp_file.path()));
    }

    #[test]
    fn test_filter_extension() {
        let temp_file = NamedTempFile::with_suffix(".txt").unwrap();
        let metadata = temp_file.as_file().metadata().unwrap();
        
        let filter = Filter {
            min_size: 0,
            max_size: None,
            ext: Some("txt".to_string()),
            pattern: Regex::new(".*").unwrap(),
            since: None,
        };
        
        assert!(filter.matches(&metadata, temp_file.path()));
    }
}
