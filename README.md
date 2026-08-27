# Config Rationale Guard

Keep the *why* beside strict JSON, YAML, and TOML without putting comment keys
or secrets into the config. Config Rationale Guard is an offline, zero-telemetry
CLI for maintainers of agent, CI, and developer-tool settings. It validates a
JSON Schema, checks adjacent rationale targets and fingerprints, enforces
policy-sensitive coverage, and produces a value-free PR decision report.

## Install

Download a release binary, or build from source with Rust 1.85+:

```sh
cargo install --path cli
crg --help
```

## Usage

Given `agent.json`, initialize its adjacent `agent.json.rationale.json` file:

```sh
crg init agent.json
```

Edit each generated `TODO` rationale, then stamp the reviewed values. The stamp
is a one-way SHA-256 fingerprint; config values never enter reports.

```sh
crg stamp agent.json
crg check agent.json --schema agent.schema.json
```

To require rationale coverage for policy-sensitive paths, add rules to the
sidecar. `*` matches one JSON Pointer segment and `**` matches the rest:

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

Compare two revisions. The text report and `--json` output never contain
configuration values:

```sh
crg diff old/agent.json agent.json
crg diff old/agent.json agent.json --json > decision-report.json
```

Exit code `0` means valid, `1` means policy/validation findings, and `2` means
the command or input could not be read. Every command is non-interactive.

### Format and schema support

JSON, YAML, and TOML are parsed into one JSON data model. Draft 2020-12, draft
7, draft 6, and draft 4 JSON Schemas can validate all three formats through
`--schema`; format-specific schema languages are deliberately unsupported.
Duplicate map keys and non-string YAML keys are rejected. Source config is
never rewritten, so its formatting and comments remain intact. Schema failures
report only their JSON Pointer path and a stable validation message; they never
render the failing configuration value.

## Team unlock

The complete CLI is free and MIT-licensed. A $49 one-time Team unlock on the
documentation site adds convenience policy presets and ready-to-copy CI review
recipes. It never gates validation, reporting, export, privacy, or accessibility.

## Develop, test, and package

```sh
npm install
npm test
npm run build          # CLI release build + site -> dist/site
npm run build:site     # site only -> dist/site
cargo package --manifest-path cli/Cargo.toml --allow-dirty
```

`npm run dev` starts the Vite documentation site. The project has no telemetry,
runtime CDN, or cloud dependency. See [CHANGELOG.md](CHANGELOG.md), the
[visual thesis](.factory/design.md), and the live site at
https://config-rationale-guard.sociobot.in.
