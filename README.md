# git-issue 🗂️

A Git-native, file-backed issue tracker.

Issues live alongside your code inside `.gitissues/`, making them platform-independent, version-controlled, branchable, mergeable, reviewable and offline-friendly.

## Features

- ✅ Git-native, file-backed issues under `.gitissues/`
- ✅ Core commands: `init`, `new`, `list`, `show`, `set`, `edit`
- ✅ Each issue has a markdown description incl. attachments
- ✅ Each issue has metadata: `id`, `title`, `state`, `type`, `labels`, `assignee`, `priority`, `due_date`, `created`, `updated`
- ✅ Configurable: default columns for `list`, commit message template, external editor
- ✅ External editor renders issue information as markdown
- ✅ Git-integration: auto-commit of changes
- 🚧 Filtering/sorting of `list` view
- 🚧 Add `search` command across all issue titles and descriptions
- 🚧 Relationships between issues
- 🚧 Comments / discussions
- 🚧 Automated tests

## Usage

```bash
# Help page
git issue -h

# Version
git issue -V
git issue --version

# Initialize tracking in your repo
git issue init
git issue init --no-commit

# Create a new issue
git issue new "Login redirection problem"
git issue new "Login redirection problem" --type bug --labels software,ui --assignee t.burkard --priority P1 --due-date 2026-02-15

# List issues
git issue list
git issue list --columns id,assignee,title
git issue list --columns "*"

# Show all issue information (markdown) -- launches external text editor
git issue show 1234

# Change issue meta fields
git issue set 1234 --title "LCD driver has a problem"
git issue set 1234 --state resolved --type bug --assignee "t.burkard" --priority P1 --due-date 2026-01-31

# Change issue meta fields: labels
git issue set 1234 --labels        cli,driver  # set labels (overwrite)
git issue set 1234 --labels-add    cli,driver  # add labels
git issue set 1234 --labels-remove cli,driver  # remove labels

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

# Default columns to display in `issue list`
# ["*"] can be used to include all available columns
list_columns:
  - id
  - state
  - assignee
  - title
```

This config can be edited by the user.

#### Config Options

- `commit_auto` (boolean): If `true`, automatically commit changes to `.gitissues/`. Default: `true`
- `commit_message` (string): Template for git commit messages. Supports placeholders:
  - `{id}`: Issue ID
  - `{title}`: Issue title
  - `{action}`: Command that triggered the commit (`new`, `edit description`, `set <field>`)
- `editor` (string): External text editor (set `git` to use configured git core.editor)
- `list_columns` (string list): Default columns shown in `list` command

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
├── .tmp/           # Temporary files: Put in `.gitignore`
├── config.yaml     # Configuration
├── description.md  # Description template
└── issues/
    └── 0000000001/
        ├── meta.yaml       # Structured metadata
        ├── description.md  # Markdown description
        └── attachments/    # Attachments of markdown description
    ├── 0000000002/
        ├── meta.yaml       # Structured metadata
        ├── description.md  # Markdown description
        └── attachments/    # Attachments of markdown description
    └── ...
```

### meta.yaml Format

```yaml
id: 1234                       # (Integer) Identifier
title: Login screen is broken  # (String) Title
state: new                     # (String) E.g.: new, active, resolved, junked
type: bug                      # (String) E.g.: feature, bug, task
labels:                        # (List of Strings) Labels / tags
  - software
  - ui
assignee: t.burkard            # (String) To whom the issue is assigned
priority: P2                   # (Enum) Priority: P0 = highest, P2 = default, P4 = lowest
due_date: 2026-01-31           # (Date) Due date in ISO format: YYYY-MM-DD
created: 2025-11-13T15:54:52Z  # (Timestamp) Issue was created at
updated: 2025-12-22T20:36:11Z  # (Timestamp) Issue was last updated at
```

## Architecture

- `config/`
  - `config-default.yaml`    -- Default configuration, applied at `git issue init`
  - `description-default.md` -- Default description template, applied at `git issue init`
- `src/`
  - `main.rs`  -- CLI parsing with clap
  - `model.rs` -- Shared data types, functions and utilities
  - `edit.rs`  -- Edit issue description (markdown) with external text editor
  - `init.rs`  -- Initialize `.gitissues/` directory and copy default config
  - `list.rs`  -- List all issues
  - `new.rs`   -- Create new issues
  - `set.rs`   -- Change issue meta fields
  - `show.rs`  -- Show details of an issue

## Dependencies

- `clap`        -- CLI argument parsing
- `chrono`      -- Timestamp generation
- `serde`       -- Serialization framework
- `serde_yaml`  -- YAML parsing for meta.yaml files
- `shell-words` -- Process command line according to parsing rules of Unix shell
- `regex`       -- Regular expressions
