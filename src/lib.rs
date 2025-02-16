pub mod cli;
pub mod git_utils;
pub mod process_file;
pub mod walk_utils;

use std::{
    io::{self, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use cli::Args;
use git_utils::clone_or_fallback;
use process_file::{process_file_contents, should_process_file};
use walk_utils::walk_directory;

use base64::prelude::*;

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
        "*.txt".to_string(),
        "*.cuh".to_string(),
        "*.cu".to_string(),
    ];

    let includes = args.clone().include.unwrap_or(default_include);
    let excludes = args.clone().exclude.unwrap_or_default();

    let mut input = args.root.clone();

    // If input starts with "https://github.com", treat it as a remote GitHub repo
    if args.root.starts_with("https://github.com") {
        let repo_path = clone_or_fallback(
            &args.root,
            &args.checkout, // new argument to specify branch, commit etc.
        )?;
        input = repo_path.to_str().unwrap().to_string();
    }

    let tmp = walk_directory(&input, args.no_ignore)?;
    // filter paths
    let paths = tmp
        .iter()
        .filter(|path| path.is_file() && should_process_file(path, &includes, &excludes))
        .collect::<Vec<_>>();
    eprintln!("Found {} files to process", paths.len());

    let processed = process_files(paths, args.keep_blank_lines)?;

    if args.copy_to_clipboard {
        // prepend OSC 52 escape sequence to the processed data, and print
        // it to stdout
        eprintln!("Copying result to clipboard!");
        let mut osc52 = "\x1B]52;c;".to_string();
        osc52.push_str(&BASE64_STANDARD.encode(&processed));
        osc52.push('\x07');
        print!("{}", osc52);
        io::stdout().flush().unwrap();

        return Ok(());
    }

    match args.output {
        Some(output) => {
            eprintln!("Writing to file: {:?}", &output);
            std::fs::write(&output, processed)
                .with_context(|| format!("Failed to write to file: {:?}", &output))?;
        }
        None => {
            println!("{}", processed);
        }
    }
    Ok(())
}

/// Process a local folder by walking it, filtering files, and concatenating their contents.
fn process_files(paths: Vec<&PathBuf>, keep_blank_lines: bool) -> Result<String> {
    let mut processed = String::new();

    for path in paths {
        let data = process_file_contents(&path, keep_blank_lines)
            .with_context(|| format!("Failed to process file: {:?}", &path))?;
        processed.push_str(&data);
        processed.push_str("\n");
    }

    Ok(processed)
}
