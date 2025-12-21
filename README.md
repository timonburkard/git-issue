# git-issue 🗂️

A Git-native, file-backed issue tracker.

Issues live alongside your code inside `.gitissues/`, making them platform-independent, version-controlled, branchable, mergeable, reviewable and offline-friendly.

## Features

- ✅ `git issue init` -- Initialize `.gitissues/` in your repository
- ✅ `git issue new <"title">` -- Create a new issue with auto-incremented ID
- 🚧 `git issue list` -- List all issues
- 🚧 `git issue show <id>` -- Display issue details
- 🚧 `git issue state <id> <state>` -- Change issue state
- 🚧 Git integration -- Auto-commit `.gitissues/` changes
- 🚧 Testing

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

-  `meta.yaml` (metadata: id, title, status, timestamps)
- `issue.md` (human-readable markdown description)
- IDs are 10-digit zero-padded (0000000001, 0000000002, …)

### meta.yaml Format

```yaml
id: 0000000001
title: "Fix login bug"
status: new
created: 2025-12-21T15:54:52Z
updated: 2025-12-21T15:54:52Z
```

### issue.md Format

```markdown
# Fix login bug

## Description

TBD
```

## Usage

```bash
# Initialize tracking in your repo
git issue init

# Create a new issue
git issue new "Fix login redirect bug"

# List issues
git issue list

# Show issue details
git issue show 0000000001

# Close an issue
git issue state 0000000001 resolved
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

## Architecture

- `src/main.rs` -- CLI parsing with clap
- `src/init.rs` -- Initialize `.gitissues/` directory
- `src/new.rs` -- Create new issues with ID allocation

## Dependencies

- `clap` -- CLI argument parsing
- `chrono` -- Timestamp generation
