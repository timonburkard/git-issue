# Description

Everything is in git, so user could figure out (`git log` / `git blame`) himself how an issue evolved.

But might be nice to add a dedicated `git issue history <id>` command?

Not sure how to implement though:

## Approach A) Dedicated history file per issue

Keep a dedicated history file per issue where all update operations are logged

- Each issue has `.gitissues/issues/<id>/history.log`
- Append timestamped entries on every change
- Format: `2025-12-29T15:30:00Z alice changed state: new --> active`

## Approach B) Parse the git history

### B.1) Simply provide the output from a "nice" git log, e.g.:

```
git log --oneline --pretty="| %an | %ad | %s" --date=short -- .gitissues/issues/0000000001
| Timon Burkard | 2026-01-04 | [issue] set state #1 -- bug in lcd driver
| Timon Burkard | 2025-12-26 | [issue] links updated #1 -- bug in lcd driver
| Timon Burkard | 2025-12-26 | [issue] links updated #1 -- bug in lcd driver
| Timon Burkard | 2025-12-26 | [issue] links updated #1 -- bug in lcd driver
| Timon Burkard | 2025-12-22 | [issue] set state #1 -- bug in lcd driver
| Timon Burkard | 2025-12-22 | [issue] set state #1 -- bug in lcd driver
| Timon Burkard | 2025-12-22 | [issue] set labels #1 -- bug in lcd driver
| Timon Burkard | 2025-12-22 | [issue] set type,assignee #1 -- bug in lcd driver
| Timon Burkard | 2025-12-22 | [issue] edit description #1 -- bug in lcd driver
| Timon Burkard | 2025-12-22 | [issue] new #1 -- bug in lcd driver
```

#### B.1.1) To also see what the actual value changes of the metadata fields are, the commit messages would need to be extended.

### B.2) Parse through `git diff`

If we don't want to include more info in the commit messages, we could parse the output from `git diff` to figure out what the actual value changes of the metadata fields were.

# Repro Steps

# Expected Behavior

# System Info
