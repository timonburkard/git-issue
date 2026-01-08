# Description

Currently filtering in `list` command only supports `=` operator:

```
git issue list --filter <filter>=<value>
```

This checks for equality, incl. possibility of wildcard (`*`).
Resp. for `labels` and `relationships` it checks if `<value>` is IN the corresponding list.

For numeric fields, it would be nice to be able to filter for a range with `>` and `<`.

This should be implemented for the following meta fields:
- `id`
- `priority`
- `due_date`
- `created`
- `updated`

E.g.:

```
git issue list --filter due_date\>2025-06-01 due_date\<2025-12-31
```

Note, `>` and `<` need to be escaped due to standard shell handling. Alternative:

```
git issue list --filter 'due_date>2025-06-01' 'due_date<2025-12-31'
```

# Repro Steps

# Expected Behavior

# System Info
