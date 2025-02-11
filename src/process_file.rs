use anyhow::Result;
use glob::Pattern;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Determine whether a file should be processed based on include and exclude patterns.
pub fn should_process_file(path: &Path, include: &[String], exclude: &[String]) -> bool {
    let path_str = path.to_string_lossy();

    let included = include.iter().any(|pattern| {
        Pattern::new(pattern)
            .map(|p| p.matches(&path_str))
            .unwrap_or(false)
    });

    let excluded = exclude.iter().any(|pattern| {
        Pattern::new(pattern)
            .map(|p| p.matches(&path_str))
            .unwrap_or(false)
    });

    included && !excluded
}

/// Read file contents, optionally strip blank lines, and produce a final string output.
pub fn process_file_contents(path: &Path, keep_blank_lines: bool) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // If user doesn't want blank lines, filter them out.
    let processed_lines: Vec<String> = if keep_blank_lines {
        // Just trim the end of each line to remove trailing whitespace
        contents
            .lines()
            .map(str::trim_end)
            .map(String::from)
            .collect()
    } else {
        contents
            .lines()
            .map(str::trim_end)
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect()
    };

    // Prepend the file path as a header
    Ok(format!(
        "*** {}\n{}",
        path.to_str().unwrap_or("Unknown Path"),
        processed_lines.join("\n")
    ))
}
