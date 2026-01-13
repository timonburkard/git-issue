# Description

Currently the `git issue list --filter` command can only filter on metadata fields.

Would be nice if we could also filter/search through the `description.md`:

```
git issue list --filter description=<pattern>
```

Note similar issue #10, which suggests a new command `search` already exists. However the current issue is about extending the filter capability of the `list` command. #10 is more like a completely new `grep` feature.

# Repro Steps

# Expected Behavior

# System Info

v0.6.6
