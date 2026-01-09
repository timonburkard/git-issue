# Description

Seems that commands are only executable from the root of the repo:

```
Timi_@tburkard_lenovo MINGW64 ~/git/example-project/src (main)
$ git issue list
Error: config.yaml not found.

Timi_@tburkard_lenovo MINGW64 ~/git/example-project/src (main)
$ cd ..

Timi_@tburkard_lenovo MINGW64 ~/git/example-project (main)
$ git issue list
id  state   assignee   title
--------------------------------------------------------------------------
5   new     r.federer  show dancing unicorn while application is loading
4   new     -          check if update to latest ubunto makes sense
3   active  j.doe      segmentation fault
2   new     -          possibility to show the logo on the screen
1   closed  t.burkard  bug in lcd driver

Timi_@tburkard_lenovo MINGW64 ~/git/example-project (main)
```

Should also be possible from sub-directories, I think. Same as it is for "real" git commands ;)

# Repro Steps

1. Try to execute `git issue list` from a subdirectory
2. Observe error response

# Expected Behavior

- Executable also from sub-directories

# System Info

v0.6.3
