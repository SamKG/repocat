use anyhow::{Context, Result};
use clap::Parser;
use git2::FetchOptions;
use ignore::WalkBuilder;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
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

    /// Optional JSON config file
    #[arg(short, long)]
    config: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    file_extensions: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let config = load_config(&args.config)?;

    if args.input.starts_with("https://github.com") {
        process_github_repo(&args.input, &args.output, &config)?;
    } else {
        process_local_folder(&args.input, &args.output, &config)?;
    }

    println!(
        "All text files have been concatenated into '{}'",
        args.output
    );
    Ok(())
}

fn load_config(config_path: &Option<PathBuf>) -> Result<Config> {
    match config_path {
        Some(path) => {
            let config_str = fs::read_to_string(path).context("Failed to read config file")?;
            serde_json::from_str(&config_str).context("Failed to parse config file")
        }
        None => Ok(Config {
            file_extensions: vec![
                // "toml".to_string(),
                // "md".to_string(),
                "py".to_string(),
                // "rs".to_string(),
                // "cpp".to_string(),
                // "h".to_string(),
                // "hpp".to_string(),
                // "c".to_string(),
                // "rst".to_string(),
                // "txt".to_string(),
            ],
        }),
    }
}

fn process_github_repo(repo_url: &str, output_file: &str, config: &Config) -> Result<()> {
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();

    println!("Cloning repository...");
    let mut binding = FetchOptions::default();
    binding.depth(1);
    let repo = git2::build::RepoBuilder::new()
        .fetch_options(binding)
        .clone(repo_url, repo_path)?;

    process_local_folder(repo_path.to_str().unwrap(), output_file, config)
}

fn has_valid_extension(config: &Config, entry: &ignore::DirEntry) -> bool {
    entry
        .path()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| config.file_extensions.contains(&ext.to_string()))
        .unwrap_or(false)
}

fn process_file(file_path: &Path) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // strip consecutive newlines and excess whitespace
    let mut processed_lines: Vec<String> = vec![];
    for line in contents.split("\n").map(str::trim_end).filter(|s| s.len() > 0){
        processed_lines.push(line.to_string());
    }
    Ok(format!("*** {}\n{}", file_path.to_str().unwrap(), processed_lines.join("\n")))
}

fn process_local_folder(folder_path: &str, output_file: &str, config: &Config) -> Result<()> {
    let mut output = File::create(output_file).context("Failed to create output file")?;
    let walker = WalkBuilder::new(folder_path).build();
    for result in walker.filter_ok(|p| has_valid_extension(config, p)) {
        let entry = result?;
        let path = entry.path();
        let data = process_file(path).context("Failed to process file")?;
        println!("{}", path.to_str().unwrap());
        writeln!(output, "{}", data)?;
    }
    Ok(())
}
