//! # `rm` command implementation
//!
//! ## English
//! This module implements the `rm` command.  
//! It deletes a single file. Directories and recursive deletion are not supported yet.
//!
//! ## 日本語
//! このモジュールは `rm` コマンドを実装しています。  
//! 単一ファイルの削除のみ対応しており、ディレクトリや再帰削除は未対応です。

use std::fs;
use std::io;
use std::path::Path;

/// Delete a single file.
///
/// ## English
/// Deletes a file at the given path. If the file does not exist or is a directory, it returns an error.
///
/// ## 日本語
/// 指定されたパスのファイルを削除します。  
/// ファイルが存在しない場合や、ディレクトリだった場合はエラーになります。
pub fn rm_command(path: &str) -> io::Result<()> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File does not exist",
        ));
    }
    if path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::Other, "Is a directory"));
    }
    fs::remove_file(path)
}
