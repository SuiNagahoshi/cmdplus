use cmdplus::commands::touch::touch_command;
use std::fs;
use std::path::Path;

#[test]
fn test_touch_creates_file_and_directories() {
    let test_path = "test_temp_dir/subdir/testfile.txt";
    let _ = fs::remove_file(&test_path);
    let _ = fs::remove_dir_all("test_temp_dir");

    assert!(!Path::new(test_path).exists());

    let result = touch_command(test_path);
    assert!(result.is_ok());
    assert!(Path::new(test_path).exists());

    // クリーンアップ
    let _ = fs::remove_file(test_path);
    let _ = fs::remove_dir_all("test_temp_dir");
}
