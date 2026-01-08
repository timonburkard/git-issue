# Description

Maybe the commands `show` and `edit` could somehow be merged into one?

I can imagine that it is a bit annoying for the user, that when he `shows` an issue and then sees a typo, he wants to change it, but to do so, he first needs to close the editor and open it again with `edit`.

However, `edit` is about the markdown description. `show` is about the hole issue information (incl. meta data) so not sure if it makes sense?

# Repro Steps

# Expected Behavior

# System Info

# Comments

**t.burkard, 2025-12-25T11:17:27Z**

> One idea would be to use front-matter.
>
> E.g.:
>
> ```md
> ---
> id: 3
> title: segmentation fault
> state: active
> type: bug
> labels:
> - memsafety
> assignee: j.doe
> priority: P1
> due_date: ''
> created: 2025-12-22T21:29:33Z
> updated: 2025-12-23T19:17:04Z
> ---
>
> # Description
>
> We can get a segmentation fault if we press the ESC button 10'000 times within 3.8 seconds
>
> This is what the screen looks like:
>
> ![](attachments/segfault.jpg)
>
> I've also attached the log file: [log file](attachments/file.log)
>
> # Repro Steps
>
> - Press ESC button 10'000 times, but you need to be fast <= 3.8 sec
> - Check the screen hehe
>
> # Expected Behavior
>
> Well, memory safty would be nice. Maybe look into using Rust? ;)
>
> # System Info
>
> LCD driver v1.0.3
>
> ```

**t.burkard, 2025-12-26T19:48:58Z**

> So one can edit the meta data as well?
> Invalid metadata, no changes or an empty message aborts the edit?
>
>

**t.burkard, 2025-12-26T19:50:53Z**

> Something like this maybe, yes
