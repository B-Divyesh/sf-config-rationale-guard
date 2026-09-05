# Repair handoff — Config Rationale Guard v0.1.1

**Work order:** `config-rationale-guard-repair-3`
**Implementation SHA:** `d934605658fbd1bae829512584d4916644553528`
**Previous review/report SHA:** `f16c40f9204f5db0bd84c25c189263d0f0507e82`
**Live URL:** https://config-rationale-guard.sociobot.in/

## What changed

- Rewrote the first screen in plain words. It now states the job, names the
  maintainers it serves, and leads with **Try it with sample data**.
- Added `crg demo`, which writes bundled realistic config, rationale, and schema
  files into a temporary or supplied safe directory, then runs a real check.
- Added a direct `/demo` browser sandbox with a persistent **Demo — sample
  data, nothing is saved** label, `demo:config-rationale-guard:checker` storage,
  Reset demo, and Start for real controls. The demo cannot modify a non-demo
  local-storage key.
- Added 15 declared, isolated public-claim checks in `.factory/claims.json` and
  CLI/browser tests that prove observed results rather than source strings.
- Added `/demo`, `/privacy/`, `/terms/`, `robots.txt`, `sitemap.xml`, a styled
  HTTP 404, route metadata, canonical URLs, social image, apple touch icon, and
  Static Web Apps response configuration.
- Removed the unconfigured paid Team checkout rather than presenting a mock
  billing flow. The free MIT CLI remains fully useful.
- Added real Playwright axe, phone layout, console, reduced-motion, offline
  service-worker, direct-route, and browser privacy coverage.

## Review finding disposition

| Earlier finding | Current disposition |
| --- | --- |
| Vague first-screen copy and no sample action | Fixed: job-led H1, audience sentence, visible one-click sample action, and action result text. |
| No isolated demo | Fixed: direct `/demo`, shipped sample, persistent label, demo-only storage, reset, and explicit exit. |
| No claim inventory | Fixed: 15 claims, each with one declared executable sandbox command. |
| Missing routes, crawler files, and 404 | Fixed: real routes, `robots.txt`, `sitemap.xml`, and a deliberate HTTP 404 page. |
| Missing copy audit | Fixed: `.factory/copy-audit.md` records counts, banned-word scan, and terminology. |
| Schema reports could expose values | Still fixed: regression sends a secret-shaped sentinel through human and JSON schema reports and confirms it never appears. |
| 390px overflow | Still fixed: live and local phone width is 390/390/390. |
| Header/cache deployment mismatch | Fixed: live hashed JS is byte-identical to the deployed local artifact; immutable asset caching, `no-cache` worker, CSP, and Permissions-Policy are live. |
| Offline subresource errors | Fixed: a live service-worker-controlled 390px offline `/demo` reload shows the offline note with no errors. |

## Verification

From the documented clean setup, `npm ci` completed with 0 vulnerabilities.
These commands passed:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm test
npm run build
cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty
```

All 15 exact commands declared in `.factory/claims.json` were run after the
clean install and passed. `npm test` covers 16 Rust tests, 5 TypeScript tests,
and 8 browser tests. The browser suite includes axe WCAG 2/2.1 A/AA checks on
home, demo, Privacy, Terms, and 404; it found no serious or critical issue.

The packed `config-rationale-guard-0.1.0.crate` was extracted into a fresh
temporary consumer directory. `cargo install --path … --root … --locked`,
`crg --version`, and `crg demo --output …` all passed; the installed artifact
created its adjacent rationale file and completed its check.

Production deployment completed successfully from `dist/site`. Live checks
found home 200, `/demo` 200, Privacy 200, Terms 200, `robots.txt` 200,
`sitemap.xml` 200, and `/not-a-real-page` 404. The deliberate 404 console
network record is expected; normal home/demo/legal page loads have no console
or page errors. Crawled product links all return 200. The live initial JS
SHA-256 is `1a91692421ffbb00d68913bcd33da0034ae6314ad46a3f29b8fbde724079fa77`,
matching this implementation's `dist/site` asset.

Live Lighthouse mobile: Performance **100**, Accessibility **100**, Best
Practices **100**, SEO **100**; LCP 1,900 ms, CLS 0, TBT 42 ms. Production
asset sizes: JavaScript 6.87 KB, CSS 16.00 KB, font 18.10 KB, hero image
160.26 KB, and social image 112.86 KB.

Evidence is in `/work/.evidence/repair-3-live/`; the required catalog copy is
also at `/work/.evidence/catalog-description.txt`.

## Known gap

The researched opportunity allows a one-time team purchase, but no factory
billing registration was supplied for this repair. The site therefore presents
no price, checkout, or simulated paid state. Adding a real one-time Team
material bundle later requires factory registration with the Sociobot billing
API; it must not be replaced with a mock flow.
