//! # `lsz` command implementation
//!
//! ## English
//! This module implements the `lsz` command.
//! It lists the total size of each directory and file under the given path.
//!
//! ## 日本語
//! このモジュールは `lsz` コマンドを実装しています。  
//! 指定したパス以下のファイルおよびディレクトリの合計サイズを一覧表示します。

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// List directory and file sizes recursively.
///
/// ## English
/// Walks through the target directory and prints size of files and directories.
///
/// ## 日本語
/// 指定ディレクトリ以下を再帰的に走査し、  
/// 各ファイル・ディレクトリのサイズを表示します。
pub fn lsz_command(dir: &str) -> io::Result<()> {
    let path = Path::new(dir);
    let mut entries = Vec::new();
    collect_sizes(path, &mut entries)?;

    entries.sort_by(|a, b| b.1.cmp(&a.1)); // Sort descending by size

    for (path, size) in entries {
        println!("{:>10}  {}", format_size(size), path.display());
    }

    Ok(())
}

/// Collect sizes of files and directories recursively.
///
/// ## English
/// Populates a vector with the total size for each item.
///
/// ## 日本語
/// ファイルとディレクトリの合計サイズを計算してベクタに格納します。
fn collect_sizes(path: &Path, entries: &mut Vec<(PathBuf, u64)>) -> io::Result<u64> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.is_file() {
        let size = metadata.len();
        entries.push((path.to_path_buf(), size));
        Ok(size)
    } else if metadata.is_dir() {
        let mut total = 0;
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            total += collect_sizes(&entry.path(), entries)?;
        }
        entries.push((path.to_path_buf(), total));
        Ok(total)
    } else {
        Ok(0)
    }
}

/// Format size in bytes to a readable string.
///
/// ## English
/// Converts bytes to a human-readable format like KB, MB, etc.
///
/// ## 日本語
/// バイト数を KB や MB などの読みやすい形式に変換します。
fn format_size(size: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    format!("{:.1}{}", size, UNITS[unit])
}
