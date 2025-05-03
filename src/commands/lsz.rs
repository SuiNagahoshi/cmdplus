use std::fs;
use std::path::{Path, PathBuf};

/// 指定されたディレクトリ内のファイルやサブディレクトリを
/// サイズ順に表示するコマンド本体
pub fn execute_lsz(path: &str) {
    // 指定パスのエントリ一覧を取得
    let mut entries: Vec<(u64, PathBuf)> = match fs::read_dir(path) {
        Ok(rd) => rd
            .filter_map(Result::ok)
            .map(|e| {
                let p = e.path();
                let size = compute_size(&p);
                (size, p)
            })
            .collect(),
        Err(err) => {
            eprintln!("cannot read directory {}: {}", path, err);
            std::process::exit(1);
        }
    };

    // サイズ順（昇順）
    entries.sort_by_key(|(size, _)| *size);

    // 出力
    for (size, path) in entries {
        println!("{:>10}  {}", size, path.display());
    }
}

/// ファイルなら metadata.len() を返し、
/// ディレクトリなら再帰的に中の全ファイルサイズを合計して返す
fn compute_size(p: &Path) -> u64 {
    if let Ok(md) = fs::metadata(p) {
        if md.is_file() {
            return md.len();
        }
        if md.is_dir() {
            let mut total = 0;
            if let Ok(rd) = fs::read_dir(p) {
                for entry in rd.filter_map(Result::ok) {
                    total += compute_size(&entry.path());
                }
            }
            return total;
        }
    }
    0
}
