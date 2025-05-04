#[cfg(test)]
mod tests {
    use cmdplus::commands::rm::rm_command;
    use lazy_static::lazy_static;
    use std::fs::{File, create_dir_all, remove_dir_all};
    use std::io::{self, Write};
    use std::path::Path;
    use std::sync::Mutex;

    // テスト間の同期を確保
    lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

    fn setup_file_and_dir() -> io::Result<()> {
        cleanup_file_and_dir()?; // 初期クリーンアップを追加

        let test_dir = Path::new("test_dir");
        let test_file = test_dir.join("test_file.txt");

        create_dir_all(test_dir)?;
        let mut file = File::create(test_file)?;
        writeln!(file, "This is a test file")?;

        Ok(())
    }

    fn cleanup_file_and_dir() -> io::Result<()> {
        let test_dir = Path::new("test_dir");
        if test_dir.exists() {
            remove_dir_all(test_dir)?;
        }
        Ok(())
    }

    #[test]
    fn test_rm_file() {
        let _lock = TEST_MUTEX.lock().unwrap();
        setup_file_and_dir().expect("Failed to setup test environment");

        let file_path = "test_dir/test_file.txt";
        rm_command(file_path, false, false).expect("Failed to remove file");

        assert!(!Path::new(file_path).exists());

        cleanup_file_and_dir().expect("Failed to cleanup");
    }

    // 他のテストケースも同様に修正...

    #[test]
    fn test_rm_empty_dir() {
        let _lock = TEST_MUTEX.lock().unwrap();
        let test_dir = Path::new("test_dir");

        cleanup_file_and_dir().expect("Failed to cleanup before test");
        create_dir_all(test_dir).expect("Failed to create test directory");

        rm_command("test_dir", false, true).expect("Failed to remove directory");

        assert!(!test_dir.exists());
    }

    #[test]
    fn test_rm_non_existing_file() {
        let _lock = TEST_MUTEX.lock().unwrap();
        cleanup_file_and_dir().expect("Failed to cleanup before test");

        let result = rm_command("non_existing_file.txt", false, false);
        assert!(result.is_err());
    }
}
