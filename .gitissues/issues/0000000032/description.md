# Description

## `new`

Note, this is already supported by `new` command:

By default the `reporter` is set to `settings.yaml:user`. Alternatively, the user can overwrite it with `--reporter <reporter>`.

## `set`

Would be nice to have something similar for `set` command:

Maybe possible like this (without an argument value)

```
git issue set <id> --reporter
git issue set <id> --assignee
```

Or with a placeholder:

```
git issue set <id> --reporter me
git issue set <id> --assignee me
```

# Repro Steps

# Expected Behavior

# System Info

# Comments

**t.burkard, 2025-12-27T15:56:18Z**

> Lets use `'me'` placeholder. More intuitive, instead of no value, I think
