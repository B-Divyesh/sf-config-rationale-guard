# Config Rationale Guard CLI

`crg` keeps human decision records adjacent to strict JSON, YAML, and TOML.
It runs offline, emits no telemetry, never rewrites config, and omits config
values from its reports.

```sh
crg init agent.json
# replace TODO entries in agent.json.rationale.json
crg stamp agent.json
crg check agent.json --schema agent.schema.json
crg diff old/agent.json agent.json --json
```

See the [project repository](https://github.com/B-Divyesh/sf-config-rationale-guard)
for the sidecar format, exit codes, and complete documentation.
