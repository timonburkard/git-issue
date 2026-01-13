# Description

The `show` command not only displays the `description.md` but prior to that also shows the metadata fields in a markdown table.

E.g.:

```md
<!-- READ-ONLY VIEW -->

# Issue #1 -- [meta] add field `assignee`

## Meta Data

| **field**         | **value** |
| ----------------- | --------- |
| **id**            | 1 |
| **title**         | [meta] add field `assignee` |
| **state**         | resolved |
| **type**          | feature |
| **labels**        | - |
| **reporter**      | t.burkard |
| **assignee**      | t.burkard |
| **priority**      | - |
| **due_date**      | - |
| **relationships** | related:  |
| **created**       | 2025-12-22T12:43:37Z |
| **updated**       | 2025-12-22T14:23:47Z |

## Description

...
```

It looks OK, but not perfect. Ideally, it would look like this:

```md
<!-- READ-ONLY VIEW -->

# Issue #1 -- [meta] add field `assignee`

## Meta Data

| **field**         | **value**                   |
| ----------------- | --------------------------- |
| **id**            | 1                           |
| **title**         | [meta] add field `assignee` |
| **state**         | resolved                    |
| **type**          | feature                     |
| **labels**        | -                           |
| **reporter**      | t.burkard                   |
| **assignee**      | t.burkard                   |
| **priority**      | -                           |
| **due_date**      | -                           |
| **relationships** | related:                    |
| **created**       | 2025-12-22T12:43:37Z        |
| **updated**       | 2025-12-22T14:23:47Z        |

## Description
...
```

# Repro Steps

# Expected Behavior

# System Info

v0.6.6
