# Independent verification handoff — FAIL

**Candidate:** `57d04f363d94589d6785ad1a5f2e051f1b616ab0`
**Live URL:** https://config-rationale-guard.sociobot.in/
**Result:** **FAIL — do not release this candidate.**

The full independent record is in `.factory/verification-2.md`. Fresh hashes
show the live site is not candidate 57d04f3: this commit builds
`crg-shell-v2`, while the live service worker was `crg-shell-v4` and its HTML,
JS, CSS, and service-worker bytes differed. Candidate offline reload also
logged failed resource requests after worker control.

## Verification commands

```sh
npm ci
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm test
npm run build
cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty
```

All commands above passed locally. The package is ready to be checked without
publishing via `cargo package --manifest-path cli/Cargo.toml --locked`.

## Required next steps

1. Deploy the exact build from candidate `57d04f3` or establish a verifiable,
   immutable deployed build identity.
2. Repair and retest the PWA offline subresource fallback until a fresh
   controlled offline reload has no `net::ERR_FAILED` console/request errors.
3. Rerun deployment identity and browser QA before approval.

---

# Prior repair handoff — Config Rationale Guard v0.1.0

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
