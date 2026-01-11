# Changelog 🗒️

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Added

    - [cli] improved error message for invalid options (#37)
    - [cli] enabled commands to be executed from subdirectories (#51)

### Fixed

    - [git]    print error if trying to commit when not in a git repository
    - [config] automatically create missing settings.yaml file (#53)

## [v0.6.4] - 2026-01-09

### Added

    - [cmd/list] formatting: added support for colored output (#48)
    - [cmd/list] formatting: bold headers (#49)
    - [cmd/list] formatting: added dashed line between headers and values (#50)

## [v0.6.3] - 2026-01-08

### Added

    - [issue] eat your own dog food: started issue-tracking for this repo with `git-issue` instead of GitHub Issues
    - [cargo] MSRV v1.88
    - [test]  added automated tests for `link` command
    - [test]  added automated tests for `set` bulk operations

### Changed

    - [cmd/list] harmonized sorting of empty values: First for asc, last for desc (#43)

### Fixed

    - [cmd/list] fix: OR filter for `id` and `priority` did not work (#44)

## [v0.6.2] - 2026-01-06

### Added

    - [test] added more automated tests for `new`, `set` and `list` commands

### Changed

    - [config] changed default user in `settings.yaml` to empty

### Fixed

    - [cmd/new] fix: explicitly empty labels not handled correctly (#42)

## [v0.6.1] - 2026-01-05

### Added

    - [cmd/set]  added support for bulk operation with list of issue IDs or wildcard (#36)
    - [cmd/list] added support for OR filter (#38, #39)

## [v0.6.0] - 2025-12-29

### Added

    - [meta]     added possibility for `priority` to be empty (#35)
    - [config]   added `priority_default` field (#35)
    - [cmd/list] added possibility to filter for ranges: `>` and `<` (#34)
    - [cmd/list] added support for relationships (#29)
    - [config]   added possibility to change ID generation strategy `id_generation` (#6)

### Fixed

    - [cmd/list] fix: filter for empty `labels` and `relationships` does not work (#33)

## [v0.5.0] - 2025-12-27

### Added

    - [cmd/set]  added shortcut 'me' for `--reporter` and `--assignee`, automatically takes the value from `settings.yaml:user` (#32)
    - [cmd/new]  added shortcut 'me' for `--reporter` and `--assignee`, automatically takes the value from `settings.yaml:user` (#32)
    - [meta]     added `reporter` field (#30)
    - [settings] added `settings.yaml` to hold local non-version-controlled user settings (#31)

### Changed

    - [config] moved `editor` from `config.yaml` to `settings.yaml` (#31)

## [v0.4.0] - 2025-12-26

### Added

    - [cmd/list] added `--csv` option (#28)
    - [cmd/list] added `--sort` option (#4)
    - [cmd/list] added `--filter` option (#3)
    - [cmd]      added `link` command to change relationships between issues (#18)
    - [cli]      accept `due-date` and `due_date` for `list` columns, `new` and `set` commands
    - [test]     added more checks for `basic_workflow` and more tests for `set` command
    - [meta]     added configurable list of available options for `state`, `type` and `assignee` (users) (#25)
    - [ci]       added automated integration tests

## [v0.3.0] - 2025-12-23

### Added

    - [cmd/new] added option to provide initial values for other meta fields (#26)
    - [meta]    added field `due_date` (#20)
    - [meta]    added field `priority` (#25)
    - [cmd/new] automatically create the `attachments/` directory (#22)
    - [config]  made default columns for `list` command configurable as `list_columns` in `config.yaml` (#23)
    - [cmd]     added `--version` / `-V` command to print the version of git-issue
    - [cmd/set] added option `--labels` to overwrite all labels (#15)
    - [cmd/set] added option `--labels-remove` to remove specific labels (#15)

### Changed

    - [cmd/list] included 'assignee' in default columns
    - [git]      commit does not silently fail anymore
    - [cmd/init] create a commit message by default, can be disabled by option `--no-commit` (#14)
    - [cmd/set]  renamed option `--labels` to `--labels-add`, which added specific labels (#15)

### Removed

    - [cmd] removed `remove` command, replaced with `set --labels-remove` (#15)

## [v0.2.0] - 2025-12-23

### Added

    - [cmd]      added `remove` command to remove elements from issue meta fields of type list, currently only for labels (#13)
    - [cmd]      added `set` command to change all editable issue meta fields (#11)
    - [cmd]      added `edit` command to edit issue descriptions (markdown) with external text editor (#5)
    - [config]   added `.gitissues/description.md` used as template when new issue is created (#9)
    - [meta]     added fields `type`, `labels` and `assignee` (#1, #2, #7)
    - [cmd/list] added option `--column` (#8)

### Changed

    - [cmd/show]  improved representation by using external editor and markdown format (#12)
    - [cmd/list]  renamed option `--column` to `--columns`
    - [structure] renamed `issue.md` to `description.md`

### Removed

    - [cmd] removed `state` command, replaced with `set` command (#11)

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
