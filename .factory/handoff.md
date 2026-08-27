# Verification status — FAIL

Independent QA on 2026-08-27 rejected candidate
`137d88f54c99f8101ec1dd5efa324f1b152b36aa`. `crg check --schema` leaks raw
configuration values in schema-validation reports, violating the brief's
privacy constraint. The live URL https://config-rationale-guard.sociobot.in
is byte-identical to this candidate and is affected.

See `.factory/verification.md` for exact reproduction and all evidence. Do
not publish or approve this candidate. Required remediation: value-free schema
errors plus regression tests; fix the 390 px horizontal overflow; configure
the deployed cache and response-policy headers; then rerun independent QA.

Verified commands: `npm ci`, `cargo fmt --all -- --check`, `cargo clippy
--workspace --all-targets -- -D warnings`, `npm test`, `npm run build`, and
`cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty`.

# Original build handoff — Config Rationale Guard v0.1.0

## Shipped

- A release-buildable Rust/Clap `crg` binary with four non-interactive commands:
  `init`, `stamp`, `check`, and `diff`.
- Strict JSON (including duplicate-key rejection), YAML, and TOML normalization;
  explicit JSON Schema validation for every supported config format.
- Adjacent `.rationale.json` records with JSON Pointer targets, SHA-256 value
  fingerprints, owner/policy/review metadata, wildcard coverage rules, orphan
  detection, and overdue/stale checks.
- Human and `--json` reports with stable exit codes. Reports include changed
  paths and human rationale but never configuration values.
- A responsive static documentation site at `dist/site`, including a fully
  local browser checker, install documentation, first-class empty/error/offline
  states, privacy and terms pages, a versioned service-worker shell, and a
  one-time $49 Team unlock.
- The Team flow implements the Sociobot contract: hosted buy link, returned
  license capture and URL stripping, local storage, at-most-daily background
  verification, optimistic cached offline access, invalid-license locking, and
  paste-to-restore. Core validation, reports, and export remain free.
- The required original risograph hero illustration (157 KB WebP), generated
  with the factory image deployment. The exact prompt/deployment metadata is in
  `site/public/rationale-press.provenance.json`; visual tokens and rationale are
  in `.factory/design.md`.

## Run and verify

```sh
npm ci
npm test
npm run build
dist/bin/crg --help
cargo package --manifest-path cli/Cargo.toml --locked
```

`npm run build` is the reproducible work-order build command. It produces the
single CLI binary at `dist/bin/crg` and the static deployment root (including
`index.html`) at `dist/site`.

Verification completed on 2026-08-27:

- `cargo fmt --all -- --check`: pass.
- `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `npm test`: pass — 4 CLI integration tests and 3 browser-checker tests.
- `npm run build`: pass from the locked dependency state.
- `cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty`: pass;
  75.6 KiB package / 20.2 KiB compressed.
- Factory `verify-url.sh` on desktop and 390 px: pass; no console errors, one
  `h1`, `lang`, `main`, image alt, or unnamed-button failures.
- Axe Core WCAG 2 A/AA and 2.1 A/AA on `/`, `/privacy/`, and `/terms/` at both
  1366 px and 390 px: zero violations (and zero serious/critical findings).
- Playwright mobile smoke: local stamp → pass, edited value → stale, malformed
  JSON → error, and mocked returned-license verification → unlocked; pass.
- Lighthouse mobile: Performance 99, Accessibility 100, Best Practices 100,
  SEO 92; LCP 2.0 s, CLS 0, total blocking time 0 ms.
- Initial assets: JS 8.54 KB, CSS 16.39 KB, font 18.10 KB, hero 157 KB — all
  below the product budgets.

## Known gaps and release steps

- The factory still needs to register the production billing product before a
  real checkout can succeed. No product ID or payment-provider integration is
  embedded; the page intentionally uses the slug-based Sociobot production
  endpoint from the contract.
- Release binaries are not cross-compiled or attached here. The factory owns
  registry/release credentials; `cargo package --manifest-path cli/Cargo.toml
  --locked` is ready for its publishing workflow.
- JSON Schema is the only schema language by design. CUE, OpenAPI-specific
  dialect extensions, JSONC source files, and format-specific schema languages
  remain explicit non-goals for v1.
