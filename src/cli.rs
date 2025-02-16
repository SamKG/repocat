use clap::Parser;

/// Command line arguments for repocat.
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// GitHub repo URL or local folder path
    #[arg(short, long, default_value = ".")]
    pub root: String,

    /// Output to file name, if provided. Else, output to stdout.
    #[arg(short, long)]
    pub output: Option<String>,

    /// If set, outputs using OSC 52 escape sequences to copy to clipboard directly.
    #[arg(short, long = "cb", long, conflicts_with = "output")]
    pub copy_to_clipboard: bool,

    /// Glob patterns to include files. Typically used to indicate file extensions to focus on. (e.g., "*.rs,*.toml")
    #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
    pub include: Option<Vec<String>>,

    /// Glob patterns to exclude files (e.g., "*.md,*.txt")
    #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
    pub exclude: Option<Vec<String>>,

    /// A specific branch, tag, or commit to checkout after cloning
    /// e.g. --checkout "my-branch" or --checkout "abc123"
    #[arg(long)]
    pub checkout: Option<String>,

    /// If set, blank lines will be preserved in the output
    #[arg(long)]
    pub keep_blank_lines: bool,

    /// If set, repocat will not ignore hidden or binary files.
    /// (In other words, it won't use the default ignore rules from ripgrep.)
    #[arg(long)]
    pub no_ignore: bool,
}
