use std::{
    cmp::Ordering,
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

use crate::{state::DirEntryUi, types::SortKey};

fn is_hidden(name: &str, _path: &Path) -> bool {
    // Cross-platform: dotfiles. (Windows hidden attribute is a later upgrade.)
    name.starts_with('.')
}

fn modified_unix(meta: &fs::Metadata) -> i64 {
    meta.modified()
        .ok()
        .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub fn list_dir(path: &Path, show_hidden: bool, sort: SortKey) -> std::io::Result<Vec<DirEntryUi>> {
    let mut out = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let p = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let meta = entry.metadata().ok();

        let is_dir = meta.as_ref().map(|m| m.is_dir()).unwrap_or(false);
        let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
        let modified_unix = meta.as_ref().map(modified_unix).unwrap_or(0);
        let hidden = is_hidden(&name, &p);

        if !show_hidden && hidden {
            continue;
        }

        out.push(DirEntryUi {
            path: p,
            name,
            is_dir,
            size,
            modified_unix,
            is_hidden: hidden,
        });
    }

    out.sort_by(|a, b| {
        // directories first
        match (a.is_dir, b.is_dir) {
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            _ => {}
        }

        match sort {
            SortKey::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            SortKey::Modified => b.modified_unix.cmp(&a.modified_unix),
            SortKey::Size => b.size.cmp(&a.size),
        }
    });

    Ok(out)
}

pub fn delete_path(path: &PathBuf) -> std::io::Result<()> {
    let meta = fs::symlink_metadata(path)?;
    if meta.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    Ok(())
}
