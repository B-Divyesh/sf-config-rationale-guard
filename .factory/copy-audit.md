# Landing copy audit

Audited 5 September 2026. Word counts exclude code identifiers, file names, and
navigation labels. No audited sentence exceeds 22 words. The banned-word scan
found no occurrences of leverage, seamless, effortless, robust, powerful,
intuitive, reimagine, supercharge, unlock, delightful, journey, ecosystem, or
AI-powered.

| Landing text | Words | Result |
| --- | ---: | --- |
| Keep config decisions reviewable in CI. | 6 | Pass |
| For maintainers of agent, CI, and tool settings who need every policy change explained before it ships. | 17 | Pass |
| Opens a temporary sample project. | 5 | Pass |
| Config and its reason are reviewed together. | 7 | Pass |
| Run the included sample. | 4 | Pass |
| `crg demo` writes a separate sample folder, then checks its adjacent rationale file and schema. | 14 | Pass |
| Add a rationale next to each decision. | 7 | Pass |
| The source config remains unchanged. | 5 | Pass |
| The CLI writes and checks a separate JSON rationale file beside it. | 12 | Pass |
| Generate one adjacent decision record for each config leaf. | 9 | Pass |
| Add the reason, then fingerprint the reviewed setting. | 8 | Pass |
| Find missing, orphaned, overdue, schema-invalid, or stale decision records. | 8 | Pass |
| Keep values out of review reports. | 6 | Pass |
| Reports show paths, rationale, and status. | 6 | Pass |
| They do not print configuration values. | 6 | Pass |
| The demo sends no input away from this site. | 9 | Pass |
| It does not rewrite source configuration or require a cloud service. | 10 | Pass |
| Build the CLI and run a check. | 8 | Pass |
| Use Rust 1.85 or later. | 6 | Pass |
| The sample command works without prompts, and `--json` produces a scriptable report. | 11 | Pass |

## Terminology

| Concept | One term used |
| --- | --- |
| Settings file | config |
| Adjacent metadata file | rationale file |
| Individual explanation record | decision record |
| Shipped try-out data | sample |
| Automated rule execution | check |
| Changed reviewed value marker | fingerprint |
