use std::process::Command;
use tempfile::TempDir;
use std::fs;
use std::io::Write;

#[test]
fn test_find_command() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create test files properly
    let file1_path = temp_dir.path().join("file1.txt");
    fs::write(&file1_path, "content1").unwrap();
    
    let file2_path = temp_dir.path().join("file2.txt");
    fs::write(&file2_path, "content2").unwrap();
    
    let mut cmd = Command::new("cargo");
    cmd.args(&["run", "--bin", "deduper-cli", "--", "find"])
        .arg(temp_dir.path())
        .current_dir(env!("CARGO_MANIFEST_DIR"));
    
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    assert!(stdout.contains("file1.txt"));
    assert!(stdout.contains("file2.txt"));
}

#[test]
fn test_scan_command() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create test files properly
    let file1_path = temp_dir.path().join("file1.txt");
    fs::write(&file1_path, "content1").unwrap();
    
    let file2_path = temp_dir.path().join("file2.txt");
    fs::write(&file2_path, "content2").unwrap();
    
    let output_file = temp_dir.path().join("report.json");
    
    let mut cmd = Command::new("cargo");
    cmd.args(&["run", "--bin", "deduper-cli", "--", "scan"])
        .arg(temp_dir.path())
        .arg("--ext").arg("txt")
        .arg("--output").arg(&output_file)
        .current_dir(env!("CARGO_MANIFEST_DIR"));
    
    let output = cmd.output().unwrap();
    assert!(output.status.success());
    
    // Check that report file was created
    assert!(output_file.exists());
    
    // Check report content
    let report_content = fs::read_to_string(&output_file).unwrap();
    let entries: Vec<deduper_engine::FileEntry> = serde_json::from_str(&report_content).unwrap();
    assert_eq!(entries.len(), 2);
}

#[test]
fn test_quarantine_command() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create duplicate files
    let file1_path = temp_dir.path().join("file1.txt");
    fs::write(&file1_path, "duplicate content").unwrap();
    
    let file2_path = temp_dir.path().join("file2.txt");
    fs::write(&file2_path, "duplicate content").unwrap();
    
    let mut cmd = Command::new("cargo");
    cmd.args(&["run", "--bin", "deduper-cli", "--", "quarantine"])
        .arg(temp_dir.path())
        .arg("--ext").arg("txt")
        .current_dir(env!("CARGO_MANIFEST_DIR"));
    
    let output = cmd.output().unwrap();
    assert!(output.status.success());
    
    // One file should remain, one should be quarantined
    let remaining_files: Vec<_> = fs::read_dir(temp_dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "txt"))
        .collect();
    
    assert_eq!(remaining_files.len(), 1);
}
