use std::process::Command;
use tempfile::TempDir;
use std::fs;

pub fn run_cli_command(args: &[&str], temp_dir: &TempDir) -> std::process::Output {
    let mut cmd = Command::new("cargo");
    cmd.args(&["run", "--bin", "deduper-cli", "--"])
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"));
    
    cmd.output().unwrap()
}

pub fn create_test_structure(dir: &TempDir) {
    // Create subdirectories
    fs::create_dir(dir.path().join("subdir1")).unwrap();
    fs::create_dir(dir.path().join("subdir2")).unwrap();
    
    // Create files in root
    fs::write(dir.path().join("root1.txt"), "Root file content").unwrap();
    
    // Create files in subdirectories
    fs::write(dir.path().join("subdir1/sub1.txt"), "Subdirectory file content").unwrap();
    fs::write(dir.path().join("subdir2/sub2.txt"), "Subdirectory file content").unwrap(); // Duplicate
}
