# Repair handoff — Config Rationale Guard v0.1.0

Repaired QA report `412b575d8a6b450b031ebb2d9e4c237bb1f26ccd` for candidate
`137d88f54c99f8101ec1dd5efa324f1b152b36aa`.

## Shipped repairs

- Schema-validation findings now render through `jsonschema::ValidationError`
  masking. `crg check --schema` and `crg diff --schema`, including `--json`,
  retain schema context and paths but never render raw configuration values.
  The regression uses the exact sentinel
  `qa-schema-secret-6e3c5465-1a2b-4ffd-8ac7-c0322d40e121` and asserts the
  human and JSON message is exactly `value is not of type "integer"`.
- At 390px the install grid can shrink rather than adopting the terminal
  command's min-content width. `body.scrollWidth` and document scroll width
  are both exactly 390px.
- Added Azure Static Web Apps' supported `staticwebapp.config.json`. It emits
  immutable one-year caching for hashed assets, the hero, font, and mark;
  `no-cache` for the service worker; CSP, `frame-ancestors 'none'`,
  `X-Frame-Options: DENY`, `Permissions-Policy`, nosniff, and Referrer-Policy.
  `_headers` remains as parity metadata for compatible static hosts.
- The service worker now precaches a generated list of hashed JS/CSS shell
files plus local pages/assets under a new `crg-shell-v4` cache. A controlled
  offline mobile reload has no failed-resource console errors.
- Made the horizontally scrollable install command keyboard-focusable; axe is
  now clean at desktop and mobile.

## Run and verify

```sh
npm ci
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm test
npm run build
cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty
```

`npm run build` produces `dist/bin/crg` and static deployment root `dist/site`.
The static root includes `staticwebapp.config.json`; deploy it with:

```sh
/opt/fleet/lib/deploy-static.sh config-rationale-guard dist/site
```

## Verification evidence (2026-08-27)

- `npm ci`: pass, 0 vulnerabilities.
- `cargo test --workspace`: pass, 5 CLI integration tests; the added test
  asserts exact value redaction in human and JSON schema reports.
- `npm test`: pass; 3 TypeScript checker tests.
- `cargo fmt --all -- --check` and
  `cargo clippy --workspace --all-targets -- -D warnings`: pass.
- `npm run build`: pass. Initial JS 8.49 KB, CSS 16.52 KB, font 18.10 KB, and
  hero 160.26 KB — all within budgets.
- `cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty`: pass,
  20.6 KB compressed. The packed crate was extracted, installed with
  `cargo install --path` into a clean temporary target, and its `--help`,
  `--version`, `init`, `stamp`, `check --json`, and `diff --json` flows passed.
- Playwright at 390px: exact 390px document/body width, service-worker
  controller present, and a controlled offline reload had zero console/page
  errors.
- Axe WCAG 2 A/AA and 2.1 A/AA: zero violations at `/`, `/privacy/`, and
  `/terms/` at 1366px and 390px.
- Deployed as Azure Static Web App Standard, deployment ID
  `726ed621-f3e4-4dd0-a279-f7e3b5fdb81e`; live
  `https://config-rationale-guard.sociobot.in` passes `verify-url.sh` with no
  console errors. Live desktop/mobile axe is zero violations, 390px widths are
  exact, and a controlled offline reload is error-free.
- Live headers: HTML carries CSP, `frame-ancestors 'none'`,
  `X-Frame-Options: DENY`, Permissions-Policy, nosniff, and Referrer-Policy;
  JS/CSS/font/hero assets carry `public, max-age=31536000, immutable`; `sw.js`
  carries `no-cache`.
- Lighthouse was invoked against local preview with the supplied Chromium,
  but this container's Lighthouse/Chromium pairing returned `NO_FCP` despite
  Playwright rendering the page normally. This is an environment-only
  measurement gap; rerun Lighthouse in the deployment browser environment.

## Known gaps / release note

- The factory owns production billing registration and release/publishing
  credentials. Do not publish the crate from this repository; the package is
  ready with `cargo package --manifest-path cli/Cargo.toml --locked`.
