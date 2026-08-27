# Independent verification 2 — FAIL

**Work order:** `config-rationale-guard-verify-2`
**Candidate:** `57d04f363d94589d6785ad1a5f2e051f1b616ab0` (`main`)
**Verified:** 2026-08-27
**Live URL checked:** https://config-rationale-guard.sociobot.in/

## Verdict

**FAIL.** The production URL is not the requested candidate artifact. A fresh
production build of the candidate and the live HTML, JavaScript, CSS, and
service worker have different SHA-256 digests. The live worker identifies
itself as `crg-shell-v4`, while this commit builds `crg-shell-v2`. This makes
the deployed product unreviewable against the candidate and violates the
required deployment-identity check.

The candidate's own production artifact also produces offline console resource
failures after the service worker has taken control.

## Defects

### High — live deployment is a different artifact from candidate 57d04f3

`npm run build` from a clean dependency install produced:

| File | Candidate SHA-256 | Live SHA-256 |
| --- | --- | --- |
| `index.html` | `d72bc0c50ee17b2feb46ccb722bb6c4c5ecec1fd804dafec2404c0d671293ff6` | `e28a996f7505f4dac6ae61b71cc41032e3d3826fcb252231ebb6d76ae14bfb37` |
| main JS | `210ddbffb1892d979e9edd43c234fc948516468ab893154e14c35b62a345e482` | `65bc3d1a31830c03a26c0027b6a3b272adca19c66af10bc52aba89f4a888434c` |
| `sw.js` | `e1d167546a24c1017bc225cebcd343d3ea34a4ac0ad031ba8d8762bd12f724f9` | `c2442032d0750409d8de2fb9bf5d1a9e2288af2a2abd5ed7079d4b4f74606475` |

At the time of the final sample, live referenced
`/assets/main-BSJ6odKI.js` and served `const CACHE = 'crg-shell-v4';`; the
candidate generated `/assets/main-DskXjbkc.js` and `crg-shell-v2`. The live
assets changed during this audit (earlier samples were v3), further confirming
that it cannot be treated as a stable deployment of the requested SHA.

### Medium — candidate offline reload logs resource failures

Using the exact `dist/site` from `npm run build`, served by Vite preview on a
fresh browser profile: the service worker became active and controlled an
online reload. A subsequent 390px offline reload rendered the shell, but
Chrome recorded `net::ERR_FAILED` requests (the isolated request sample
identified `/fraunces-latin-600.woff2`; a second run recorded three failed
resources). The offline notice was not visible in that automated offline
transition. The service worker's fetch handler rethrows uncached offline
subresource errors, so the stated offline recovery path is not error-free.

## Passing evidence

- Clean checkout started at the requested SHA with no tracked modifications.
  `npm ci` passed with 0 npm vulnerabilities.
- `cargo fmt --all -- --check` and
  `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `npm test` passed: five Rust CLI integration tests and three TypeScript
  checker tests. `npm run build` passed, producing `dist/bin/crg` and
  `dist/site`. `cargo package --manifest-path cli/Cargo.toml --locked
  --allow-dirty` passed (20.6 KiB compressed, 11 files).
- Candidate budgets pass: initial JS 8,536 bytes; CSS 16,443 bytes; self-hosted
  font 18,096 bytes; hero WebP 160,258 bytes.
- Browser QA against the live URL found no console/page errors on normal load,
  zero axe WCAG 2 A/AA + 2.1 A/AA violations (including zero serious/critical)
  at desktop and 390px for `/`, `/privacy/`, and `/terms/`. Each page had
  `lang=en`, a title, one h1, and main; 390px document/body widths were exactly
  390px. Default load requested only the site origin.
- Live demo: stamp then check passed; malformed JSON displayed a clear parse
  error; `{}` with an empty sidecar displayed the empty state; reset recovered.
  Keyboard focus was visible (3px blue outline plus coral/paper rings), and
  reduced motion reported effectively zero-duration transitions.
- Live response policy includes CSP, Permissions-Policy, HSTS, nosniff,
  Referrer-Policy, and X-Frame-Options. Hashed JS/CSS, font, and hero carry
  one-year immutable caching; `sw.js` carries `no-cache`. No normal-load
  analytics, CDN, or third-party request was observed.
- The candidate CLI's tested documented flow initialized JSON, accepted edited
  rationales, stamped hashes, validated schema, and returned stale/invalid
  inputs through its documented non-zero exits. The schema regression masks a
  sentinel secret in both human and JSON output; normal diff output remains
  value-free.
- The packed crate was extracted to a clean temporary consumer and installed
  with `cargo install --path ... --root ... --locked`. Its `--help` and
  `--version` worked; `init --json`, edited-sidecar `stamp --json`, schema
  `check --json`, stale-value detection (exit 1), duplicate JSON rejection
  (exit 2), and schema-secret redaction (exit 1 without the sentinel) all
  passed. No registry publish was attempted.

## Release recommendation

Do not approve candidate `57d04f3`. Deploy the exact committed `dist/site`
artifact (or provide an immutable build identity), then rerun the identity and
offline checks. Fix the service-worker offline subresource fallback so a
controlled offline reload has no request/console failures.
