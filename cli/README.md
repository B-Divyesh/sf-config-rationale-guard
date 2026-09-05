# Config Rationale Guard CLI

`crg` keeps a separate rationale record beside JSON, YAML, and TOML
configuration. Run the bundled sample first:

```sh
crg demo
```

Then initialize, explain, stamp, and check a repository config:

```sh
crg init agent.json
# replace TODO entries in agent.json.rationale.json
crg stamp agent.json
crg check agent.json --schema agent.schema.json
```

Use `crg check agent.json --json` for a machine-readable check report.
See the repository README for the sidecar format, supported behavior, tests,
and privacy details.
