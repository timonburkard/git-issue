# Description

How to sort empty values?

Need to define clear strategy on how empty fields should be sorted with the `list --sort` command.

E.g.:
- Empty fields on top of the list with `asc`
- Empty fields on bottom of the list with `desc`

Or, if easily possible, empty fields always on bottom of the list?

# Repro Steps

# Expected Behavior

# System Info

# Comments

**t.burkard, 2026-01-06T23:09:13Z**

> Currently, for most fields it works that `asc` puts empty first, `desc` puts empties last.
>
> But for `priority` it is the other way around, because of the enum value of Empty being 255.
> And somehow for `relationships` fields is also the other way around.

**t.burkard, 2026-01-06T23:17:12Z**

> Lets go with the classical approach of empty values first for `asc` and last for `desc`.
> Because this is easiest to implement.
>
> In the future we can consider to change to empty fields always last.
