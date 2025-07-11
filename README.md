Intelligent File Deduplicator
git
A high-performance, safe file deduplication tool written in Rust that helps you find and manage duplicate files across your filesystem.

🚀 Features
Core Functionality
Multi-Algorithm Hashing: Support for SHA-256, BLAKE3, and xxHash for different performance/security needs

Parallel Processing: Lightning-fast file processing using Rayon for multi-threaded operations

Advanced Filtering: Filter files by size, extension, regex patterns, and modification date

Safe Operations: Quarantine system moves duplicates instead of deleting them permanently

Recovery System: Easily restore quarantined files if needed

Detailed Reports: Generate JSON and HTML reports with file relationships and space savings

Advanced Features
Incremental Scanning: Only process changed files using modification times and checksums

Content Similarity: Compare text files using edit distance algorithms (planned)

Image Similarity: Perceptual hashing for images (planned)

Space Analysis: Detailed breakdown of potential space savings

Metadata Analysis: Consider file attributes and EXIF data for better deduplication

📦 Installation
Prerequisites
Rust 1.70+ (install from rustup.rs)

Git

Build from Source
bash
# Clone the repository
git clone https://github.com/yourusername/intelligent-deduplicator.git
cd intelligent-deduplicator

# Build the project
cargo build --workspace --release

# The binary will be available at target/release/deduper-cli
Install using Cargo
bash
cargo install --path crates/deduper-cli
🛠️ Usage
Command Overview
The deduplicator provides four main commands:

Command	Description
find	List all files in a directory
scan	Hash files and generate duplicate reports
quarantine	Move duplicate files to quarantine
recover	Restore files from quarantine
1. Find Files
List all files in a directory recursively:

bash
# Find all files in current directory
cargo run --bin deduper-cli -- find .

# Find files in specific directory
cargo run --bin deduper-cli -- find /path/to/directory
2. Scan for Duplicates
Analyze files and generate reports:

bash
# Basic scan of text files
cargo run --bin deduper-cli -- scan . --ext txt

# Advanced scan with custom options
cargo run --bin deduper-cli -- scan . \
  --min-size 1024 \
  --ext pdf \
  --algo blake3 \
  --output report.json

# Scan all files with pattern matching
cargo run --bin deduper-cli -- scan . \
  --pattern ".*\.(jpg|jpeg|png)$" \
  --output image_duplicates.json
Scan Options
--min-size <BYTES>: Minimum file size (default: 0)

--ext <EXTENSION>: File extension filter (default: "txt")

--pattern <REGEX>: Regex pattern for filenames (default: ".*")

--algo <ALGORITHM>: Hash algorithm - sha256, blake3, xxh3 (default: sha256)

--output <FILE>: Output JSON report file

3. Quarantine Duplicates
Safely move duplicate files to quarantine:

bash
# Quarantine duplicate text files
cargo run --bin deduper-cli -- quarantine . --ext txt

# Quarantine with size filter
cargo run --bin deduper-cli -- quarantine . \
  --min-size 100000 \
  --ext jpg

# Quarantine all file types
cargo run --bin deduper-cli -- quarantine . --pattern ".*"
Quarantine Location
Windows: C:\Users\<username>\.deduper\quarantine\

macOS/Linux: ~/.deduper/quarantine/

4. Recover Files
Restore quarantined files:

bash
# Recover a specific file
cargo run --bin deduper-cli -- recover filename.txt

# The file will be restored to the current directory
📊 Example Workflow
bash
# 1. First, scan to identify duplicates
cargo run --bin deduper-cli -- scan ~/Documents \
  --ext pdf \
  --min-size 1024 \
  --output pdf_analysis.json

# 2. Review the report (optional)
cat pdf_analysis.json

# 3. Quarantine duplicates
cargo run --bin deduper-cli -- quarantine ~/Documents \
  --ext pdf \
  --min-size 1024

# 4. Check results
ls -la ~/.deduper/quarantine/

