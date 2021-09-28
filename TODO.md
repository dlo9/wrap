- Set logging level in config file (not important since there are no logs right now?)
- Support more shells for auto-install
- Add config to add default argument at end instead of beginning
- Github actions CI
- Github actions binary releases
- Better examples
- Rename to alias++?
- Print available aliases with `--aliases` or `--help`?
- Support per-alias help flag:
	- --alias-help
	- `description` in yaml for each alias, keyword, etc.
- Allow positional arguments with $1, $2, $@ etc.
  If not used, then will be added to the end
  Possibly want a template engine for this?
- Allow specifying where a default arg should be, so that a flag can be at the end of the default run string
  and can easily have something passed to it.
  Right now this doesn't play nicely with overrides
  Could possibly merge `keywords` and `overrides` fields with this?
- Allow empty/missing `cleared-by`
- Rename `arguments` -> `defaults` and `keywords` -> `overrides`
- Tests
- Test & example for YAML anchors
- Don't print unnecessary quotes with --dry-run

Bugs:
- overriding HOME at runtime doesn't work? -> downstream bug in `config-rs`, need to update version
