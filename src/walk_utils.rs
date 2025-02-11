use anyhow::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

/// Walk the directory, optionally ignoring hidden & binary files if `no_ignore` is false.
pub fn walk_directory(folder_path: &str, no_ignore: bool) -> Result<Vec<PathBuf>> {
    let mut builder = WalkBuilder::new(folder_path);

    // If user wants no ignore logic, we disable all filtering
    if no_ignore {
        builder.hidden(false);
        builder.ignore(false);
        builder.git_ignore(false);
        builder.git_exclude(false);
        builder.follow_links(true);
    }

    let mut paths = Vec::new();
    for result in builder.build() {
        let entry = result?;
        paths.push(entry.path().to_path_buf());
    }
    Ok(paths)
}
