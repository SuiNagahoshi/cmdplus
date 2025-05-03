use cmdplus::touch_file;
use std::fs;
use tempfile::tempdir;

#[test]
fn creates_new_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("newfile.txt");

    assert!(!file_path.exists());
    touch_file(&file_path).unwrap();
    assert!(file_path.exists());
}

#[test]
fn updates_timestamp_on_existing_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("existing.txt");

    fs::write(&file_path, b"hello").unwrap();
    let old_time = fs::metadata(&file_path).unwrap().modified().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));
    touch_file(&file_path).unwrap();
    let new_time = fs::metadata(&file_path).unwrap().modified().unwrap();

    assert!(new_time > old_time);
}

#[test]
fn creates_file_in_missing_nested_dirs() {
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("nested/path/file.txt");

    assert!(!file_path.exists());
    touch_file(&file_path).unwrap();
    assert!(file_path.exists());
}
