use deduper_engine::*;
use tempfile::TempDir;
use std::fs::File;
use std::io::Write;

#[test]
fn test_complete_deduplication_workflow() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create test files with duplicate content
    let file1_path = temp_dir.path().join("file1.txt");
    let mut file1 = File::create(&file1_path).unwrap();
    writeln!(file1, "Duplicate content").unwrap();
    
    let file2_path = temp_dir.path().join("file2.txt");
    let mut file2 = File::create(&file2_path).unwrap();
    writeln!(file2, "Duplicate content").unwrap();
    
    let file3_path = temp_dir.path().join("file3.txt");
    let mut file3 = File::create(&file3_path).unwrap();
    writeln!(file3, "Different content").unwrap();
    
    // Create filter
    let filter = filtering::Filter {
        min_size: 0,
        max_size: None,
        ext: Some("txt".to_string()),
        pattern: regex::Regex::new(".*").unwrap(),
        since: None,
    };
    
    // Scan directory
    let entries = scan_directory(temp_dir.path(), &filter, hashing::Algo::Sha256).unwrap();
    
    // Should find 3 files
    assert_eq!(entries.len(), 3);
    
    // Find duplicates
    let mut hash_map = std::collections::HashMap::new();
    for entry in &entries {
        let list = hash_map.entry(&entry.hash).or_insert_with(Vec::new);
        list.push(entry);
    }
    
    // Should have 2 unique hashes (2 duplicates + 1 unique)
    assert_eq!(hash_map.len(), 2);
    
    // One hash should have 2 files (duplicates)
    let duplicate_group = hash_map.values().find(|v| v.len() == 2).unwrap();
    assert_eq!(duplicate_group.len(), 2);
}

#[test]
fn test_different_hash_algorithms() {
    let temp_dir = TempDir::new().unwrap();
    
    let file_path = temp_dir.path().join("test.txt");
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "Test content for hashing").unwrap();
    drop(file);
    
    let filter = filtering::Filter {
        min_size: 0,
        max_size: None,
        ext: None,
        pattern: regex::Regex::new(".*").unwrap(),
        since: None,
    };
    
    // Test all hash algorithms
    let sha256_entries = scan_directory(temp_dir.path(), &filter, hashing::Algo::Sha256).unwrap();
    let blake3_entries = scan_directory(temp_dir.path(), &filter, hashing::Algo::Blake3).unwrap();
    let xxh3_entries = scan_directory(temp_dir.path(), &filter, hashing::Algo::Xxh3).unwrap();
    
    // All should find the same file
    assert_eq!(sha256_entries.len(), 1);
    assert_eq!(blake3_entries.len(), 1);
    assert_eq!(xxh3_entries.len(), 1);
    
    // But hashes should be different
    assert_ne!(sha256_entries[0].hash, blake3_entries[0].hash);
    assert_ne!(blake3_entries[0].hash, xxh3_entries[0].hash);
    assert_ne!(sha256_entries[0].hash, xxh3_entries[0].hash);
}
