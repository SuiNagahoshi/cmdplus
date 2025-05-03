use std::env;
use std::fs;
use std::path::{Path, PathBuf};

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
            if full_path.exists() && fs::metadata(&full_path).map(|m| m.is_file()).unwrap_or(false) {
                return Some(full_path);
            }
        }
    }

    None
}
