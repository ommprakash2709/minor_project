use tempfile::TempDir;
use std::fs::File;
use std::io::Write;

pub fn create_test_files(dir: &TempDir) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    
    // Create a text file
    let file1_path = dir.path().join("test1.txt");
    let mut file1 = File::create(&file1_path).unwrap();
    writeln!(file1, "Test content 1").unwrap();
    files.push(file1_path);
    
    // Create a duplicate
    let file2_path = dir.path().join("test2.txt");
    let mut file2 = File::create(&file2_path).unwrap();
    writeln!(file2, "Test content 1").unwrap(); // Same content
    files.push(file2_path);
    
    // Create a different file
    let file3_path = dir.path().join("test3.txt");
    let mut file3 = File::create(&file3_path).unwrap();
    writeln!(file3, "Different content").unwrap();
    files.push(file3_path);
    
    files
}

pub fn create_large_test_file(dir: &TempDir, size_mb: usize) -> std::path::PathBuf {
    let file_path = dir.path().join("large_file.bin");
    let mut file = File::create(&file_path).unwrap();
    
    let chunk = vec![0u8; 1024 * 1024]; // 1MB chunk
    for _ in 0..size_mb {
        file.write_all(&chunk).unwrap();
    }
    
    file_path
}
