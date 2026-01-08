# Description

Would me nice I think.

However, as the user didn't hat the change to configure `commit_auto`, maybe he doesn't want it?

Not sure...

Or make it create a commit by default and add optional `--no-commit` flag?

However, it is just a single action, so maybe OK that the user cannot disable this commit message?

TBD...

# Repro Steps

# Expected Behavior

# System Info

# Comments

**t.burkard, 2025-12-23T12:26:10Z**

> Lets add a commit message for the init

**t.burkard, 2025-12-23T12:55:06Z**

> Make it create the commit message by default. But provide possibility to disable it via option:
> 
> ```
> git issue init
> git issue init --no-commit
> ```
