use anyhow::Result;
use sha2::{Digest as ShaDigest, Sha256};
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Algo {
    Sha256,
    Blake3,
    Xxh3,
}

pub fn hash_file(path: &Path, algo: Algo) -> Result<String> {
    let mut reader = BufReader::new(File::open(path)?);
    match algo {
        Algo::Sha256 => {
            let mut hasher = Sha256::new();
            pipe(&mut reader, |buf| { hasher.update(buf); })?;
            Ok(format!("{:x}", hasher.finalize()))
        }
        Algo::Blake3 => {
            let mut hasher = blake3::Hasher::new();
            pipe(&mut reader, |buf| { hasher.update(buf); })?;
            Ok(hasher.finalize().to_hex().to_string())
        }
        Algo::Xxh3 => {
            let mut hasher = xxhash_rust::xxh3::Xxh3::new();
            pipe(&mut reader, |buf| { hasher.update(buf); })?;
            Ok(format!("{:016x}", hasher.digest()))
        }
    }
}

fn pipe<R: Read, F: FnMut(&[u8])>(r: &mut R, mut feed: F) -> Result<()> {
    let mut buf = [0u8; 8192];
    loop {
        let n = r.read(&mut buf)?;
        if n == 0 {
            break;
        }
        feed(&buf[..n]);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_hash_file_sha256() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Hello World").unwrap();
        
        let hash = hash_file(temp_file.path(), Algo::Sha256).unwrap();
        // Correct SHA256 hash for "Hello World\n" (with newline)
        assert_eq!(hash, "d2a84f4b8b650937ec8f73cd8be2c74add5a911ba64df27458ed8229da804a26");
    }

    // ... rest of your tests remain the same



    #[test]
    fn test_hash_file_blake3() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Hello World").unwrap();
        
        let hash = hash_file(temp_file.path(), Algo::Blake3).unwrap();
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64); // BLAKE3 produces 64-char hex string
    }

    #[test]
    fn test_hash_file_xxh3() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Hello World").unwrap();
        
        let hash = hash_file(temp_file.path(), Algo::Xxh3).unwrap();
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 16); // XXH3 produces 16-char hex string
    }

    #[test]
    fn test_same_content_same_hash() {
        let content = "Identical content for testing";
        
        let mut temp_file1 = NamedTempFile::new().unwrap();
        writeln!(temp_file1, "{}", content).unwrap();
        
        let mut temp_file2 = NamedTempFile::new().unwrap();
        writeln!(temp_file2, "{}", content).unwrap();
        
        let hash1 = hash_file(temp_file1.path(), Algo::Sha256).unwrap();
        let hash2 = hash_file(temp_file2.path(), Algo::Sha256).unwrap();
        
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_different_content_different_hash() {
        let mut temp_file1 = NamedTempFile::new().unwrap();
        writeln!(temp_file1, "Content 1").unwrap();
        
        let mut temp_file2 = NamedTempFile::new().unwrap();
        writeln!(temp_file2, "Content 2").unwrap();
        
        let hash1 = hash_file(temp_file1.path(), Algo::Sha256).unwrap();
        let hash2 = hash_file(temp_file2.path(), Algo::Sha256).unwrap();
        
        assert_ne!(hash1, hash2);
    }

}