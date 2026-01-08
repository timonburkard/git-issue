# Description

So far we can only edit the meta field `state` by the following command:

```
git issue state <id> <new_state`
git issue state 1234 resolved
```

We need similar concept for the other editable meta fields, which are all the fields expect for `created` and `updated`.

# Repro Steps

# Expected Behavior

# System Info

# Comments

**t.burkard, 2025-12-22T20:57:42Z**

> Lets add a more general command `set` which then takes options for the different fields:
>
> E.g.:
>
> ```
> git issue set 1234 --state resolved
> git issue set 1234 --type bug
> git issue set 1234 --title "LCD driver has a problem"
> git issue set 1234 --assignee "t.burkard"
> git issue set 1234 --labels cli,driver
> git issue set 1234 --state resolved --type bug --title "LCD driver has a problem" --assignee "t.burkard" --labels cli,driver
> ```
>
> For labels, this command will add labels which not yet exist (prevent duplicates); not overwrite existing label. To remove existing label, another command will be added in the future.
