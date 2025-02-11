use anyhow::Result;
use clap::Parser;
use repocat::{cli::Args, run_repocat};

fn main() -> Result<()> {
    let args = Args::try_parse()?;
    run_repocat(args)
}
