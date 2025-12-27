# Changelog 🗒️

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Added

    - [settings] added `settings.yaml` to hold local non-version-controlled user settings (GitHub Issue #65)

### Changed

    - [config] moved `editor` from `config.yaml` to `settings.yaml` (GitHub Issue #65)

## [v0.4.0] - 2025-12-26

### Added

    - [cmd/list] added `--csv` option (GitHub Issue #57)
    - [cmd/list] added `--sort` option (GitHub Issue #7)
    - [cmd/list] added `--filter` option (GitHub Issue #6)
    - [cmd] added `link` command to change relationships between issues (GitHub Issue #29)
    - [cli] accept `due-date` and `due_date` for `list` columns, `new` and `set` commands
    - [test] added more checks for `basic_workflow` and more tests for `set` command
    - [meta] added configurable list of available options for `state`, `type` and `assignee` (users) (GitHub Issue #36)
    - [ci] added automated integration tests

## [v0.3.0] - 2025-12-23

### Added

    - [cmd/new] added option to provide initial values for other meta fields (GitHub Issue #43)
    - [meta] added field `due_date` (GitHub Issue #31)
    - [meta] added field `priority` (GitHub Issue #35)
    - [cmd/new] automatically create the `attachments/` directory (GitHub Issue #33)
    - [config] made default columns for `list` command configurable as `list_columns` in `config.yaml` (GitHub Issue #34)
    - [cmd] added `--version` / `-V` command to print the version of git-issue
    - [cmd/set] added option `--labels` to overwrite all labels (GitHub Issue #26)
    - [cmd/set] added option `--labels-remove` to remove specific labels (GitHub Issue #26)

### Changed

    - [cmd/list] included 'assignee' in default columns
    - [git] commit does not silently fail anymore
    - [cmd/init] create a commit message by default, can be disabled by option `--no-commit` (GitHub Issue #23)
    - [cmd/set] renamed option `--labels` to `--labels-add`, which added specific labels (GitHub Issue #26)

### Removed

    - [cmd] removed `remove` command, replaced with `set --labels-remove` (GitHub Issue #26)

## [v0.2.0] - 2025-12-23

### Added

    - [cmd] added `remove` command to remove elements from issue meta fields of type list, currently only for labels (GitHub Issue #22)
    - [cmd] added `set` command to change all editable issue meta fields (GitHub Issue #18)
    - [cmd] added `edit` command to edit issue descriptions (markdown) with external text editor (GitHub Issue #8)
    - [config] added `.gitissues/description.md` used as template when new issue is created (GitHub Issue #14)
    - [meta] added fields `type`, `labels` and `assignee` (GitHub Issue #4, #5, #10)
    - [cmd/list] added option `--column` (GitHub Issue #12)

### Changed

    - [cmd/show] improved representation by using external editor and markdown format (GitHub Issue #19)
    - [cmd/list] renamed option `--column` to `--columns`
    - [structure] renamed `issue.md` to `description.md`

### Removed

    - [cmd] removed `state` command, replaced with `set` command (GitHub Issue #18)

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
