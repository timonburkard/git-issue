# Description

For metadata fields which can have multiple values (e.g., `labels`, `relationships`) it needs to be defined what kind of filter is applied if comma-separated values are provided as a filter.

E.g.

```
git issue list --filter labels=ui,cli
```

- Should it only list issues which have both labels: 'ui' AND `cli`?
- Or should it list all issues which have any of the labels: 'ui' OR 'cli'?

Note, for fields which have a single value (e.g., `state`, `type`, `assignee`) it is clear that only OR makes sense --> see #38

# Repro Steps

# Expected Behavior

# System Info

# Comments

**t.burkard, 2026-01-04T16:02:04Z**

> It makes the most sense to follow the same approach as for single value fields: comma-separated filter values are always treaded as OR operations.
>
> So, this:
>
> ```
> git issue list --filter labels=ui,cli
> ```
>
> Should list all issues which have any of the labels: 'ui' OR 'cli'.
>
> Reason: If the user wants AND operation, he can simply add an additional filter field, e.g.: `git issue list --filter labels=ui labels=cli` -- which will return only the issues which have both labels: 'ui' AND 'cli'
