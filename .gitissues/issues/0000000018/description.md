# Description

Issues are currently unrelated.

Usually, issues can have relationships. E.g.,
- `related`
- `duplication_of` + `duplicates`
- `blocks` + `blocked_by`

As a first simple approach, we could maybe start with a general `related` field?

# Repro Steps

# Expected Behavior

# System Info

# Comments

**t.burkard, 2025-12-25T12:42:23Z**

> Makes sense to have an object which holds all relationships (future-proof)
>
> ```yaml
> relationships:
>   related: [2, 5, 8]
>   parent: [13]
>   child: [33, 77]
>   references: [45]
> ```
>
> As a start, lets just implement this:
>
> ```yaml
> relationships:
>   related: [2, 5, 8]
> ```
>
> Makes sense to have it automatically bidirectional, I think.

**t.burkard, 2025-12-26T12:14:47Z**

> It might make sense to implement this configureable (in `config.yaml`) from the beginning.
>
> This is how the `config.yaml` could look:
>
> ```yaml
> relationships:
>   related:
>     link: related        # bidirectional, symmetric
>   parent:
>     link: child          # bidirectional, asymmetric
>   child:
>     link: parent         # bidirectional, asymmetric
>   references:
>     link: null           # unidirectional
> ```
