//! # `lsz` command implementation
//!
//! ## English
//! This module implements the `lsz` command.
//! It displays the size of all files and directories within a given directory.
//!
//! ## 日本語
//! このモジュールは `lsz` コマンドを実装しています。  
//! 指定されたディレクトリ内のすべてのファイルとディレクトリのサイズを表示します。

use std::fs;
use std::path::Path;

/// ## English
/// Calculate and return the size of all files and subdirectories within a given directory.
/// Returns a formatted string including each entry and the total size.
///
/// ## 日本語
/// 指定されたディレクトリ内のファイルとサブディレクトリのサイズを合計し、  
/// 一覧表示形式の文字列で返します。
pub fn lsz_command(path: &str) -> Result<String, std::io::Error> {
    let path = Path::new(path);
    let mut output = String::new();
    let mut total_size = 0u64;

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let file_size = if metadata.is_dir() {
                calculate_dir_size(&entry.path())?
            } else {
                metadata.len()
            };

            total_size += file_size;

            output.push_str(&format!(
                "{}\t{} bytes\n",
                entry.file_name().to_string_lossy(),
                file_size
            ));
        }

        output.push_str(&format!("Total size: {} bytes\n", total_size));
        Ok(output)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Not a directory",
        ))
    }
}

/// ## English
/// Recursively calculates the total size of a directory and its contents.
///
/// ## 日本語
/// ディレクトリ内のすべてのファイルおよびサブディレクトリのサイズを  
/// 再帰的に合計して返します。
fn calculate_dir_size(path: &Path) -> Result<u64, std::io::Error> {
    let mut size = 0;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            size += calculate_dir_size(&entry.path())?;
        } else {
            size += metadata.len();
        }
    }

    Ok(size)
}
