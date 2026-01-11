# Description

There is a flaw in the workflow when a repo, which already uses `git-issue` for issue tracking, gets cloned.

When `git-issue` gets initially added to a repo with `git issue init`, the local user settings.yaml gets created. This is a non-version controlled file because each user of a team should be able to make individual changes to it. So far so good.

However, if then the repo gets cloned again, it will not contain the settings.yaml file and there is no command to create it. The user would then need to manually create the file based on the example given in the README.md -- this is not good.

# Repro Steps

1. Initialize issue tracking in a repo: `git issue init`
2. Push changes to main branch
3. Clone the repo again (in another directory)
4. Try to run any `git-issue` command
   1. You'll get `Error: settings.yaml not found.`

# Expected Behavior

There should be a way to locally initialize a repo, which is already using `git-issue` for issue tracking.

E.g., by a new switch: `git issue init --settings`

# System Info

v0.6.4
