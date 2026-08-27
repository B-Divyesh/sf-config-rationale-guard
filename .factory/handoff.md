# Repair handoff — Config Rationale Guard v0.1.0

## Completed

- Removed the schema-report value disclosure. `schema_violation` now reports
  only its JSON Pointer path and the stable message `configuration does not
  satisfy the supplied JSON Schema`; it never stringifies a validator error or
  renders the failing instance.
- Added exact CLI regressions using the sentinel
  `QA_SCHEMA_SECRET_DO_NOT_REPORT_7c75506b`. Both human and `--json` schema
  reports must contain `schema_violation` and the stable message while omitting
  the sentinel.
- Fixed the 390px Install-section overflow by allowing its grid children to
  shrink and flattening the terminal treatment inside the mobile reading width.
  The horizontally scrollable command block is now keyboard focusable.
- Replaced the static-host-ignored `_headers` approach with emitted Azure Static
  Web Apps configuration. Content-hashed `/assets/*` receive one-year
  immutable caching; documents revalidate; `/sw.js` is no-cache/no-store.
  Production responses now carry a self-only CSP (with the Sociobot license
  API explicitly allowed for `connect-src`), `X-Frame-Options: DENY`, and a
  restrictive Permissions-Policy.
- Versioned the service-worker cache, precached the local font, disabled
  production source maps, and return safe cached/empty offline fallbacks so an
  offline reload stays usable without failed-resource console errors.

## Verification

Run from a clean clone:

```sh
npm ci
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm test
npm run build
cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty
```

Completed on 2026-08-27:

- Rust format and clippy passed. `npm test` passed: 5 CLI integration tests
  (including the human/JSON schema-redaction sentinel regression) and 3 site
  checker tests.
- `npm run build` passed and produced `dist/bin/crg` plus `dist/site`.
  Production assets are 8.49 KB JS, 16.49 KB CSS, 18.10 KB local font, and
  160.26 KB hero WebP.
- `cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty` passed
  (20.4 KiB compressed). The crate was extracted, installed into an isolated
  consumer root with `cargo install --path ... --locked`, and exercised with
  `--help`, `init`, and schema `check --json`; the consumer secret did not
  appear in the saved report.
- Local and live `verify-url.sh` passed with title, `lang`, one `h1`, `main`,
  image alt text, and zero page/console errors.
- Axe Core WCAG 2 A/AA and 2.1 A/AA returned zero violations for `/`,
  `/privacy/`, and `/terms/` at 1366px and 390px. The live home document and
  body both measured exactly 390px at the 390px viewport.
- A controlled live offline reload rendered the page and offline notice with no
  console errors.
- Live header checks confirmed `Cache-Control: public, max-age=31536000,
  immutable` for `/assets/*`, `no-cache, no-store, must-revalidate` for
  `/sw.js`, and CSP, X-Frame-Options, Permissions-Policy, nosniff, and
  Referrer-Policy on production responses.

## Deployment

Committed product repair: `61623cc2bbeb2517191b12bce4b7fade0649fa33`

Deployed as a Standard Azure Static Web App to
https://config-rationale-guard.sociobot.in on 2026-08-27. No registry publish
was performed; the factory owns release credentials.

## Known gaps / next steps

- The Team billing product still needs factory registration before real hosted
  checkout can complete. The client remains scoped to the required Sociobot API
  contract and core CLI features remain free.
- Release binaries are not cross-compiled or attached here. The verified crate
  is ready for the factory publishing workflow with the `cargo package` command
  above.
