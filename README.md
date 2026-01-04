# git-issue 🗂️

A Git-native, file-backed issue tracker.

Issues live alongside your code inside `.gitissues/`, making them platform-independent, version-controlled, branchable, mergeable, reviewable and offline-friendly.

## 1.) Features

- ✅ Git-native, file-backed issues under `.gitissues/`
- ✅ Core commands: `init`, `new`, `list`, `show`, `set`, `edit`, `link`
- ✅ Each issue has a markdown description incl. attachments
- ✅ Each issue has metadata: `id`, `title`, `state`, `type`, `labels`, `reporter`, `assignee`, `priority`, `due_date`, `created`, `updated`
- ✅ Each issue has `relationships`: Desired relationship categories (e.g, related, child/parent, ...) are configurable and bidirectional links can be managed automatically
- ✅ Issues can be filtered and sorted
- ✅ Highly configurable: default columns for `list`, available options for `state` and `type`, relationship categories, commit message template, external editor, and more...
- ✅ External editor renders issue information as markdown
- ✅ Git-integration: auto-commit of changes
- ✅ Possibility to export issue list into CSV file
- ✅ Automated integration tests
- 🚧 Add `search` command across all issue titles and descriptions
- 🚧 Comments / discussions

## 2.) Usage

### 2.1) Installation

Different installation approaches are explained here.

#### 2.1.1) GitHub Release

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

#### 2.1.2) Cargo

Installation with cargo works as follows:

```
cargo install --git https://github.com/timonburkard/git-issue
```

#### 2.1.3) Crates

Package is available on https://crates.io/crates/git-issue, so it can be installed as follows:

```
cargo install git-issue
```

### 2.2) How To

Lets imagine this is the structure of your git repo, for which you want to add issue tracking:

```
.git/
src/
README.md
.gitignore
```

In the root of your repo, run:

```bash
git issue init
```

This will automatically create the `.gitissues/` directory in your git repo:

```
.git/
.gitissues/
src/
README.md
.gitignore
```

