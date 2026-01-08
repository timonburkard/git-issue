# Description

Lets take this example:

```
Timi_@tburkard_lenovo MINGW64 ~/git/example-project (main)
$ git issue list --columns "*"
id  title                                              state     type     labels      reporter  assignee   priority  due_date    related  parent  child  created               updated
5   show dancing unicorn while application is loading  new       feature  ui          -         r.federer  P4        2042-12-31  1        -       -      2025-12-23T21:14:45Z  2025-12-26T17:43:18Z
4   check if update to latest ubunto makes sense       new       -        -           -         -          P4        -           1        -       -      2025-12-22T21:37:06Z  2025-12-26T17:43:18Z
3   segmentation fault                                 active    bug      memsafety   alice     j.doe      P1        -           1        -       -      2025-12-22T21:29:33Z  2025-12-27T14:28:36Z
2   possibility to show the logo on the screen         new       feature  driver,gui  -         -          P2        2026-01-30  1        -       -      2025-12-22T21:23:41Z  2025-12-26T17:38:29Z
1   bug in lcd driver                                  resolved  bug      driver,log  -         t.burkard  P2        -           2,3,4,5  -       -      2025-12-22T21:11:36Z  2025-12-26T17:43:18Z
```

Filtering for empty **string values** works, e.g.:

```
Timi_@tburkard_lenovo MINGW64 ~/git/example-project (main)
$ git issue list --columns "*" --filter type=''
id  title                                         state  type  labels  reporter  assignee  priority  due_date  related  parent  child  created               updated
4   check if update to latest ubunto makes sense  new    -     -       -         -         P4        -         1        -       -      2025-12-22T21:37:06Z  2025-12-26T17:43:18Z
```

However, it does not work for empty **list values**, e.g.:

```
Timi_@tburkard_lenovo MINGW64 ~/git/example-project (main)
$ git issue list --columns "*" --filter labels=''
id  title  state  type  labels  reporter  assignee  priority  due_date  related  parent  child  created  updated
```

--> Should display ID 4, but it doesn't.

Same problem for relationship categories, e.g.:

```
Timi_@tburkard_lenovo MINGW64 ~/git/example-project (main)
$ git issue list --columns "*" --filter parent=''
id  title  state  type  labels  reporter  assignee  priority  due_date  related  parent  child  created  updated
```

# Repro Steps

# Expected Behavior

# System Info

# Comments

**t.burkard, 2025-12-28T22:23:07Z**

> Has been fixed. Now it works.
>
> E.g., for labels:
>
> ```
> Timi_@tburkard_lenovo MINGW64 ~/git/example-project (main)
> $ git issue list --columns "*" --filter labels=''
> id  title                                         state  type  labels  reporter  assignee  priority  due_date  related  parent  child  created               updated
> 4   check if update to latest ubunto makes sense  new    -     -       -         -         P4        -         1        -       -      2025-12-22T21:37:06Z  2025-12-26T17:43:18Z
> ```
>
> And, e.g., for relationships categories:
>
> ```
> Timi_@tburkard_lenovo MINGW64 ~/git/example-project (main)
> $ git issue list --columns "*" --filter parent=''
> id  title                                              state     type     labels      reporter  assignee   priority  due_date    related  parent  child  created               updated
> 5   show dancing unicorn while application is loading  new       feature  ui          -         r.federer  P4        2042-12-31  1        -       -      2025-12-23T21:14:45Z  2025-12-26T17:43:18Z
> 4   check if update to latest ubunto makes sense       new       -        -           -         -          P4        -           1        -       -      2025-12-22T21:37:06Z  2025-12-26T17:43:18Z
> 3   segmentation fault                                 active    bug      memsafety   alice     j.doe      P1        -           1        -       -      2025-12-22T21:29:33Z  2025-12-27T14:28:36Z
> 2   possibility to show the logo on the screen         new       feature  driver,gui  -         -          P2        2026-01-30  1        -       -      2025-12-22T21:23:41Z  2025-12-26T17:38:29Z
> 1   bug in lcd driver                                  resolved  bug      driver,log  -         t.burkard  P2        -           2,3,4,5  -       -      2025-12-22T21:11:36Z  2025-12-26T17:43:18Z
> ```
