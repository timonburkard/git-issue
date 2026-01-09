# Description

Currently:

```
$ git issue list --filter id\>40
id  state     assignee   title
49  new       -          [cmd/list] formatting: bold headers
48  new       -          [cli] colorized output
47  new       -          [test] add automated tests for git commit
46  new       -          [test] add automated tests for show command
45  new       -          [test] add automated tests for edit command
44  resolved  t.burkard  [cmd/list] bug: OR filter for `id` and `priority` does not work
43  resolved  t.burkard  [cmd/list] harmonize sorting of empty values
42  resolved  t.burkard  [cmd/new] bug: explicitly empty labels not handled correctly
41  new       -          [cmd] do we need a command to show the history of an issue ?
```

Suggestion to change to:

```
$ git issue list --filter id\>40
id  state     assignee   title
------------------------------------------------------------------------------------------
49  new       -          [cmd/list] formatting: bold headers
48  new       -          [cli] colorized output
47  new       -          [test] add automated tests for git commit
46  new       -          [test] add automated tests for show command
45  new       -          [test] add automated tests for edit command
44  resolved  t.burkard  [cmd/list] bug: OR filter for `id` and `priority` does not work
43  resolved  t.burkard  [cmd/list] harmonize sorting of empty values
42  resolved  t.burkard  [cmd/new] bug: explicitly empty labels not handled correctly
41  new       -          [cmd] do we need a command to show the history of an issue ?
```

# Repro Steps

# Expected Behavior

# System Info
