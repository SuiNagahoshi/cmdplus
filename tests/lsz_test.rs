// tests/lsz_test.rs

use std::fs::File;
use std::io::Write;
use tempfile::{NamedTempFile, tempdir};

use cmdplus::commands::lsz::lsz_command;

#[test]
fn test_lsz_directory_with_files() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    // small.txt (3 bytes)
    let small = dir.path().join("small.txt");
    let mut f1 = File::create(&small)?;
    write!(f1, "abc")?;
    // large.txt (10 bytes)
    let large = dir.path().join("large.txt");
    let mut f2 = File::create(&large)?;
    write!(f2, "abcdefghij")?;

    // Run lsz_command
    let output = lsz_command(dir.path().to_str().unwrap())?;
    // Should list both files
    assert!(output.contains("small.txt"));
    assert!(output.contains("large.txt"));
    // Should include total size line
    assert!(output.contains("Total size:"));

    Ok(())
}

#[test]
fn test_lsz_empty_directory() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let output = lsz_command(dir.path().to_str().unwrap())?;
    // No entries, but total size should be 0 bytes
    assert!(output.contains("Total size: 0 bytes"));
    Ok(())
}

#[test]
fn test_lsz_not_a_directory() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path().to_str().unwrap();
    let err = lsz_command(path).unwrap_err();
    // Expect an InvalidInput error for non-directory
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
}
