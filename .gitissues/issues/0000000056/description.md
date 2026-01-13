# Description

If an issue has an empty relationship category, but not empty relationships, e.g.:

```yaml
relationships:
  related: []
```

Then the `show` command does not parse the metadata field table correctly/consistently for relationships row.

This is how this example looks:

```
## Meta Data

...
| **relationships** | related:  |
...
```

Instead, it should show:

```
...
## Meta Data

| **relationships** | - |
...
```

Note, it works correctly if the `relationships` metafield in YAML is completely empty: `{}`.

Not sure this is actually something that needs to be fixed in the `show` command? -- Maybe, instead, we should ensure that YAML fields never end up in this state?

# Repro Steps

# Expected Behavior

# System Info

v0.6.6
