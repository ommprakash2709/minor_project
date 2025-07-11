use anyhow::Result;
use dirs::home_dir;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn quarantine(src: &Path) -> Result<PathBuf> {
    let qdir = home_dir()
        .ok_or_else(|| anyhow::anyhow!("cannot resolve $HOME"))?
        .join(".deduper/quarantine");
    fs::create_dir_all(&qdir)?;
    let dest = qdir.join(src.file_name().unwrap());
    fs::rename(src, &dest)?;
    Ok(dest)
}

pub fn recover(file_name: &str) -> Result<PathBuf> {
    let qdir = home_dir()
        .ok_or_else(|| anyhow::anyhow!("cannot resolve $HOME"))?
        .join(".deduper/quarantine");
    let src = qdir.join(file_name);
    if !src.exists() {
        anyhow::bail!("{} not found in quarantine", file_name);
    }
    let dest = PathBuf::from(file_name);
    fs::rename(&src, &dest)?;
    Ok(dest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;  // Remove NamedTempFile import
    use std::io::Write;

    #[test]
    fn test_quarantine_and_recover() {
        let temp_dir = TempDir::new().unwrap();
        
        let file_path = temp_dir.path().join("test_file.txt");
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "Test content").unwrap();
        drop(file);
        
        let quarantined_path = quarantine(&file_path).unwrap();
        assert!(!file_path.exists());
        assert!(quarantined_path.exists());
        
        let recovered_path = recover("test_file.txt").unwrap();
        assert!(recovered_path.exists());
        assert!(!quarantined_path.exists());
    }

    #[test]
    fn test_recover_nonexistent_file() {
        let result = recover("nonexistent_file.txt");
        assert!(result.is_err());
    }
}
