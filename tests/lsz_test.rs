use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn lsz_lists_files_in_size_order() {
    let dir = tempdir().unwrap();
    let small = dir.path().join("small.txt");
    let mut f1 = File::create(&small).unwrap();
    write!(f1, "a").unwrap(); // 1 byte

    let large = dir.path().join("large.txt");
    let mut f2 = File::create(&large).unwrap();
    write!(f2, "bbbb").unwrap(); // 4 bytes

    Command::cargo_bin("cmdplus")
        .unwrap()
        .arg("lsz")
        .arg(dir.path())
        .assert()
        .success()
        // first line should contain "1" and "small.txt" then later "4" and "large.txt"
        .stdout(predicate::str::contains("small.txt").and(predicate::str::contains("large.txt")));
}

#[test]
fn lsz_error_on_invalid_path() {
    Command::cargo_bin("cmdplus")
        .unwrap()
        .arg("lsz")
        .arg("nonexistent_dir_xyz")
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot read directory"));
}
