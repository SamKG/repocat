pub mod cli;
pub mod git_utils;
pub mod process_file;
pub mod walk_utils;

use anyhow::{Context, Result};
use cli::Args;
use git_utils::clone_or_fallback;
use process_file::{process_file_contents, should_process_file};
use std::fs::File;
use std::io::Write;
use walk_utils::walk_directory;

/// The main entry point for the library.
/// Called from main() after CLI arguments are parsed.
pub fn run_repocat(args: Args) -> Result<()> {
    // Prepare default includes
    let default_include = vec![
        "*.toml".to_string(),
        "*.md".to_string(),
        "*.py".to_string(),
        "*.rs".to_string(),
        "*.cpp".to_string(),
        "*.h".to_string(),
        "*.hpp".to_string(),
        "*.c".to_string(),
        "*.rst".to_string(),
        "*.txt".to_string(),
        "*.cuh".to_string(),
        "*.cu".to_string(),
    ];

    let includes = args.clone().include.unwrap_or(default_include);
    let excludes = args.clone().exclude.unwrap_or_default();

    let mut input = args.input.clone();

    // If input starts with "https://github.com", treat it as a remote GitHub repo
    if args.input.starts_with("https://github.com") {
        let repo_path = clone_or_fallback(
            &args.input,
            &args.checkout, // new argument to specify branch, commit etc.
        )?;
        input = repo_path.to_str().unwrap().to_string();
    }
    process_local_folder(&input, &args.output, &includes, &excludes, &args)?;

    println!(
        "All matching files have been concatenated into '{}'",
        &args.output
    );
    Ok(())
}

/// Process a local folder by walking it, filtering files, and concatenating their contents.
fn process_local_folder(
    folder_path: &str,
    output_file: &str,
    include: &[String],
    exclude: &[String],
    args: &Args,
) -> Result<()> {
    let mut output = File::create(output_file).context("Failed to create output file")?;

    let paths = walk_directory(folder_path, args.no_ignore)?;

    for path in paths {
        if path.is_file() && should_process_file(&path, include, exclude) {
            let data = process_file_contents(&path, args.keep_blank_lines)
                .with_context(|| format!("Failed to process file: {:?}", &path))?;
            println!("{}", path.to_str().unwrap());
            writeln!(output, "{}", data)?;
        }
    }
    Ok(())
}
