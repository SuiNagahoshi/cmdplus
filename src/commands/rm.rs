use std::fs::{self, remove_dir, remove_file};
use std::io::{self, ErrorKind};
use std::path::Path;

/// Remove a file or directory.
///
/// ## English
/// This function removes a file or directory. If the `recursive` flag is set, it will remove a directory and all of its contents.
/// If the `dir` flag is set, it will only remove empty directories.
///
/// ## 日本語
/// この関数はファイルまたはディレクトリを削除します。  
/// `recursive` フラグが設定されている場合、ディレクトリとその内容を再帰的に削除します。  
/// `dir` フラグが設定されている場合、空のディレクトリのみを削除します。
pub fn rm_command(path: &str, recursive: bool, dir: bool) -> io::Result<()> {
    let path = Path::new(path);

    // Check if it's a directory and handle the flags accordingly
    if path.is_dir() {
        if recursive {
            // If recursive, remove directory and all its contents
            remove_dir_all(path)
        } else if dir {
            // If dir flag, ensure the directory is empty before removal
            if fs::read_dir(path)?.count() == 0 {
                remove_dir(path)
            } else {
                Err(io::Error::new(ErrorKind::Other, "Directory is not empty"))
            }
        } else {
            // If it's a directory but no flags are set, return an error
            Err(io::Error::new(
                ErrorKind::Other,
                "Directory removal requires --recursive or --dir",
            ))
        }
    } else if path.is_file() {
        // If it's a file, simply remove it
        remove_file(path)
    } else {
        // If path is neither a file nor directory, return an error
        Err(io::Error::new(
            ErrorKind::NotFound,
            "File or directory not found",
        ))
    }
}

/// Recursively remove a directory and its contents.
///
/// ## English
/// This function recursively removes a directory and all its contents.
///
/// ## 日本語
/// この関数はディレクトリとその中身を再帰的に削除します。
fn remove_dir_all(path: &Path) -> io::Result<()> {
    // Iterate through all files and subdirectories in the directory
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            // Recursively remove subdirectories
            remove_dir_all(&entry_path)?;
        } else {
            // Remove files
            remove_file(entry_path)?;
        }
    }

    // Finally, remove the empty directory
    remove_dir(path)
}
