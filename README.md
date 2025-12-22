# git-issue 🗂️

A Git-native, file-backed issue tracker.

Issues live alongside your code inside `.gitissues/`, making them platform-independent, version-controlled, branchable, mergeable, reviewable and offline-friendly.

## Features

- ✅ `git issue init`                      -- Initialize `.gitissues/` in your repository
- ✅ `git issue new <"title">`             -- Create a new issue with auto-incremented ID
- ✅ `git issue list [--column <columns>]` -- List all issues
- ✅ `git issue show <id>`                 -- Display issue details
- ✅ `git issue state <id> <state>`        -- Change issue state
- ✅ Git integration                       -- Auto-commit `.gitissues/` changes (configurable)
- 🚧 Testing                               -- CI/CD automated tests

## Usage

```bash
# Initialize tracking in your repo
git issue init

# Create a new issue
git issue new "Fix login redirect bug"

# List issues
git issue list

# Show issue details
git issue show 1234

# Change issue state
git issue state 1234 resolved
```

## Example

Dummy example project to see how `git-issue` is used in a repo: https://github.com/timonburkard/example-project

## Configuration

After running `git issue init`, a default config file is created at `.gitissues/config.yaml`:

```yaml
# Automatically create a git commit after mutating commands
commit_auto: true

# Commit message template
# Available placeholders: {action}, {id}, {title}
commit_message: "[issue] {action} #{id} - {title}"
```

### Configuration Options

- `commit_auto` (boolean): If `true`, automatically commit changes to `.gitissues/`. Default: `true`
- `commit_message` (string): Template for git commit messages. Supports placeholders:
  - `{id}`: Issue ID
  - `{title}`: Issue title
  - `{action}`: Command that triggered the commit (`new`, `state change`)

## Installation

Download the latest release from GitHub and put the binary on your PATH.

1) Go to the Releases page and download the binary for your platform:
   - `git-issue-linux-x86_64`
   - `git-issue-macos-x86_64` or `git-issue-macos-aarch64`
   - `git-issue-windows-x86_64.exe`
2) Rename to the canonical name and place on your PATH
   - Linux/macOS:
     ```bash
     mv git-issue-<your-platform> git-issue
     chmod +x git-issue
     sudo mv git-issue /usr/local/bin/
     ```
   - Windows: rename `git-issue-windows-x86_64.exe` to `git-issue.exe` and move it to a directory on your PATH.
3) Verify:
   ```bash
   git issue -h
   ```

## Building & Development

```bash
# Build
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## Storage Layout

Issues live in `.gitissues/issues/{ID}/`:

```
.gitissues/
└── issues/
    └── 0000000001/
        ├── meta.yaml      # Structured metadata
        └── issue.md       # Markdown description
    ├── 0000000002
        ├── meta.yaml      # Structured metadata
        └── issue.md       # Markdown description
    └── ...
```

- `meta.yaml` (metadata: id, title, state, timestamps)
- `issue.md` (human-readable markdown description)
- Directory names are the 10-digit zero-padded IDs (0000000001, 0000000002, …)

### meta.yaml Format

```yaml
id: 1234
title: Fix login bug
state: new
created: 2025-12-21T15:54:52Z
updated: 2025-12-21T15:54:52Z
```

### issue.md Format

```markdown
# Fix login bug

## Description

TBD
```

## Architecture

- `config/`
  - `config-default.yaml` -- Default configuration, applied at `git issue init`
- `src/`
  - `main.rs`  -- CLI parsing with clap
  - `model.rs` -- Shared data types, functions and utilities
  - `init.rs`  -- Initialize `.gitissues/` directory and copy default config
  - `list.rs`  -- List all issues
  - `new.rs`   -- Create new issues with ID allocation
  - `show.rs`  -- Show details of an issue
  - `state.rs` -- Change issue state

## Dependencies

- `clap`       -- CLI argument parsing
- `chrono`     -- Timestamp generation
- `serde`      -- Serialization framework
- `serde_yaml` -- YAML parsing for meta.yaml files
