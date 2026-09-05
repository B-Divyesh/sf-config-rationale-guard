# Review handoff — Config Rationale Guard v0.1.0

**Work order:** `config-rationale-guard-review-1`
**Verdict:** **FAIL**
**Implementation reviewed:** `fed7394ce256b785c71d301caec7a6fef8df65d9`
**Documentation/report HEAD:** `78d107e0b63a131bb72632757c7fe8b4aeda6338`

The CLI and the prior implementation fixes remain verified, but this review
does not approve the product. There are 5 findings (high 3, medium 1, low 1)
and 20 untested public claim groups. The blocking work is an explicit
one-click, isolated and labelled demo; plain first-screen wording; executable
claim coverage; and complete public routes/crawl files.

## How to verify the passing parts

```sh
npm ci
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm test
npm run build
cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty
```

The packed crate installs and its real binary passes the documented init,
stamp, check, and diff flow in a clean consumer directory. The live desktop
and 390px phone pages have no console errors or axe violations; normal,
invalid, empty, reset, keyboard, reduced-motion, and controlled offline
checker paths pass. The live static assets match the last implementation
candidate.

## Known gaps / next steps

See `.factory/review-1.md` for the complete FAIL evidence and repair list.
Do not claim approval until all findings are repaired and every public claim
has a passing declared sandbox command.
