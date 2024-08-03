use std::cmp::Ordering;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{fs, io};

pub mod print;
pub mod tree;

use self::tree::{DirectoryNode, FSNode, FileNode};

fn is_hidden(name: String) -> bool {
    name.starts_with('.')
}

pub fn read_recursive(path: &Path, follow_symlinks: bool) -> FSNode {
    let name = path
        .file_name()
        .unwrap_or(path.as_os_str())
        .to_string_lossy()
        .to_string();
    let mut node = DirectoryNode::new(name);

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                let entry = match entry {
                    Ok(e) => e,
                    Err(err) => {
                        let _ = writeln!(
                            io::stderr(),
                            "Error reading {:?}, caused by I/O error: {}",
                            path,
                            err
                        );
                        continue;
                    }
                };
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                let hidden = is_hidden(name.clone());
                let meta = match if follow_symlinks {
                    fs::metadata(&path)
                } else {
                    entry.metadata()
                } {
                    Ok(m) => m,
                    Err(err) => {
                        let _ = writeln!(
                            io::stderr(),
                            "Error reading {:?}, caused by I/O error: {}",
                            path,
                            err
                        );
                        continue;
                    }
                };

                if meta.is_file() {
                    node.children.push(FSNode::File(FileNode {
                        name,
                        size: meta.len(),
                        hidden,
                        symlink: meta.file_type().is_symlink(),
                        executable: meta.permissions().mode() & 0o111 != 0,
                    }));
                    node.size += meta.len();
                } else if meta.is_dir() {
                    let dir = read_recursive(&path, follow_symlinks);
                    node.size += dir.size();
                    node.children.push(dir);
                }
            }
        }
        Err(err) => {
            let _ = writeln!(
                io::stderr(),
                "Error reading {:?}, caused by I/O error: {}",
                path,
                err
            );
        }
    }

    node.children.sort_by(biggest_size_first);
    FSNode::Directory(node)
}

fn biggest_size_first(lhs: &FSNode, rhs: &FSNode) -> Ordering {
    lhs.size().cmp(&rhs.size()).reverse()
}
