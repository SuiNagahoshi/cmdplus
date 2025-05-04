//! # `which` command implementation
//!
//! ## English
//! This module implements the `which` command.
//! It searches for a command in the user's `PATH` and returns the full path if found.
//!
//! ## 日本語
//! このモジュールは `which` コマンドを実装しています。  
//! ユーザーの `PATH` 環境変数を検索し、該当するコマンドのフルパスを返します。

use std::env;
use std::fs;
use std::path::PathBuf;

/// Find the full path of the given command.
///
/// ## English
/// Searches all directories in the `PATH` environment variable and
/// returns the first matching executable.
///
/// ## 日本語
/// `PATH` 環境変数に含まれる全ディレクトリを検索し、  
/// 最初に見つかった実行可能ファイルのパスを返します。
pub fn which_command(program: &str) -> Option<PathBuf> {
    let path_env = env::var_os("PATH")?;
    let paths = env::split_paths(&path_env);

    let is_windows = cfg!(windows);
    let exts: Vec<String> = if is_windows {
        env::var("PATHEXT")
            .unwrap_or(".EXE;.CMD;.BAT;.COM".to_string())
            .split(';')
            .map(|e| e.to_ascii_lowercase())
            .collect()
    } else {
        vec!["".to_string()]
    };

    for dir in paths {
        for ext in &exts {
            let mut full_path = dir.join(program);
            if !ext.is_empty() {
                full_path.set_extension(&ext[1..]); // remove leading `.`
            }
            if full_path.exists()
                && fs::metadata(&full_path)
                    .map(|m| m.is_file())
                    .unwrap_or(false)
            {
                return Some(full_path);
            }
        }
    }

    None
}
