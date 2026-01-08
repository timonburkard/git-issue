# Description

For metadata fields which have a single value (e.g., `state`, `type`, `assignee`) it would be nice to have the possibility to OR filter for options.

E.g.

```
git issue list --filter state=new,active
```

Would list all issues with state new OR active.

Note, not sure how this should be handled for multi-value fields (e.g., `labels`, `relationships`) --> See #39

# Repro Steps

# Expected Behavior

# System Info
