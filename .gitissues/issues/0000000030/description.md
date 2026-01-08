# Description

It's important to know who reported/created an issue.

Theoretically, as everything is in git, one could figure it out using `git blame`, but I think it is better to have a field for it.

Question is on which values to allow? Same as for `assignee`, i.e., needs to be in the `users.yaml`?

What when a new issue gets created? We need a way to save the local username -- this cannot be part of the `config.yaml` because this file is version controlled.

# Repro Steps

# Expected Behavior

# System Info

# Comments

**t.burkard, 2025-12-27T15:09:01Z**

> > Question is on which values to allow? Same as for assignee, i.e., needs to be in the users.yaml?
>
> yes
>
> > What when a new issue gets created? We need a way to save the local username -- this cannot be part of the config.yaml because this file is version controlled.
>
> #31 introduced a new concept of `settings.yaml` which are non-version-controlled user settings. We can add a `user` field there. It can be taken by default when creating a new issue and it can be overwritten with `--reporter <reporter>`
>
