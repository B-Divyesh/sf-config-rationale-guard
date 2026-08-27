# Independent verification 3 — PASS

**Work order:** `config-rationale-guard-verify-3`
**Candidate:** `fed7394ce256b785c71d301caec7a6fef8df65d9` (`main`)
**Verified:** 2026-08-27
**Live URL:** https://config-rationale-guard.sociobot.in/

## Verdict

**PASS.** The candidate meets the researched brief's smallest useful product:
an offline CLI can create adjacent rationale records for strict JSON/YAML/TOML,
validate schema/targets/freshness/coverage, and render value-free decision
diffs. The deployed static site is byte-identical to this candidate's freshly
built production artifact. The earlier deployment-identity and offline
subresource failures do not reproduce.

## Defects

No release-blocking defects found.

| Severity | Count | Detail |
| --- | ---: | --- |
| Critical | 0 | None |
| High | 0 | None |
| Medium | 0 | None |
| Low | 0 | None |

## Clean build, quality gates, and package

- Began at the requested SHA with a clean tracked worktree. `npm ci` completed
  with 0 production-project npm vulnerabilities.
- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D
  warnings`, `npm test`, and exact `npm run build` all passed. Tests: five Rust
  CLI integration tests plus five site tests (checker and service-worker
  regression tests).
- `cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty`
  passed: 11 files, 77.6 KiB unpacked / 20.6 KiB compressed. The packed crate
  was extracted into a fresh `/tmp` consumer root and installed with `cargo
  install --path … --root … --locked`; installed `crg 0.1.0`, `--help`, and
  `init` worked. No publishing was attempted.
- Production build emitted `dist/bin/crg` and `dist/site`; initial JS is 8,483
  B, CSS 16,520 B, self-hosted font 18,096 B, and hero WebP 160,258 B. All are
  within the stated 200/50/120/300 KiB budgets.
- Lighthouse against local production preview (mobile configuration):
  Performance 99, Accessibility 100, Best Practices 100, SEO 92; LCP 2,075
  ms, CLS 0, TBT 69 ms.

## CLI end-to-end evidence

- Exercised the release binary with a representative JSON agent config:
  `init --json`, review/rule edits, `stamp --json`, and `check --schema
  --json` passed. The coverage rule `/permissions/**` reported 1/1 (100%).
- `diff --json` of a policy-sensitive setting change reported only path,
  setting/rationale status, rationale, and policy—never either old or new
  configuration value.
- Changing a stamped boundary value (`retries` 2 → 3) returned exit 1 with
  `stale_rationale`; a further `stamp` recovered to PASS. Missing input and a
  duplicate JSON key each returned exit 2 with actionable errors.
- The suite independently covers JSON/YAML/TOML init, schema validation,
  duplicate map rejection, coverage, stale rationale, and a unique
  secret-shaped schema sentinel. Both human and JSON schema reports omit the
  sentinel and report the value-free message `value is not of type
  "integer"`.

## Live deployment identity, privacy, and response policy

Fresh `npm run build` hashes exactly match live responses:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `e28a996f7505f4dac6ae61b71cc41032e3d3826fcb252231ebb6d76ae14bfb37` |
| `assets/main-BSJ6odKI.js` | `65bc3d1a31830c03a26c0027b6a3b272adca19c66af10bc52aba89f4a888434c` |
| `assets/main-BzNPClii.css` | `6cccd01392934d29ca7bf2d806b8aeedb8d1d28aaeda332736c1d03f11fc0b83` |
| `sw.js` | `7670c945be98318699ac1cdd963ca4b695b37e0b20a424e4b36c68aa95eb839f` |

- A normal live page load requested only `config-rationale-guard.sociobot.in`;
  no analytics, tracking, CDN, or other third-party runtime request occurred.
  The only optional external origin is the documented Sociobot billing API,
  called only after a license is supplied.
- Mocking a valid license API response verified token storage in the documented
  local-storage key, query-token removal with `history.replaceState`, and
  visible unlock. No free validation/reporting path is gated.
- Live CSP, HSTS, `nosniff`, Referrer-Policy, Permissions-Policy, and frame
  denial are present. Hashed JS/CSS, font, and hero use one-year immutable
  caching; `sw.js` is `no-cache`.

## Browser, accessibility, and PWA evidence

- Playwright Chromium tested the live home at 1366px and 390px. Both had
  `lang=en`, title, exactly one h1, main landmark, no console/page errors, and
  no horizontal overflow (390/390/390 document/body/client widths on mobile).
- Axe WCAG 2 A/AA + 2.1 A/AA found zero violations, including zero
  serious/critical, on `/`, `/privacy/`, and `/terms/`; legal pages also had
  title, language, one h1, main, and no 390px overflow/errors.
- Keyboard demo flow reached the local PASS state; malformed JSON returned a
  clear parse error; `{}` plus an empty sidecar returned the empty state; reset
  recovered. Focus was visible as a `3px` blue outline. Reduced-motion browser
  context measured `0.00001s` transitions.
- The live worker is controller-active. After an online controlled reload, a
  390px offline reload rendered the shell and offline note with no console/page
  errors and no overflow. `sw.js` is cache `crg-shell-v5`, precaches current
  emitted assets, and is versioned for update.

## Release recommendation

Approve `fed7394ce256b785c71d301caec7a6fef8df65d9`.
