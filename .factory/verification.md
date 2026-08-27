# Independent verification — FAIL

**Work order:** `config-rationale-guard-verify-1`  
**Candidate:** `137d88f54c99f8101ec1dd5efa324f1b152b36aa` (`main`)  
**Verified:** 2026-08-27  
**Live URL:** https://config-rationale-guard.sociobot.in/

## Verdict

**FAIL.** The CLI prints a configuration value in a schema-validation report.
The brief requires generated reports never include config values or secrets.
The live site byte-matches this candidate, so deployment does not change the
result.

## Defects

### Critical — schema errors disclose config values

`crg check <config> --schema <schema> --json` emits raw
`jsonschema::ValidationError` text. A JSON config with a unique secret-shaped
QA sentinel at `/token`, paired with a schema requiring an integer, exited 1
and reported:

```json
{"code":"schema_violation","path":"/token","message":"\"<QA secret sentinel>\" is not of type \"integer\""}
```

The human report leaks the same value. In `cli/src/main.rs`,
`validate_schema` stores `issue.to_string()` in the report. CI logs or saved
decision reports can therefore disclose real secrets. Render only value-free
schema details and add human/JSON regression tests with a sentinel secret.

### Medium — 390 px page horizontally overflows

At the requested 390 CSS-pixel viewport, live `body.scrollWidth` was 428 px
while `clientWidth` was 390. The Install section (`.install-copy` and
`.terminal-block`) reaches x=428 and requires horizontal scrolling.

### Medium — production ignores shipped cache/policy rules

`site/public/_headers` specifies immutable assets, `no-cache` `/sw.js`, and a
Permissions-Policy. Live HTML, JS, CSS, font, image, and service worker all
return `Cache-Control: public, must-revalidate, max-age=30`; the response has
no Permissions-Policy. HSTS, `nosniff`, and Referrer-Policy are present.

### Low — offline reload logs failed-resource errors

Cached offline reload still renders the page, offline notice, and service
worker controller, but Chrome logs three `net::ERR_FAILED` resource errors.
Normal online desktop/mobile loads have no console or page errors.

## Passing evidence

- Clean initial worktree at the requested SHA; `npm ci` completed with 0
  vulnerabilities.
- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D
  warnings`, `npm test`, and exact `npm run build` all passed. Tests: 4 CLI
  integration tests and 3 TypeScript checker tests.
- `cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty` passed;
  package is 20.2 KiB compressed. The packed crate was extracted and installed
  into a clean consumer; `--help`, `--version`, `init`, `stamp`, `check`, and
  `diff --json` were exercised.
- End-to-end JSON init/edit/stamp/check passed. Edited stamped value returns
  exit 1 with `stale_rationale`; JSON/YAML/TOML initialize; duplicate JSON and
  missing input return exit 2. Normal diff reports no values.
- Desktop browser flow passed malformed-JSON error/recovery, stamp/check PASS,
  and empty state. Keyboard focus is a visible 3 px ring; reduced motion is
  effectively zero-duration.
- Axe WCAG 2 A/AA + 2.1 A/AA found zero violations (including zero
  serious/critical) on `/`, `/privacy/`, `/terms/` at desktop and 390 px.
- No analytics/tracking or runtime CDN was found. Normal load made no external
  request. Mocked license return verified URL stripping, local storage,
  verification, and unlock.
- Service worker controlled a successful offline reload. Lighthouse mobile
  local preview: Performance 99, Accessibility 100, Best Practices 100, SEO
  92; LCP 1.956 s, CLS 0, TBT 75 ms.
- Asset budgets pass: JS 8.54 KB, CSS 16.39 KB, font 18.10 KB, hero 160.26 KB.

## Deployment identity

Live `index.html`, JS, CSS, font, hero, SVG, `sw.js`, privacy, and terms files
all matched `npm run build` output byte-for-byte by SHA-256 and `cmp`. The
candidate is deployed despite the work-order `deploy: none` field.

## Release recommendation

Do not approve this candidate. Fix the critical value disclosure, then the
390 px overflow and deployed headers, and rerun independent QA.