For infos about the `.gitissues/` directory structure, see chapter [4.) Storage Layout](#4-storage-layout).

#### 2.2.1) Gitignore

This is the suggested content for the `.gitignore`:

```
.gitissues/.tmp/
.gitissues/exports/
.gitissues/settings.yaml
```

### 2.3) Commands

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
git issue new 'Login redirection problem'
git issue new 'Login redirection problem' --type bug --labels software,ui --reporter alice --assignee bob --priority P1 --due-date 2026-02-15
git issue new 'Login redirection problem' --reporter me --assignee me  # 'me' is automatically replaced with `settings.yaml:user`

# List issues
git issue list
git issue list --columns id,assignee,title
git issue list --columns '*'
git issue list --filter priority=P2 title=*driver* assignee=''
git issue list --filter due_date\>2025-05-31 due_date\<2026-01-01
git issue list --sort assignee=asc priority=desc
git issue list --csv  # export issue list into CSV file (.gitissues/exports/)

# Show all issue information (markdown) -- launches external text editor
git issue show 1234

# Change issue meta fields
git issue set 1234 --title 'LCD driver has a problem'
git issue set 1234 --state resolved --type bug --reporter alice --assignee bob --priority P1 --due-date 2026-01-31
git issue set 1234 --reporter me --assignee me  # 'me' is automatically replaced with `settings.yaml:user`

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

### 2.4) Example

Dummy example project to see how `git-issue` is used in a repo: [Example Project](https://github.com/timonburkard/example-project)

## 3.) Configuration

After running `git issue init`, the following default files are automatically created:

 - `.gitissues/config.yaml`:    Project configuration file (should be version-controlled)
 - `.gitissues/settings.yaml`:  Local user settings file (should **not** be version-controlled)
 - `.gitissues/users.yaml`:     Users (should be version-controlled)
 - `.gitissues/description.md`: Issue description template (should be version-controlled)

These files can be edited by the user.

### 3.1) config.yaml

This file holds the project configuration. It should be version-controlled.

```yaml
# Automatically create a git commit after mutating commands
commit_auto: true

# Commit message template
# Available placeholders: {action}, {id}, {title}
commit_message: '[issue] {action} #{id} -- {title}'

# Default columns to display in `issue list`
# ['*'] can be used to include all available columns
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

# Separator used when exporting to CSV
export_csv_separator: ','

# ID generation strategy (always u32)
# Options:
#  - sequential: Sequential numbers (1, 2, 3, ...)
#  - timestamp:  Timestamps in seconds since 2025-01-01
#                (in teams this reduces the chance of merge conflicts)
id_generation: sequential

# Default priority for new issues
# Options: '', P0, P1, P2, P3, P4
priority_default: ''
```

#### 3.1.1) Options

- `commit_auto` (boolean): If `true`, automatically commit changes to `.gitissues/`
- `commit_message` (string): Template for git commit messages. Supports placeholders:
  - `{id}`: Issue ID
  - `{title}`: Issue title
  - `{action}`: Command that triggered the commit (`new`, `edit description`, `set <fields>`, `links updated`)
- `list_columns` (list of strings): Default columns shown in `list` command
- `states` (list of strings): Available issue states. The default for new issues is the first element.
- `types` (list of strings): Available issue types. The default for new issues is empty.
- `relationships` (object): Available relationships between issues
- `export_csv_separator` (char): Separator for CSV file exports
- `id_generation` (string): ID generation strategy. Supports options:
  - `sequential`: Sequential numbers (1, 2, 3, ...)
  - `timestamp`: Timestamps in seconds since 2025-01-01 (in teams this reduces the chance of merge conflicts)
- `priority_default`: (string): Default priority for new issues.

### 3.2) users.yaml

This file holds the available users in the project. It should be version-controlled.

```yaml
users:
  - id: alice
  - id: bob
  - id: carol
```

### 3.3) settings.yaml

This file holds the local user settings. It should **not** be version-controlled.

```yaml
# Editor to edit/show issue descriptions
# git = use the git-configured editor
editor: git

# User name
# Used as default reporter for new issues
user: alice
```

#### 3.3.1) Options

- `editor` (string): External text editor (set `git` to use configured git core.editor)
- `user` (string): User name, used per default as reporter for new issues (can be '')

### 3.4) description.md

This file holds the template for the issue descriptions. It is use when a new issue is created with `git issue new`.

```md
# Description

# Repro Steps

# Expected Behavior

# System Info

```

## 4.) Storage Layout

This is the directory structure of `.gitissues/`:

```
.gitissues/
├── .tmp/           # Temporary files (put in `.gitignore`)
├── config.yaml     # Project configuration
├── description.md  # Description template
├── users.yaml      # Available users
├── settings.yaml   # Local user settings (put in `.gitignore`)
├── exports/        # Location of CSV exports (put in `.gitignore`)
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

### 4.1) meta.yaml Format

```yaml
id: 1234                       # (Integer) Identifier
title: Login screen is broken  # (String) Title
state: new                     # (String) E.g.: new, active, resolved, junked
type: bug                      # (String) E.g.: feature, bug, task
labels:                        # (List of Strings) Labels / tags
  - software
  - ui
reporter: t.burkard            # (String) Who reported the issue
assignee: j.doe                # (String) To whom the issue is assigned
priority: P2                   # (Enum) Priority: P0 = highest, P4 = lowest
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

Don't edit these files manually. Instead use the `git issue set` and `git issue link` commands.

## 5.) Development

### 5.1) Building & Testing

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

### 5.2) Architecture

- `config/`
  - `config-default.yaml`    -- Default configuration, copy-pasted at `git issue init` to `.gitissues/`
  - `description-default.md` -- Default description template, copy-pasted at `git issue init` to `.gitissues/`
  - `users-default.yaml`     -- Default users, copy-pasted at `git issue init` to `.gitissues/`
  - `settings-default.yaml`  -- Default local user settings, copy-pasted at `git issue init` to `.gitissues/`
- `src/`
  - `main.rs`  -- CLI parsing with clap
  - `model.rs` -- Shared data types, functions and utilities
  - `edit.rs`  -- Edit issue description (markdown) with external text editor
  - `init.rs`  -- Initialize `.gitissues/` directory and copy default config
  - `link.rs`  -- Change relationships between issues
  - `list.rs`  -- List all issues
  - `new.rs`   -- Create new issues
  - `set.rs`   -- Change issue meta fields
  - `show.rs`  -- Show all issue information (markdown) with external text editor
- `tests/`     -- Automated tests

### 5.3) Dependencies

- `clap`        -- CLI argument parsing
- `chrono`      -- Timestamp generation
- `serde`       -- Serialization framework
- `serde_yaml`  -- YAML parsing for meta.yaml files
- `shell-words` -- Process command line according to parsing rules of Unix shell
- `regex`       -- Regular expressions
- `indexmap`    -- Provides IndexMap datatype
