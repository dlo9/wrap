# wrap
A small command to wrap or "bookmark" other commands. Like `alias`, but better

# Usage
Given the following file in `$HOME/.config/wrap.yaml` (other formats are also supported):
```yaml
wrappers:
  Run a shell command:
    trigger: ["sh"]
    command: sh
    args:
      - -c
  Echo something:
    trigger: ["echo"]
    command: echo
    args: []
```

`wrap` has the following behavior:
```sh
# Trigger "Echo something"
$ wrap echo hi
hi

# Trigger "Run a shell command"
$ wrap sh -c "echo hi"
hi

# Missing trigger
$ wrap -n ls .
Error: No trigger found: ["ls", "."]

# Dry-run
$ wrap -n echo hi
echo "hi"
```