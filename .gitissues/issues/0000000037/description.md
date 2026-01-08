# Description

E.g. how some error messages currently look:

```
$ git issue set 1 --state resolved
Error: Invalid state: Check config.yaml:states
```

Better would be something like:

```
$ git issue set 1 --state resolved
Error: Invalid state 'resolved', valid options: 'new', 'active', 'closed', 'deleted'
```

# Repro Steps

# Expected Behavior

# System Info
