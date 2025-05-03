use std::fs::{self, OpenOptions};
use std::io;
use std::path::Path;

/// ファイルが存在すればタイムスタンプ更新、なければ作成
pub fn touch_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?; // ここが追加されたロジック
    }

    if path.exists() {
        let now = filetime::FileTime::now();
        filetime::set_file_times(path, now, now)?;
    } else {
        OpenOptions::new().create(true).write(true).open(path)?;
    }

    Ok(())
}
