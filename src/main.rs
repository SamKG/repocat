use anyhow::{Context, Result};
use clap::Parser;
use glob::Pattern;
use ignore::WalkBuilder;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

#[cfg(feature = "git")]
use git2::FetchOptions;
#[cfg(feature = "git")]
use tempfile::TempDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// GitHub repo URL or local folder path
    #[arg(short, long)]
    input: String,

    /// Output file name
    #[arg(short, long, default_value = "concatenated_output.txt")]
    output: String,

    /// Glob patterns to include files (e.g., "*.rs,*.toml")
    #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
    include: Option<Vec<String>>,

    /// Glob patterns to exclude files (e.g., "*.md,*.txt")
    #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
    exclude: Option<Vec<String>>,
}

fn main() -> Result<()> {
    let args = Args::parse();

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

    let include = args.include.unwrap_or(default_include);
    let exclude = args.exclude.unwrap_or_default();

    if args.input.starts_with("https://github.com") {
        process_github_repo(&args.input, &args.output, &include, &exclude)?;
    } else {
        process_local_folder(&args.input, &args.output, &include, &exclude)?;
    }

    println!(
        "All matching files have been concatenated into '{}'",
        args.output
    );
    Ok(())
}

fn process_github_repo(
    repo_url: &str,
    output_file: &str,
    include: &[String],
    exclude: &[String],
) -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let repo_path = temp_dir.path();

    println!("Cloning repository...");

    // Try using native Git CLI first
    let clone_result = Command::new("git")
        .args(&["clone", "--depth", "1", repo_url])
        .arg(repo_path)
        .output();

    match clone_result {
        Ok(output) if output.status.success() => {
            println!("Successfully cloned using native Git CLI");
        }
        _ => {
            println!("Native Git CLI failed, falling back to git2 library");
            #[cfg(feature = "git")]
            {
                let mut binding = FetchOptions::default();
                binding.depth(1);
                git2::build::RepoBuilder::new()
                    .fetch_options(binding)
                    .clone(repo_url, repo_path)?;
            }
            #[cfg(not(feature = "git"))]
            {
                return Err(anyhow::anyhow!("Git support is not enabled and native Git CLI failed. Please use a local folder path instead."));
            }
        }
    }

    process_local_folder(repo_path.to_str().unwrap(), output_file, include, exclude)
}

fn should_process_file(path: &Path, include: &[String], exclude: &[String]) -> bool {
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

fn process_file(file_path: &Path) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // strip consecutive newlines and excess whitespace
    let processed_lines: Vec<String> = contents
        .split('\n')
        .map(str::trim_end)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();
    Ok(format!(
        "*** {}\n{}",
        file_path.to_str().unwrap(),
        processed_lines.join("\n")
    ))
}

fn process_local_folder(
    folder_path: &str,
    output_file: &str,
    include: &[String],
    exclude: &[String],
) -> Result<()> {
    let mut output = File::create(output_file).context("Failed to create output file")?;
    let walker = WalkBuilder::new(folder_path).build();
    for result in walker {
        let entry = result?;
        let path = entry.path();
        if path.is_file() && should_process_file(path, include, exclude) {
            let data = process_file(path).context("Failed to process file")?;
            println!("{}", path.to_str().unwrap());
            writeln!(output, "{}", data)?;
        }
    }
    Ok(())
}
