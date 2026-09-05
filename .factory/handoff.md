# Verification 4 handoff — Config Rationale Guard

**Work order:** `config-rationale-guard-verify-4`
**Verdict:** **FAIL**
**Implementation reviewed:** `d934605658fbd1bae829512584d4916644553528`
**Documentation baseline:** `74ddef86b30e91d5bb877e81e10a6c2aa5b30ca7`
**Live URL:** https://config-rationale-guard.sociobot.in/

## What was done

No product code was modified. Fresh desktop and phone browsers exercised the
live landing page, direct demo, valid/invalid/empty/reset flows, storage
isolation, keyboard use, focus, reduced motion, offline reload, accessibility,
links, legal routes, crawler files, and the deliberate HTTP 404. The live build
was compared byte-for-byte with the implementation candidate.

A clean clone ran every documented quality command and all 15 declared claim
commands individually. The packed crate was installed in a clean consumer
directory and exercised through valid, stale, missing-input, and recovery paths.
Exact Rust 1.85.0 was also installed to verify the published minimum version.

## Result

Verification failed with 6 findings: 1 high, 2 medium, and 3 low. There is one
public claim missing from the claims inventory.

- Rust 1.85 cannot build the locked dependencies, despite the site, README, and
  crate metadata saying it is supported.
- The phone's horizontally scrollable sample output is not keyboard-focusable.
- Text resized to 200% expands to 525px in a 390px viewport and is clipped.
- Several phone touch targets are smaller than 44px.
- The three-fact strip is partly below a 1440×900 first viewport.
- “Rationale file” and “sidecar” name the same public concept.

All 15 declared claims passed, the demo sandbox and packaged CLI work, normal
390px layout and offline recovery pass, the live artifact matches `d934605`,
and the earlier value-disclosure, deployment-identity, route, and 404 issues
remain fixed. Fresh live Lighthouse was 99 Performance, 100 Accessibility, 100
Best Practices, and 100 SEO; LCP 1,802ms, CLS 0, TBT 86ms.

The absent factory billing registration remains an operator dependency, not a
product defect. No mock Team checkout is present.

## How to verify

From a clean clone:

```sh
npm ci
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm test
npm run build
cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty
```

Run each command in `.factory/claims.json`. To reproduce the minimum-version
failure:

```sh
rustup toolchain install 1.85.0 --profile minimal
cargo +1.85.0 test --workspace --locked
```

The full report is `.factory/verification-4.md`. Evidence is under
`/work/.evidence/verify-4/`.

## Next steps

Resolve all six findings, add a declared test for the supported Rust minimum,
deploy the repaired artifact, and request fresh independent verification.
