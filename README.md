# wrap
A small utility to wrap other commands. Like `alias`, but better.

Unlike `alias`, it supports:
- default arguments (which can be overridden at runtime)
- argument aliases
- global dry-run (no more explanations of "I aliases `k` to `kubectl`" when sharing command snippets!)

# Usage
## Configuration
Wrap uses config files to define aliases. Multiple formats are supported, including `yaml`, `toml`, and `json`.
`yaml` is the recommended format and will be used in all examples. `wrap` reads its configuration in two ways:
1. By default, all config locations are read and marged, with more specific configurations overridding values
   in less specific configurations. Config files can be specified in the following locations and merged in order:
   - Global config: `/etc/wrap.yaml`
   - User config: `~/.config/wrap.yaml`
   - Local config: `./wrap.yaml`
2. If a config file is specified at runtime with `--config <file>`, then the following configs are read and merged:
   - Global config: `/etc/wrap.yaml`
   - Argument-specified file: `<file>`

Here's an example config (which will be used in the rest of the README):
```yaml
variables:
  KUBE_DIR: ~/.kube
  # Variables can contain previously defined variables
  PROD_KUBECONFIG: $KUBE_DIR/prod
  # Or environment variables
  CONFIG_DIR: $HOME/.config

aliases:
  - alias: k
    program: kubectl

    # These are the default arguments
    # Arguments & values can be overridden when invoked
    arguments:
      - key: -n  # Required
        value: default-namespace  # Optional
        cleared-by: [-n, -A]  # If -n or -A is specified, the default argument is ignored

    # Argument `keywords`. If the keyword is found after the alias,
    # it will be replaced with one or more argument replacements.
    # This is not recursively executed
    keywords:
        # The first argument matching a key will be replaced.
        # After the first match, these keys will be completely ignored
      - keys: [--dev, --development]
        # The key will be replaced with these values where the key was found
        values: [--context, my_development_context]

      - keys: [--prod]
        values: [--kubeconfig, $PROD_KUBECONFIG]

      - keys: [--kafka]
        values: [-n, kafka-namespace]
```

## Running
Given the config file above, here are some common usage patterns:
```sh
# Run wrap directly, printing the resulting command instead of executing it
$ wrap k --prod --dry-run
kubectl "-n" "default-namespace" "--kubeconfig" "/home/user/.kube/prod"

# Install aliases (requires a shell restart)
$ wrap --alias

# Now we can invoke aliases directly
$ k --dev -n my-namespace --dry-run
kubectl "--context" "my_development_context" "-n" "my-namespace"

# Override variables at runtime
$ KUBE_DIR=/root/.kube k --prod --dry-run
kubectl "-n" "default-namespace" "--kubeconfig" "/root/.kube/prod"

# Uninstall aliases (requires a shell restart)
$ wrap --unalias
```
