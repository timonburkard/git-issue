# Description

This is currently supported by `new` and `set` command:

```
git issue new <title> --reporter me --assignee me
git issue set <id> --reporter me --assignee me
```

'me' will be automatically replaced with the current user (`settings.yaml:user`).

Would be nice to have the same possibility for `list --filter` as well:

```
git issue list --filter reporter=me assignee=me
```

--> Currently doesn't work

# Repro Steps

See description

# Expected Behavior

Same behavior of 'me' for `list --filter` as for `new` and `set` command

# System Info

v0.7.0
