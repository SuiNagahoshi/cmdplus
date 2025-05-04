//! # `touch` command implementation
//!
//! ## English
//! This module implements the `touch` command.
//! It creates an empty file and creates intermediate directories if needed.
//!
//! ## 日本語
//! このモジュールは `touch` コマンドを実装しています。  
//! 空のファイルを作成し、必要に応じて中間ディレクトリも作成します。

use std::fs::{self, OpenOptions};
use std::io;
use std::path::Path;

/// Create a file and any missing parent directories.
///
/// ## English
/// This function emulates the behavior of Unix `touch`.  
/// If directories do not exist, it creates them first.
///
/// ## 日本語
/// Unix の `touch` コマンドと同等の動作を行います。  
/// ディレクトリが存在しない場合は先に作成します。
pub fn touch_command(path_str: &str) -> io::Result<()> {
    let path = Path::new(path_str);

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    if path.exists() {
        // 更新日時だけ変更（metadata取得で存在確認済み）
        let now = filetime::FileTime::now();
        filetime::set_file_mtime(path, now)?;
    } else {
        // Open or create the file, then immediately drop the handle
        OpenOptions::new().create(true).append(true).open(path)?;
    }

    Ok(())
}
