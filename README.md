# REPOCAT 🐱

A simple CLI tool that accepts either:
1. A GitHub repository URL
2. A local folder path

and concatenates all text/code files into a single `.txt` file. This can be useful for providing context to LLMs or other tools that need a single “flattened” representation of your codebase.

## Features

- **Configurable Include/Exclude**: Specify which file types to include or exclude using glob patterns.
- **Automatic Ignore**: By default, `repocat` respects `.gitignore` and other ignore files (unless you disable it).
- **GitHub Repo Cloning**: Automatically clones a GitHub repository and concatenates matching files.
- **Checkout Specific Branch/Commit/Tag** (via `--checkout`).
- **Preserve or Strip Blank Lines** (via `--keep-blank-lines`).
- **Optionally Disable Ignore Rules** (via `--no-ignore`).

## Installation

If you have Rust (and Cargo) installed:

```bash
cargo install repocat
```

Alternatively, clone this repository and run:

```bash
cargo build --release
```

Your compiled binary will be in the `target/release` directory.

## Usage Examples

### 1. Local Folder Input

```bash
repocat --root /path/to/my-project
```

- This will walk the `my-project` folder, respecting `.gitignore` by default.
- Includes files matching `*.toml, *.md, *.py, *.rs, *.cpp, *.h, *.hpp, *.c, *.rst, *.txt, *.cuh, *.cu`.
- Writes all content into `concatenated_output.txt`.
- By default, root is `.`

### 2. GitHub Repository

```bash
repocat --root https://github.com/owner/repo
```

- Clones `repo` from GitHub into a temporary folder.
- By default, it checks out the default branch (e.g., `main` or `master`).
- Gathers all matching files and writes them to `concatenated_output.txt`.

### 3. Checking Out a Specific Branch or Commit

```bash
repocat --root https://github.com/owner/repo --checkout feature-branch
```

```bash
repocat --root https://github.com/owner/repo --checkout abcd1234
```

- Clones the specified repository, then checks out either a branch named `feature-branch` or the commit `abcd1234`.
- Proceeds to gather and concatenate files as usual.

### 4. Including and Excluding Specific File Types

```bash
repocat \
  --root /path/to/my-project \
  --include "*.rs,*.toml" \
  --exclude "*.lock,*.bak"
```

- Only gathers `.rs` and `.toml` files, while excluding anything ending with `.lock` or `.bak`.

### 5. Preserving Blank Lines

By default, repocat removes blank lines for more compact output. If you want to preserve them:

```bash
repocat --root /path/to/my-project --keep-blank-lines
```

- This keeps the blank lines in your final concatenated output.

### 6. Disabling Ignore Logic

If you want to include hidden and/or binary files, you can disable all ignore logic:

```bash
repocat --root /path/to/my-project --no-ignore
```

- This will cause repocat to walk the folder without ignoring anything.
- **Warning**: This may significantly increase the size of your output if your project has large binary files or directories like `.git`.

## Additional Info

- `repocat` uses the [ignore crate](https://github.com/BurntSushi/ripgrep) by default, which means it respects `.gitignore`, `.ignore`, and `.rgignore` files, along with hidden file filtering and binary file detection.
- The default list of “included” file extensions can be found in `src/lib.rs`, but can be overridden via the `--include` and `--exclude` flags.
- If you prefer to keep blank lines in your concatenated output, use `--keep-blank-lines`. Otherwise, empty lines are removed.

## Roadmap / Future Enhancements

- **JSON Output**: A possible future feature to output file metadata and content in a structured JSON format.
- **Partial Extraction**: Extract only certain lines or only lines matching a pattern.
- **Parallel Processing**: Speed up concatenation by reading files in parallel.

---

*Thanks for checking out repocat! Feel free to open an issue or pull request if you have suggestions or encounter any problems.*
