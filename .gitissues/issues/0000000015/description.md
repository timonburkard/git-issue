# Description

Instead of what we have now:

```
git issue set 1234 --labels cli,driver     # Adds labels
git issue remove 1234 --labels cli,driver  # Removes labels
```

We could get rid of the `remove` command and use additional options within the `set` command:

```
git issue set 1234 --labels cli,driver         # Sets labels (overwriting)
git issue set 1234 --labels-add cli,driver     # Adds labels
git issue set 1234 --labels-remove cli,driver  # Remove labels
```

# Repro Steps

# Expected Behavior

# System Info
