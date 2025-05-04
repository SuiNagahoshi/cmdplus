#[cfg(test)]
mod tests {
    use std::fs::File;
    use tempfile::tempdir;

    use cmdplus::commands::rm::rm_command;

    #[test]
    fn test_rm_existing_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("testfile.txt");
        File::create(&file_path).unwrap();
        assert!(file_path.exists());

        rm_command(file_path.to_str().unwrap()).unwrap();
        assert!(!file_path.exists());
    }

    #[test]
    fn test_rm_nonexistent_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("does_not_exist.txt");

        let result = rm_command(file_path.to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_rm_directory_should_fail() {
        let dir = tempdir().unwrap();
        let result = rm_command(dir.path().to_str().unwrap());
        assert!(result.is_err());
    }
}
