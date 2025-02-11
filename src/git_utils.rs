use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;
use std::process::Command;

#[cfg(feature = "git")]
use git2::{build::RepoBuilder, FetchOptions};
use tempfile::tempdir;

/// Clone a remote repository using either the native Git CLI or the `git2` library.
/// Returns the path to the cloned repository (a `tempdir`).
///
/// `checkout` can be a branch name, a tag, or a commit hash if the user passed one.
pub fn clone_or_fallback(repo_url: &str, checkout: &Option<String>) -> Result<PathBuf> {
    let temp_dir = tempdir()?;
    let repo_path = temp_dir.path().to_path_buf();

    println!("Cloning repository into temporary directory...");

    // Try using native Git CLI first
    let clone_result = Command::new("git")
        .args(&["clone", "--depth", "1", repo_url])
        .arg(&repo_path)
        .output();

    let cloned_via_cli = match clone_result {
        Ok(output) if output.status.success() => {
            println!("Successfully cloned using native Git CLI");
            true
        }
        _ => false,
    };

    if !cloned_via_cli {
        println!("Native Git CLI failed, falling back to git2 library");
        #[cfg(feature = "git")]
        {
            let mut fo = FetchOptions::default();
            fo.depth(1);
            RepoBuilder::new()
                .fetch_options(fo)
                .clone(repo_url, &repo_path)
                .map_err(|e| anyhow!("Failed to clone using git2: {}", e))?;
        }
        #[cfg(not(feature = "git"))]
        {
            return Err(anyhow!("Git support is not enabled and native Git CLI failed. Please use a local folder path instead."));
        }
    }

    // If we have a branch/tag/commit specified, check it out.
    if let Some(ref co) = checkout {
        println!("Checking out '{}'", co);

        // Attempt native Git CLI first
        let checkout_result = Command::new("git")
            .current_dir(&repo_path)
            .args(["checkout", co])
            .output();

        if let Err(_) = checkout_result {
            // Fallback to git2 if available
            #[cfg(feature = "git")]
            {
                let repo = git2::Repository::open(&repo_path)?;
                let (obj, reference) = repo.revparse_ext(co)?;
                repo.checkout_tree(&obj, None)?;
                if let Some(r) = reference {
                    repo.set_head(r.name().unwrap())?;
                } else {
                    repo.set_head_detached(obj.id())?;
                }
            }
            #[cfg(not(feature = "git"))]
            {
                return Err(anyhow!(
                    "Unable to checkout '{}'. Git2 is not enabled, and native git checkout also failed.",
                    co
                ));
            }
        }
    }

    Ok(repo_path)
}
