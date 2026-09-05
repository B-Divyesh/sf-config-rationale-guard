# Review 1 — Config Rationale Guard

**Work order:** `config-rationale-guard-review-1`
**Verdict:** **FAIL**
**Findings:** 5 (critical 0, high 3, medium 1, low 1)
**Untested public claims:** 20
**Implementation candidate reviewed:** `fed7394ce256b785c71d301caec7a6fef8df65d9`
**Documentation/report HEAD:** `78d107e0b63a131bb72632757c7fe8b4aeda6338`
**Live URL:** https://config-rationale-guard.sociobot.in/

## Job, audience, and first action

The job is to let maintainers of agent, CI, and developer-tool settings keep a
human rationale next to a strict JSON, YAML, or TOML setting and catch stale or
missing rationale in CI. The audience is maintainers who review
policy-sensitive configuration changes. On the fresh desktop and phone landing
screens, the first primary action is **Install the CLI**; it does not start a
sample or explain that a sample will be loaded.

## Findings

### High — First screen does not say the job plainly or offer the required sample action

The live title and only H1 say “Keep the why beside the config.” This is the
vague wording in the controller finding, not a plain description of what the
tool does, for whom, and what changes. The first action is “Install the CLI”;
the nearby “Try the local checker” is an anchor, not “Try it with sample data,”
and it gives no before-click result. The page also retains the explicitly
non-plain section line “A sidecar, not a side quest.” This fails the first-screen
and plain-words contracts.

### High — The local checker is not an explicit isolated demo sandbox

`/demo` is only the landing page returned through navigation fallback. There is
no `/demo` or `?demo=1` demo state, one-click sample action, persistent “Demo
— sample data, nothing is saved” label, separate `demo:` storage namespace, or
`.factory/demo.md`. The form is prefilled, and Stamp → Check produces a
realistic PASS; invalid JSON, empty config, and Reset recover correctly. Those
good local paths do not establish the required labelled, directly linkable
sandbox or prove that the sample cannot affect real data.

### High — Required executable claim inventory is absent

`.factory/claims.json` does not exist, so there are no declared claim commands
to run and no one-to-one `@claim:` tests. I counted 20 distinct public claim
groups with no sandbox proof: supported formats; JSON Schema validation;
adjacent rationale records; strict source preservation; preserving comments and
formatting; no config values in reports; no new language; missing/orphaned/
overdue/schema/stale checks; value-free diffs; browser-local checking; no data
leaving the page; zero telemetry; offline use; one binary; no account;
human/JSON output; free MIT CLI; the $49 one-time Team materials; offline
license caching/daily verification; and no cloud dependency. Until each is
listed and its declared command passes from the demo/clean consumer flow, they
are untested public claims.

### Medium — Required site routes and crawler files are missing

There is no designed 404 page, `robots.txt`, or `sitemap.xml` in the source or
live build. `GET /not-a-real-page` returns HTTP 200 and the home document, not
a deliberate 404 route with a way back. `GET /demo` also returns that same home
document rather than a demo route. This prevents a person, crawler, or verifier
from distinguishing those destinations.

### Low — The required plain-words copy audit is missing

`.factory/copy-audit.md` is absent. The missing audit would have flagged the
controller wording above and other product-lore or mood copy, including “Two
plates. One reviewable decision.” and “Standardize the ritual.”

## Verified behavior and checks

- Fresh Chromium desktop (1366px) and phone (390px) loads had `lang=en`, one
  H1, a main landmark, no console/page errors, and no horizontal overflow.
  First-screen checks found no demo banner and no sample action at either size.
- The checker produced PASS after Stamp → Run local check. Invalid JSON showed
  a specific parse error; `{}` plus an empty decision list showed the empty
  state; Reset restored the example. Keyboard focus was visible. Reduced motion
  measured `0.00001s`. Normal page requests had no external origin.
- A service-worker-controlled, fresh phone context reloaded offline with its
  offline notice visible, no errors, and 390px width. Axe WCAG 2 A/AA and 2.1
  A/AA found zero violations on home, Privacy, and Terms at 390px.
- Privacy and Terms have route-specific titles, one H1, main landmarks, no
  console errors, and no axe violations. Their privacy statements were read.
  Legal and normal links checked in the rendered pages resolve; this review
  does not treat the intentionally absent 404 document as an expected 404,
  because it instead returns an indistinguishable 200 home page.
- Earlier verification dispositions: the schema-value disclosure is fixed by
  the current regression test; mobile overflow is fixed (390/390); deployed
  CSP/Permissions-Policy and immutable asset rules are present; and the
  previous controlled offline subresource failures do not reproduce. Fresh
  local build JS/CSS/service-worker SHA-256 values match live. The two commits
  after `fed7394` only modify handoff/verification reports.
- Clean setup: `npm ci`, `cargo fmt --all -- --check`, `cargo clippy
  --workspace --all-targets -- -D warnings`, `npm test`, `npm run build`, and
  `cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty` pass.
  The last command produces a 20.6 KiB packed crate. The documented commands
  are otherwise workable; no claim command exists to execute.

## CLI artifact exercise

The package was extracted and installed with `cargo install --path … --root …
--locked` in a clean consumer directory. Its installed `crg` binary was
exercised with `--version`, `init --json`, edited rationale, `stamp --json`,
`check --json`, and `diff --json` on a representative JSON config. This is the
real CLI flow, not the browser checker. No registry publish was attempted.

## Required repair and re-review

Rewrite the first screen as a plain job-led headline and audience sentence;
make “Try it with sample data” the visible first action and explain its result.
Implement `/demo` (or `?demo=1`) with shipped realistic sample data, persistent
demo label, Reset and Start-for-real controls, and an isolated documented
storage namespace. Add a complete claims inventory with one isolated test per
claim, remove claims that cannot be tested, then add the copy audit, robots,
sitemap, and a real styled 404 response. Re-run this review only after the
claims commands and those user paths pass.
