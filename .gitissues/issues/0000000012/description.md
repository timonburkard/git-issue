# Description

Currently `git show <id>` doesn't look nice.

E.g.:

```
$ git issue show 1
id:       1
title:    bug in lcd driver
state:    new
type:     -
labels:   -
assignee: -
created:  2025-12-22T16:40:04Z
updated:  2025-12-22T16:40:04Z
description:
# Description

There appears to be a bug in the LCD driver.

I get the following error message, when I want to run the LCD test:

`err: 0xFFFE drive update failure`

# Repro Steps

- flash the debug build
- run lcd_test.txt
- check the log file

# System Info

LCD driver v1.0.3
GUI logger v0.5.4
```

I think this command should open an external text editor (or maybe in the future even an generated PDF/HTML) with all the information about a ticket.


# Repro Steps

# Expected Behavior

# System Info
