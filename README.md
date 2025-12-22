# git-issue 🗂️

A Git-native, file-backed issue tracker.

Issues live alongside your code inside `.gitissues/`, making them platform-independent, version-controlled, branchable, mergeable, reviewable and offline-friendly.

## Features

- ✅ `git issue init`                       -- Initialize `.gitissues/` in your repository
- ✅ `git issue new <"title">`              -- Create a new issue with auto-incremented ID
- ✅ `git issue list [--columns <columns>]` -- List all issues
- ✅ `git issue show <id>`                  -- Display issue details
- ✅ `git issue set <id> [--state <new_state> --type <new_type> --title <new_title> --assignee <new_assignee> --labels <add_labels>]` -- Change issue meta fields
- ✅ `git issue remove <id> [--labels <remove_labels>]` -- Remove elements from issue meta fields (currently only for labels)
- ✅ `git issue edit <id>`                  -- Edit issue description in external editor
- ✅ Git integration                        -- Auto-commit `.gitissues/` changes (configurable)
- 🚧 Testing                                -- CI/CD automated tests

## Usage

```bash
# Help page
git issue -h

# Initialize tracking in your repo
git issue init

# Create a new issue
git issue new "Fix login redirect bug"

# List issues
git issue list
git issue list --columns id,assignee,title
git issue list --columns "*"

# Show issue details
git issue show 1234

# Change issue meta fields
git issue set 1234 --state resolved
git issue set 1234 --type bug
git issue set 1234 --title "LCD driver has a problem"
git issue set 1234 --assignee "t.burkard"
git issue set 1234 --labels cli,driver
git issue set 1234 --state resolved --type bug --title "LCD driver has a problem" --assignee "t.burkard" --labels cli,driver

# Remove elements from issue meta fields (currently only labels)
git issue remove 1234 --labels cli
git issue remove 1234 --labels cli,driver

# Edit issue description (markdown) -- launches external text editor
git issue edit 1234
```

## Example

Dummy example project to see how `git-issue` is used in a repo: https://github.com/timonburkard/example-project

## Configuration

### Config

After running `git issue init`, a default config file is created at `.gitissues/config.yaml`:

```yaml
# Automatically create a git commit after mutating commands
commit_auto: true

# Commit message template
# Available placeholders: {action}, {id}, {title}
commit_message: "[issue] {action} #{id} - {title}"

# Editor for editing issue descriptions
# git = use the git-configured editor
editor: git
```

This config can be edited by the user.

#### Config Options

- `commit_auto` (boolean): If `true`, automatically commit changes to `.gitissues/`. Default: `true`
- `commit_message` (string): Template for git commit messages. Supports placeholders:
  - `{id}`: Issue ID
  - `{title}`: Issue title
  - `{action}`: Command that triggered the commit (`new`, `edit`, `set X`, `remove X`)
- `editor` (string): External text editor (set `git` to use configured git core.editor)

### Description Template

After running `git issue init`, a default description template file is created at `.gitissues/description.md`:

```md
# Description

# Repro Steps

# Expected Behavior

# System Info

```

This template can be edited by the user.

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
├── config.yaml     # Configuration
├── description.md  # Description template
└── issues/
    └── 0000000001/
        ├── meta.yaml       # Structured metadata
        └── description.md  # Markdown description
    ├── 0000000002/
        ├── meta.yaml       # Structured metadata
        └── description.md  # Markdown description
    └── ...
```

- `meta.yaml`      -- metadata: id, title, state, timestamps
- `description.md` -- template for the human-readable markdown description
- Directory names are the 10-digit zero-padded IDs (0000000001, 0000000002, …)

### meta.yaml Format

```yaml
id: 1234
title: Fix login bug
state: new
type: ''
labels: []
assignee: ''
created: 2025-12-21T15:54:52Z
updated: 2025-12-21T15:54:52Z
```

## Architecture

- `config/`
  - `config-default.yaml`    -- Default configuration, applied at `git issue init`
  - `description-default.md` -- Default description template, applied at `git issue init`
- `src/`
  - `main.rs`   -- CLI parsing with clap
  - `model.rs`  -- Shared data types, functions and utilities
  - `edit.rs`   -- Edit issue description (markdown) with external text editor
  - `init.rs`   -- Initialize `.gitissues/` directory and copy default config
  - `list.rs`   -- List all issues
  - `new.rs`    -- Create new issues with ID allocation
  - `remove.rs` -- Remove elements from issue meta fields (currently only labels)
  - `set.rs`    -- Change issue meta fields
  - `show.rs`   -- Show details of an issue

## Dependencies

- `clap`        -- CLI argument parsing
- `chrono`      -- Timestamp generation
- `serde`       -- Serialization framework
- `serde_yaml`  -- YAML parsing for meta.yaml files
- `shell-words` -- Process command line according to parsing rules of Unix shell
