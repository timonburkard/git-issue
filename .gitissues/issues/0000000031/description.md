# Description

To enable team workflow, we need a way to distinguish between project configurations, which are version-controlled
 --> existing `config.yaml` file

And local settings, which are per-user settings, which are not version controlled.
 --> new `settings.yaml` file

The local settings should include the `editor`: We need to move it from `config.yaml` to `settings.yaml`.

In the future, new local settings can be added, e.g., the default username to be used to fill the `reporter` meta field

# Repro Steps

# Expected Behavior

# System Info
