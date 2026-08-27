# Verification handoff — Config Rationale Guard v0.1.0

**Work order:** `config-rationale-guard-verify-3`
**Verdict:** **PASS**
**Tested candidate:** `fed7394ce256b785c71d301caec7a6fef8df65d9`
**Live URL:** https://config-rationale-guard.sociobot.in/

Independent QA passed. The live deployment is the exact candidate build:
HTML, emitted JavaScript/CSS, and service worker all match fresh local
production-build SHA-256 hashes byte-for-byte. No defects by severity were
found (critical/high/medium/low: 0/0/0/0).

## What was verified

- Clean install and gates: `npm ci`, `cargo fmt --all -- --check`, `cargo
  clippy --workspace --all-targets -- -D warnings`, `npm test`, exact `npm run
  build`, and `cargo package --manifest-path cli/Cargo.toml --locked
  --allow-dirty` all pass.
- The packed crate was installed to a clean consumer root and its installed
  binary (`crg 0.1.0`) exercised. No registry publishing was attempted.
- Release CLI flow passed: JSON init, reviewed rationale stamping, schema
  check, coverage rule, value-free diff, stale-value failure/recovery, missing
  input, and duplicate-key rejection. Existing integration coverage confirms
  JSON/YAML/TOML behavior and schema-sentinel redaction in both report modes.
- Live desktop and 390px mobile browser checks passed: normal load and
  accessibility semantics, local demo success/malformed/empty/recovery,
  visible keyboard focus, reduced motion, privacy/network behavior, response
  headers/cache policy, and license query-token storage/stripping with mocked
  billing verification.
- Axe WCAG 2 A/AA + 2.1 A/AA found zero violations (zero serious/critical) on
  home, privacy, and terms. Live controlled service-worker offline reload
  displayed the offline notice with no console/page errors or overflow.
- Budgets pass: JS 8,483 B, CSS 16,520 B, font 18,096 B, hero image 160,258 B.
  Local production-preview Lighthouse mobile: Performance 99, Accessibility
  100, Best Practices 100, SEO 92; LCP 2,075 ms, CLS 0, TBT 69 ms.

## Run or reproduce

```sh
npm ci
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm test
npm run build
cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty
```

`npm run build` produces `dist/bin/crg` and deployable static files at
`dist/site`. The factory owns registry and deployment credentials; do not
publish the crate from this repository.

Complete exact evidence is in `.factory/verification-3.md`.

## Known gaps / next steps

None for this candidate. Continue to deploy the exact `dist/site` output so
future verifications retain the checked build identity.
