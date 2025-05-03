use std::fs::{self, OpenOptions};
use std::io;
use std::path::Path;

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
        // なければ作成
        OpenOptions::new().create(true).write(true).open(path)?;
    }

    Ok(())
}
