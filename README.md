# PHP dead-code scanner

This is a Rust learning project, not meant for production use.

## Goal

A fast CLI that finds probably unused PHP files/classes in a project.
No full PHP parser — just pragmatic heuristics. Might expand if this turns out to be fun.

## Scope

### What it does

* Scan `src/` (configurable)
* Detect:
  * PHP files never referenced
  * Classes never instantiated / extended
* Output:
  * human table
  * optional --json

### What it explicitly does NOT do

* No runtime analysis
* No full AST
* No Composer autoload resolution v1 (can be v2)

## CLI UX

```shell
php-deadscan scan src \
  --ignore vendor \
  --ignore tests \
  --json
```

Flags:

* `--ignore <path>` (repeatable)
* `--only-unused`
* `--fail-on-unused` (CI use)
* `--entry index.php,bin/console`
* `--json` (output format)