# Description

| **Field**    | **Version** |
| ------------ | ----------- |
| Found in:    | v0.6.2      |
| Resolved in: | v0.6.3      |

It seems that OR filter for `id` and `priority` metadata field does not work for `list --filter` command.

```
$ git issue list --filter id=1,2
Error: ID must be an integer
```

```
$ git issue list --filter priority=P1,P2
Error: Invalid priority value
```

Note, for the other metadata fields the OR filter works correctly, e.g.:

```
$ git issue list --filter state=new,active
id  state   assignee   title
5   new     r.federer  show dancing unicorn while application is loading
4   new     -          check if update to latest ubunto makes sense
3   active  j.doe      segmentation fault
2   new     -          possibility to show the logo on the screen
```


# Repro Steps

# Expected Behavior

# System Info
