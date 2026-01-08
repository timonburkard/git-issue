# Description

Current the `set` command takes one and only one issue ID:

```
git issue set <id> ...
```

Lets add the possibility to bulk/batch edit the metadata for multiple issues at once:

```
git issue set <id1>[,<idx>]* ...
```

E.g.:

```
git issue set 1234,5555,9876 --assignee t.burkard --state active
```

# Repro Steps

# Expected Behavior

# System Info

# Comments

**t.burkard, 2026-01-04T14:36:24Z**

> Additionally, we could add `git issue set '*' --assignee bob` which will (maybe after confirmation) bulk update all issues from the latest `git issue list` command?

**t.burkard, 2026-01-05T23:06:45Z**

> Yes, lets also have wildcard support based on latest `list` command (incl. confirmation dialog)
