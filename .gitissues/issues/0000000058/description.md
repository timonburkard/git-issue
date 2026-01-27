# Description

This is about this part of the `config.yaml`:

```yml
# Separator used when exporting to CSV
export_csv_separator: ','
```

I would consider it a local setting, every user who wants to do a CSV export, should decide which CSV separator he wants to use. I don't think it should be a version-controlled project-configuration.

Therefore, lets move `export_csv_separator` from `config.yaml` to `settings.yaml`.

# Repro Steps

# Expected Behavior

# System Info

v0.7.2
