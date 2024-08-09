# REPOCAT 🐱

This is a cli that accepts either:
1) a github repo url
2) a path to a folder

and concatenates all text/code files into a single txt file. This makes it easier to use as context for LLMs.

## What file extensions does it look for?
Check [src/main.rs] for extensions. Feel free to make a PR to add more

## Does it automatically filter some files?
Yes! repocat uses the [ignore crate from ripgrep](https://github.com/BurntSushi/ripgrep/blob/master/GUIDE.md#automatic-filtering), meaning it ignores all of the following by default:

    Files and directories that match glob patterns in these three categories:
        .gitignore globs (including global and repo-specific globs). This includes .gitignore files in parent directories that are part of the same git repository. (Unless the --no-require-git flag is given.)
        .ignore globs, which take precedence over all gitignore globs when there's a conflict. This includes .ignore files in parent directories.
        .rgignore globs, which take precedence over all .ignore globs when there's a conflict. This includes .rgignore files in parent directories.
    Hidden files and directories.
    Binary files. (ripgrep considers any file with a NUL byte to be binary.)
    Symbolic links aren't followed.


