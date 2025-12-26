# git-issue 🗂️

A Git-native, file-backed issue tracker.

Issues live alongside your code inside `.gitissues/`, making them platform-independent, version-controlled, branchable, mergeable, reviewable and offline-friendly.

## Features

- ✅ Git-native, file-backed issues under `.gitissues/`
- ✅ Core commands: `init`, `new`, `list`, `show`, `set`, `edit`, `link`
- ✅ Each issue has a markdown description incl. attachments
- ✅ Each issue has metadata: `id`, `title`, `state`, `type`, `labels`, `assignee`, `priority`, `due_date`, `created`, `updated`
- ✅ Each issue has `relationships`: Desired relationship categories (e.g, related, child/parent, ...) are configurable and bidirectional links can be managed automatically
- ✅ Highly configurable: default columns for `list`, available options for `state` and `type`, relation ship categories, commit message template, external editor
- ✅ External editor renders issue information as markdown
- ✅ Git-integration: auto-commit of changes
- ✅ Automated integration tests
- 🚧 Add `search` command across all issue titles and descriptions
- 🚧 Comments / discussions

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
git issue list --filter priority=p2 title=*driver*

# Show all issue information (markdown) -- launches external text editor
git issue show 1234

# Change issue meta fields
git issue set 1234 --title "LCD driver has a problem"
git issue set 1234 --state resolved --type bug --assignee "t.burkard" --priority P1 --due-date 2026-01-31

# Change issue meta fields: labels
git issue set 1234 --labels cli,driver         # set labels (overwrite)
git issue set 1234 --labels-add cli,driver     # add labels
git issue set 1234 --labels-remove cli,driver  # remove labels

# Change issue relationships
git issue link 1234 --add related=5678                                       # add relationship link
git issue link 1234 --remove related=5678                                    # remove relationship links
git issue link 1234 --add related=5678,3333 parent=9999 --remove child=7777  # batch update relationship links

# Edit issue description (markdown) -- launches external text editor
git issue edit 1234
```

## Example

Dummy example project to see how `git-issue` is used in a repo: https://github.com/timonburkard/example-project

## Configuration

### Config

After running `git issue init`, default config file and users are created at `.gitissues/config.yaml` resp. `.gitissues/users.yaml`.

These files can be edited by the user.

#### config.yaml

```yaml
# Automatically create a git commit after mutating commands
commit_auto: true

# Commit message template
# Available placeholders: {action}, {id}, {title}
commit_message: "[issue] {action} #{id} -- {title}"

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

# Available issue states
# First state in the list is the initial state for new issues
states:
  - new
  - active
  - closed
  - deleted

# Available issue types
# Per default the type is empty for new issues
types:
  - bug
  - feature
  - task

# Available relationships between issues
# link: specifies the name of the reciprocal relationship
#  - same name:      bidirectional, symmetric
#  - different name: bidirectional, asymmetric
#  - null:           unidirectional
relationships:
  related:
    link: related
  parent:
    link: child
  child:
    link: parent
```

#### Options

- `commit_auto` (boolean): If `true`, automatically commit changes to `.gitissues/`. Default: `true`
- `commit_message` (string): Template for git commit messages. Supports placeholders:
  - `{id}`: Issue ID
  - `{title}`: Issue title
  - `{action}`: Command that triggered the commit (`new`, `edit description`, `set <field>`)
- `editor` (string): External text editor (set `git` to use configured git core.editor)
- `list_columns` (list of strings): Default columns shown in `list` command
- `states` (list of strings): Available issue states. Default is the first element.
- `types` (list of strings): Available issue types. Default is empty.
- `relationships` (object): Available relationships between issues

#### users.yaml

```yaml
users:
  - id: alice
  - id: bob
  - id: carol
```

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

Different installation approaches are explained here.

### GitHub Release

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

### Cargo

Installation with cargo works as follows:

```
cargo install --git https://github.com/timonburkard/git-issue
```

### Crates

Package is available on https://crates.io/crates/git-issue, so it can be installed as follows:

```
cargo install git-issue
```

## Building & Development

```bash
# Build
cargo build

# Format code
cargo fmt

# Lint
cargo clippy

# Run tests
cargo test
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
relationships:                 # (Object) Relationships with other issues
  related:
  - 5678
  - 7777
  parent:
  - 5555
  child:
  - 3333
  - 4444
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
  - `link.rs`  -- Change relationships between issues
  - `list.rs`  -- List all issues
  - `new.rs`   -- Create new issues
  - `set.rs`   -- Change issue meta fields
  - `show.rs`  -- Show all issue information

## Dependencies

- `clap`        -- CLI argument parsing
- `chrono`      -- Timestamp generation
- `serde`       -- Serialization framework
- `serde_yaml`  -- YAML parsing for meta.yaml files
- `shell-words` -- Process command line according to parsing rules of Unix shell
- `regex`       -- Regular expressions
- `indexmap`    -- Provides IndexMap datatype
