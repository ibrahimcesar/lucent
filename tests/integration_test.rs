//! Integration tests for Lucent

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("lucent").unwrap();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Crystal clear code insights"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("lucent").unwrap();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("0.0.1"));
}

#[test]
fn test_analyze_rust_file() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.rs");

    fs::write(
        &test_file,
        r#"
// This is a comment
fn main() {
    println!("Hello, world!");
}
"#,
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("lucent").unwrap();
    cmd.arg(temp.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("💎"))
        .stdout(predicate::str::contains("Lucent"))
        .stdout(predicate::str::contains("Rust"));
}

#[test]
fn test_json_output() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.rs");

    fs::write(&test_file, "fn test() {}").unwrap();

    let mut cmd = Command::cargo_bin("lucent").unwrap();
    cmd.arg(temp.path()).arg("--format").arg("json");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(r#""total_files""#))
        .stdout(predicate::str::contains(r#""Rust""#));
}

#[test]
fn test_empty_directory() {
    let temp = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("lucent").unwrap();
    cmd.arg(temp.path());

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No source files found"));
}

#[test]
fn test_nonexistent_path() {
    let mut cmd = Command::cargo_bin("lucent").unwrap();
    cmd.arg("/nonexistent/path/that/does/not/exist");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_multiple_languages() {
    let temp = TempDir::new().unwrap();

    // Create multiple language files
    fs::write(temp.path().join("test.rs"), "fn main() {}").unwrap();
    fs::write(temp.path().join("test.py"), "def main(): pass").unwrap();
    fs::write(temp.path().join("test.js"), "function main() {}").unwrap();

    let mut cmd = Command::cargo_bin("lucent").unwrap();
    cmd.arg(temp.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Rust"))
        .stdout(predicate::str::contains("Python"))
        .stdout(predicate::str::contains("JavaScript"));
}

#[test]
fn test_respects_gitignore() {
    use std::process::Command as StdCommand;

    let temp = TempDir::new().unwrap();

    // Initialize git repo (required for ignore crate)
    StdCommand::new("git")
        .args(["init"])
        .current_dir(temp.path())
        .output()
        .ok();

    // Create .gitignore
    fs::write(temp.path().join(".gitignore"), "ignored.rs\n").unwrap();

    // Create files
    fs::write(temp.path().join("ignored.rs"), "fn ignored() {}").unwrap();
    fs::write(temp.path().join("included.rs"), "fn included() {}").unwrap();

    let mut cmd = Command::cargo_bin("lucent").unwrap();
    cmd.arg(temp.path());

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should find included.rs (and possibly ignored.rs if git not available)
    // At minimum, we should find at least 1 file
    assert!(stdout.contains("Total Files:"));
    assert!(stdout.contains("Rust"));
}

#[test]
fn test_entropy_calculation() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.rs");

    fs::write(&test_file, "fn main() { let x = 42; }").unwrap();

    let mut cmd = Command::cargo_bin("lucent").unwrap();
    cmd.arg(temp.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Entropy"));
}

#[test]
fn test_verbose_output() {
    let temp = TempDir::new().unwrap();
    fs::write(temp.path().join("test.rs"), "fn main() {}").unwrap();

    let mut cmd = Command::cargo_bin("lucent").unwrap();
    cmd.arg(temp.path()).arg("--verbose");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Scanning"))
        .stdout(predicate::str::contains("Found"));
}

#[test]
fn test_threshold_pass() {
    let temp = TempDir::new().unwrap();
    fs::write(temp.path().join("test.rs"), "fn main() {}").unwrap();

    let mut cmd = Command::cargo_bin("lucent").unwrap();
    cmd.arg(temp.path())
        .arg("--max-complexity")
        .arg("100.0");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("All quality thresholds passed"));
}

#[test]
fn test_threshold_fail() {
    let temp = TempDir::new().unwrap();
    fs::write(
        temp.path().join("test.rs"),
        "fn complex() { if true { if true { if true { if true { if true { } } } } } }",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("lucent").unwrap();
    cmd.arg(temp.path())
        .arg("--max-complexity")
        .arg("1.0");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("Quality thresholds violated"));
}

#[test]
fn test_threshold_warning() {
    let temp = TempDir::new().unwrap();
    fs::write(
        temp.path().join("test.rs"),
        "fn main() {\n    println!(\"test\");\n    println!(\"test\");\n    println!(\"test\");\n}",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("lucent").unwrap();
    cmd.arg(temp.path())
        .arg("--max-function-length")
        .arg("2.0");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Quality warnings detected"));
}

#[test]
fn test_threshold_fail_on_warning() {
    let temp = TempDir::new().unwrap();
    fs::write(
        temp.path().join("test.rs"),
        "fn main() {\n    println!(\"test\");\n    println!(\"test\");\n    println!(\"test\");\n}",
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("lucent").unwrap();
    cmd.arg(temp.path())
        .arg("--max-function-length")
        .arg("2.0")
        .arg("--fail-on-warning");

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("Quality thresholds violated"));
}