# 5. Recover if needed
cargo run --bin deduper-cli -- recover important_document.pdf
🏗️ Project Structure
text
intelligent-deduplicator/
├── Cargo.toml                    # Workspace configuration
├── README.md                     # This file
├── test-data/                    # Test fixtures and samples
│   ├── sample-files/
│   └── expected-outputs/
└── crates/
    ├── deduper-cli/              # CLI application
    │   ├── Cargo.toml
    │   ├── src/
    │   │   └── main.rs
    │   └── tests/                # Integration tests
    │       ├── cli_tests.rs
    │       └── common/
    └── deduper-engine/           # Core library
        ├── Cargo.toml
        ├── src/
        │   ├── lib.rs
        │   ├── hashing.rs        # File hashing algorithms
        │   ├── filtering.rs      # File filtering logic
        │   └── quarantine.rs     # Quarantine operations
        └── tests/                # Unit and integration tests
🧪 Development
Running Tests
bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p deduper-engine
cargo test -p deduper-cli

# Run with output
cargo test --workspace -- --nocapture

# Run integration tests only
cargo test --workspace --test '*'
Code Coverage
bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --workspace --out Html
Performance Benchmarks
bash
# Run benchmarks (if implemented)
cargo bench --workspace
🎯 Hash Algorithms
Algorithm	Speed	Security	Use Case
xxHash3	Fastest	Good	Quick deduplication, development
BLAKE3	Fast	Excellent	General purpose, recommended
SHA-256	Moderate	Excellent	Maximum compatibility, security-critical
📈 Performance Tips
Use xxHash3 for maximum speed when cryptographic security isn't required

Set appropriate min-size to skip tiny files that rarely need deduplication

Use specific extensions instead of pattern matching when possible

Enable parallel processing (enabled by default with Rayon)

🔧 Configuration
Environment Variables
RUST_LOG: Set logging level (debug, info, warn, error)

RAYON_NUM_THREADS: Control parallel processing threads

Example Configuration
bash
export RUST_LOG=info
export RAYON_NUM_THREADS=8
cargo run --bin deduper-cli -- scan . --ext jpg
🚨 Safety Features
Non-destructive operations: Files are moved to quarantine, never deleted

Atomic operations: File moves are atomic to prevent corruption

Backup quarantine log: All operations are logged for audit trail

Recovery system: Easy restoration of quarantined files

Size verification: Files are verified before and after operations

🐛 Troubleshooting
Common Issues
"No files found" when scanning
Check that files match your extension filter (.txt, .pdf, etc.)

Verify --min-size isn't too large for your files

Use --pattern ".*" to scan all files

Permission denied errors
Ensure you have read access to the directory

Run with appropriate permissions on protected directories

Check that quarantine directory is writable

Large memory usage
Reduce the number of concurrent threads: RAYON_NUM_THREADS=4

Process directories in smaller batches

Use xxHash3 for lower memory overhead

Debug Mode
bash
RUST_LOG=debug cargo run --bin deduper-cli -- scan . --ext txt
🤝 Contributing
Fork the repository

Create a feature branch: git checkout -b feature-name

Make your changes and add tests

Ensure tests pass: cargo test --workspace

Format code: cargo fmt

Run clippy: cargo clippy

Submit a pull request

Development Guidelines
Follow Rust naming conventions

Add unit tests for new functionality

Update integration tests for CLI changes

Document public APIs with rustdoc comments

Ensure backward compatibility

📝 License
This project is licensed under the MIT License - see the LICENSE file for details.

🙏 Acknowledgments
Rayon: For excellent parallel processing capabilities

Clap: For robust command-line argument parsing

BLAKE3: For fast, secure hashing

Serde: For JSON serialization support

WalkDir: For efficient directory traversal

📚 Further Reading
Rust Book - Learn Rust programming

Clap Documentation - Command-line parsing

Rayon Guide - Parallel computing in Rust

BLAKE3 Paper - Technical details

🏷️ Version History
v0.1.0: Initial release with basic deduplication

Core features: find, scan, quarantine, recover

Support for SHA-256, BLAKE3, xxHash3

Parallel processing and JSON reports

Built with ❤️ and ⚡ in Rust
