# Description

Currently, as priority is this enum:

```rs
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ValueEnum)]
pub enum Priority {
    // clap default to lower case, so add aliases for upper case too
    #[value(alias = "P0")]
    P0,
    #[value(alias = "P1")]
    P1,
    #[value(alias = "P2")]
    P2,
    #[value(alias = "P3")]
    P3,
    #[value(alias = "P4")]
    P4,
}
```

It is not possible for an issue to have empty priority. Maybe, if someone doesn't want to use priorities, it would be nice if we could configure default priority to be empty?

# Repro Steps

# Expected Behavior

# System Info

# Comments

**t.burkard, 2025-12-29T01:17:58Z**

> Yes, lets implement this; field should accept empty value: `git issue set <id> --priority ''`
>
> Also, lets add a configuration for `priority_default` in `config.yaml`
