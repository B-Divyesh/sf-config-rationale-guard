# Config Rationale Guard

Config Rationale Guard is an offline CLI for maintainers of agent, CI, and
developer-tool settings. It keeps a separate rationale record beside strict
JSON, YAML, or TOML configuration, then checks the record before CI runs.

It does not rewrite the source config. Reports show paths, rationale, and
status without printing configuration values.

## Try the sample

Run the shipped sample with no account or prompts. It creates a new temporary
folder, writes a config, adjacent rationale file, and schema, then runs a
passing check.

```sh
crg demo
```

Use a new or empty folder when you want to keep the sample:

```sh
crg demo --output ./crg-sample
```

The browser version of this sample is at
https://config-rationale-guard.sociobot.in/demo. It uses `demo:` local storage
only; Reset demo and Start for real discard its sample edits.

## Install

Build from source with Rust 1.85 or later:

```sh
cargo install --path cli
crg --help
```

The CLI is MIT licensed. It runs locally and does not require an account.

## Use in a repository

Initialize an adjacent rationale file for `agent.json`:

```sh
crg init agent.json
```

Replace each generated `TODO` rationale, then stamp the reviewed values:

```sh
crg stamp agent.json
crg check agent.json --schema agent.schema.json
```

Add a coverage rule when a path needs a rationale. `*` matches one JSON Pointer
segment and `**` matches the rest.

```json
{
  "version": 1,
  "rules": [{ "pattern": "/permissions/**", "minimumCoverage": 1.0 }],
  "decisions": [{
    "path": "/permissions/shell",
    "rationale": "Release automation needs the signed packaging script.",
    "policy": "SEC-12",
    "owner": "platform",
    "reviewBy": "2027-01-31",
    "valueHash": "sha256:..."
  }]
}
```

`check` finds missing, orphaned, overdue, schema-invalid, and stale decision
records. The command exits `0` when valid, `1` for findings, and `2` when it
cannot read a command input. Use `--json` for one scriptable report on stdout.

Compare two revisions with a value-free decision diff:

```sh
crg diff old/agent.json agent.json --json > decision-report.json
```

## Format and schema support

The CLI parses JSON, YAML, and TOML into one data model. JSON Schema validates
all three formats through `--schema`. Format-specific schema languages are not
supported.

## Privacy

The CLI runs locally and never rewrites source configuration. The browser demo
makes only same-origin requests during its sample flow. See the live [privacy policy](https://config-rationale-guard.sociobot.in/privacy/)
and [terms](https://config-rationale-guard.sociobot.in/terms/).

## Develop, test, and package

From a clean checkout:

```sh
npm ci
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm test
npm run build
cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty
```

`npm test` runs the Rust unit and integration tests, TypeScript tests, and
browser claim tests. Every public product claim appears in
[.factory/claims.json](.factory/claims.json) with its exact standalone test
command. `npm run build` writes the release binary to `dist/bin/crg` and the
static site to `dist/site`. To exercise the packed consumer artifact, extract
the package and run:

```sh
cargo install --path . --root /tmp/crg-consumer --locked
/tmp/crg-consumer/bin/crg demo
```

No registry publish is performed from this repository. See
[CHANGELOG.md](CHANGELOG.md), the [visual thesis](.factory/design.md), and the
live documentation site.
