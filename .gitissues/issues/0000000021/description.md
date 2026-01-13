# Description

When we make a commit, we should be more careful about what we stage (`git add`)

# Repro Steps

# Expected Behavior

# System Info

# Comments

**t.burkard, 2026-01-13T22:26:00Z**

> Lets not over-engineer this for now.
>
> Simply only stage `.gitissues/issues/`, instead of `gitissues/` when we are editing issues. Only for `init` we need to stage `.gitissues/`. This will prevent that config files get committed unintentionally.
