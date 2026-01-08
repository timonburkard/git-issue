# Description

| **Field**    | **Version** |
| ------------ | ----------- |
| Found in:    | v0.6.1      |
| Resolved in: | v0.6.2      |

---

If the user runs this command:

```
git issue new "title" --labels ''
```

Then the `meta.yaml` contains:

```yaml
labels:
- ''
```

--> Which is not correct.

It should look like this:

```yaml
labels: []
```

# Repro Steps

# Expected Behavior

# System Info
