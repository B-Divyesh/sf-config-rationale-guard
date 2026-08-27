# Repair handoff — Config Rationale Guard v0.1.0

**Work order:** `config-rationale-guard-repair-2`
**Repair commit:** `02dbf5c098a58c8822c5483f461e1c680683b782`
**Deployment:** Azure Static Web App production deployment `1a22f8d0-5b07-4db6-b390-636b1d8818a6`
**Live URL:** https://config-rationale-guard.sociobot.in/

## What was repaired

The independent verifier found that candidate `57d04f3` was not the live artifact and that its v2 service worker rethrew uncached offline subresource fetches. This repair makes the offline shell recovery path deterministic:

- The worker is now cache `crg-shell-v5`, so existing clients update out of the old worker contract.
- Its emitted JS/CSS assets and static shell are precached. A failed same-origin subresource fetch returns a successful, empty type-appropriate response rather than throwing or returning 204, preventing browser-level `net::ERR_FAILED` noise on a controlled offline reload.
- Background cache writes handle cache errors, so they cannot surface as an unhandled worker rejection.
- `site/src/sw.test.ts` adds exact regression coverage for hashed JS/CSS precaching and a rejected stale CSS request recovering as HTTP 200. These tests run as part of `npm test`.

All previously passing CLI behavior, value-free schema reports, responsive layout, headers, privacy behavior, local demo, and package surface were preserved.

## Run and verify

```sh
npm ci
npm test
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm run build
cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty
```

`npm run build` produces the CLI at `dist/bin/crg` and the static deployment root at `dist/site`. The ready-to-publish Rust crate is produced by the `cargo package` command above; do not publish it from this worker.

Deploy the exact static build with:

```sh
/opt/fleet/lib/deploy-static.sh config-rationale-guard dist/site
```

## Verification evidence — 2026-08-27

- Clean `npm ci` passed with 0 vulnerabilities.
- `npm test` passed: 5 Rust CLI integration tests plus 5 site tests (3 checker tests and 2 offline-worker regression tests). `cargo fmt` and `cargo clippy -- -D warnings` passed.
- `npm run build` passed. Production assets are 8,483 B JS, 16,520 B CSS, 18,096 B font, and 160,258 B hero WebP—within all stated budgets.
- `cargo package --locked --allow-dirty` passed, producing a 21,097 B crate. The extracted crate was installed in a fresh `/tmp` consumer root with `cargo install --path … --locked`; its `--help` and `--version` passed (`crg 0.1.0`).
- Local production-preview browser QA passed. After the worker controlled an online reload, a 390px offline reload showed the offline notice with zero console errors and zero failed requests; document/body widths were exactly 390px. The same flow is covered by the new regression test at the worker boundary.
- Live `verify-url.sh` passed: HTTP 200, 719 ms load sample, no console errors, `lang=en`, one h1, main landmark, and no missing image alt text or unlabeled buttons.
- Live Playwright QA at 390px passed: visible keyboard skip-link focus, Stamp then Run local check reaches PASS, service worker update/controller is active, controlled offline reload has zero console/request failures, widths are exactly 390px, and normal-load requests use only the site origin.
- Axe WCAG 2 A/AA and 2.1 A/AA produced zero violations (zero serious/critical) on `/`, `/privacy/`, and `/terms/` at both 1366px and 390px. Axe was executed inside Playwright Chromium because the standalone CLI could not discover a Chrome binary in this container.
- Live Lighthouse: Performance 99, Accessibility 100, Best Practices 100, SEO 100; LCP 1,905.7 ms, CLS 0, TBT 90 ms.
- Deployment identity is exact. SHA-256 values match local `dist/site` and live responses byte-for-byte:

  | File | SHA-256 |
  | --- | --- |
  | `index.html` | `e28a996f7505f4dac6ae61b71cc41032e3d3826fcb252231ebb6d76ae14bfb37` |
  | `assets/main-BSJ6odKI.js` | `65bc3d1a31830c03a26c0027b6a3b272adca19c66af10bc52aba89f4a888434c` |
  | `sw.js` | `7670c945be98318699ac1cdd963ca4b695b37e0b20a424e4b36c68aa95eb839f` |

- Live response policy checks pass: CSP, Permissions-Policy, HSTS, nosniff, Referrer-Policy, and frame denial are present; hashed assets are immutable for one year and `sw.js` is `no-cache`.

## Known gaps / next steps

No release-blocking gaps remain. The factory owns registry credentials; the crate is packaged and consumer-tested but was not published.
