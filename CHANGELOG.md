# Changelog 🗒️

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.1.0] - 2025-12-21

### Added

    - [cmd]    `git issue init` - Initialize `.gitissues/` directory structure with default configuration
    - [cmd]    `git issue new <title>` - Create new issues with auto-incremented numeric IDs
    - [cmd]    `git issue list` - List all issues in tabular format (ID, State, Title)
    - [cmd]    `git issue show <id>` - Display full issue details (metadata + markdown description)
    - [cmd]    `git issue state <id> <state>` - Change issue state and update timestamp
    - [git]    Git auto-commit - Automatically commit `.gitissues/` changes after `new` and `state` commands
    - [config] Configuration file (`.gitissues/config.yaml`)
    - [db]     YAML metadata storage - Issues stored in `.gitissues/issues/{ID}/` with structured metadata
    - [db]     Markdown descriptions - Each issue includes an editable `issue.md` file
    - [cli]    CLI parsing - Full-featured argument parsing with `clap` derive macros
    - [ci]     GitHub Actions workflow - Automated builds, linting, and tests on push/PR
