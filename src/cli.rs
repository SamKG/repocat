use clap::Parser;

/// Command line arguments for repocat.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// GitHub repo URL or local folder path
    #[arg(long, default_value = ".")]
    pub input: String,

    /// Output file name
    #[arg(short, long, default_value = "concatenated_output.txt")]
    pub output: String,

    /// Glob patterns to include files (e.g., "*.rs,*.toml")
    #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
    pub include: Option<Vec<String>>,

    /// Glob patterns to exclude files (e.g., "*.md,*.txt")
    #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
    pub exclude: Option<Vec<String>>,

    /// (NEW) A specific branch, tag, or commit to checkout after cloning
    /// e.g. --checkout "my-branch" or --checkout "abc123"
    #[arg(long)]
    pub checkout: Option<String>,

    /// (NEW) If set, blank lines will be preserved in the output
    #[arg(long)]
    pub keep_blank_lines: bool,

    /// (NEW) If set, repocat will not ignore hidden or binary files.
    /// (In other words, it won't use the default ignore rules from ripgrep.)
    #[arg(long)]
    pub no_ignore: bool,
}
