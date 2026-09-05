# Verification 4 — Keep config decisions reviewable (FAIL)

**Work order:** `config-rationale-guard-verify-4`
**Verdict:** **FAIL**
**Findings:** 6 (critical 0, high 1, medium 2, low 3)
**Untested public claims:** 1
**Implementation candidate:** `d934605658fbd1bae829512584d4916644553528`
**Documentation baseline:** `74ddef86b30e91d5bb877e81e10a6c2aa5b30ca7`
**Live URL:** https://config-rationale-guard.sociobot.in/

## Job, audience, and first action

The job is to keep the reason for a configuration decision beside strict JSON,
YAML, or TOML and catch missing or stale decisions before CI. The audience is
maintainers of agent, CI, and developer-tool settings. In fresh 1440×900 desktop
and 390×844 phone browsers, the job-led heading, audience sentence, **Try it
with sample data** action, and “Opens a temporary sample project” outcome were
all visible before scrolling.

## Findings

### High — The documented Rust 1.85 setup cannot build the locked package

The home page and README both say “Use Rust 1.85 or later,” and
`cli/Cargo.toml` declares `rust-version = "1.85"`. This support statement is
not listed in `.factory/claims.json` and has no declared claim command.

From the fresh checkout, I installed exact Rust 1.85.0 and ran:

```sh
cargo +1.85.0 test --workspace --locked
```

Cargo stopped before compilation because the locked ICU packages require Rust
1.88 and `idna_adapter` requires Rust 1.86. This is a false installation claim
for the minimum supported toolchain and the one untested public claim counted
in this report. Either pin dependencies compatible with 1.85 and test that
toolchain, or raise the public and package minimum and add a declared claim
test.

### Medium — The phone sample output is not keyboard-scrollable

At 390×844, axe reports one **serious** `scrollable-region-focusable`
violation on `.terminal-recording > pre`. The recorded CLI output overflows
horizontally but the `<pre>` has neither focusable content nor its own focus
target, so a keyboard user cannot reach and scroll the complete sample. The
other audited routes have zero axe violations.

### Medium — Text resized to 200% clips the page

With text set to 200% in a 390px browser, the document and body become 525px
wide while the page applies `overflow-x: hidden`. The first screen, primary
action, illustration, terminal output, and footer extend beyond the viewport.
This fails the attached requirement that text resize to 200% without loss.
Evidence: `/work/.evidence/verify-4/live-phone-text-200-percent.png` and the
`textResize` section of `live-qa.json`.

### Low — Several phone touch targets are below 44px

At 390px, the header **Install** link is 40px high, the skip link is 42px high,
and the footer Demo, Privacy, and Terms links are 16px high. These controls are
usable but do not meet the required 44×44px target size.

### Low — The three first-screen facts are clipped on desktop

At 1440×900, the three fact strip starts at y=883 and ends at y=914, leaving
its lower 14px below the initial viewport. The required job, audience, action,
and action result do fit; only the mandatory three-fact portion of the
plain-words first-screen shape is clipped.

### Low — Public copy uses two names for the adjacent rationale file

The landing page calls the same object a “rationale file” and a “sidecar”
(“Create the sidecar”), while `.factory/copy-audit.md` says the one term is
“rationale file.” The installed CLI help also says “rationale sidecar.” This
does not block use, but it fails the plain-words rule to use one term for one
concept and makes the copy audit incomplete.

## Demo and browser results

- One click from the first screen opened `/demo` with a realistic agent config,
  two policy decisions, and the persistent **Demo — sample data, nothing is
  saved** label.
- Stamp then check produced PASS with two decision records, full required-path
  coverage, and a value-free result. Invalid JSON produced a specific parse
  error; an empty config produced an actionable empty state; Reset restored the
  shipped sample.
- A non-demo local-storage sentinel remained unchanged throughout. Reset and
  Start for real removed the `demo:config-rationale-guard:checker` key.
- The complete demo flow requested only the product origin and produced no
  console or page errors.
- Keyboard Tab reached the skip link, editors, and controls; Space activated
  Stamp. The visible focus ring measured 3px. Reduced-motion transitions and
  animations measured `0.00001s`.
- A service-worker-controlled phone context updated online, reloaded offline,
  showed the offline note, and completed Stamp → Check without errors.
- Home, Demo, Privacy, and Terms returned 200. The designed missing route
  returned the expected HTTP 404 with its own title and way back. Its browser
  404 resource message is expected and is not a defect.
- Every rendered internal link and fragment target resolved. The privacy and
  support `mailto:` links are present.

## Claims and clean consumer results

All 15 commands declared in `.factory/claims.json` passed individually from the
fresh checkout. The inventory has one source test tag per declared ID and no
duplicate command. The Rust 1.85 statement above is the one public claim absent
from that inventory.

The documented quality gate passed on the container's current Rust toolchain:

```sh
npm ci
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
npm test
npm run build
cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty
```

`npm test` passed 16 Rust tests, 5 TypeScript tests, and 8 browser tests. The
crate packaged as 23.2 KiB compressed. I extracted it into a clean consumer
directory and installed it with `cargo install --path … --root … --locked`.
The installed `crg 0.1.0` passed `--help`, `--version`, `demo`, and JSON check.
A missing input exited 2, an edited stamped value exited 1, and Stamp followed
by Check recovered to exit 0.

## Live identity, structure, privacy, and performance

The fresh production build and live HTML, JavaScript, CSS, service worker,
font, and hero image have matching SHA-256 digests. This proves the live runtime
is the `d934605` implementation; commits through the `74ddef8` baseline change
only reports. Live headers include CSP, frame denial, HSTS, Permissions-Policy,
Referrer-Policy, and `nosniff`. Hashed assets are immutable for one year and
`sw.js` is `no-cache`.

Initial assets remain within budget: JavaScript 6,867 bytes, CSS 16,003 bytes,
self-hosted font 18,096 bytes, and hero WebP 160,258 bytes. Fresh live mobile
Lighthouse scored Performance 99, Accessibility 100, Best Practices 100, and
SEO 100; LCP was 1,802ms, CLS 0, and TBT 86ms. Lighthouse's accessibility score
does not invalidate the phone-specific axe and resize findings above.

All routes have their own plain title, description, canonical URL, one H1,
`lang=en`, and one main landmark. `robots.txt` and `sitemap.xml` return 200 and
list the four public routes. Normal route loads have no console errors, and no
analytics, CDN, third-party script, or cross-origin demo request was observed.

## Earlier finding disposition

| Earlier finding | Current disposition |
| --- | --- |
| Schema reports disclosed config values | Fixed. The declared human/JSON and diff regression passed with secret-shaped sentinels absent. |
| Normal 390px layout overflowed | Fixed at normal text size: document, body, and viewport are all 390px. The separate 200% resize defect is new. |
| Live headers and cache rules differed | Fixed; live policy headers and asset caching match the built configuration. |
| Offline reload logged resource errors | Fixed; controlled phone reload and offline checking completed with no errors. |
| First screen was vague and lacked a sample action | Fixed for job, audience, action, and outcome. The clipped fact strip is a smaller remaining layout issue. |
| No isolated demo | Fixed; direct route, label, demo-only storage, reset, exit, and realistic output all passed. |
| No claim inventory | Mostly fixed: all 15 declared commands pass, but the public Rust 1.85 support claim is undeclared and false. |
| Missing routes, crawler files, and real 404 | Fixed; all expected routes and crawler files work, and the deliberate 404 is correctly classified. |
| Missing copy audit | Present, but it misses the public “sidecar” terminology inconsistency. |
| Factory billing registration absent | Still an operator dependency, not a product defect. No mock checkout or paid state is shown. |

## Evidence and release decision

Detailed evidence is in `/work/.evidence/verify-4/`, including clean-gate logs,
all claim command output, packed-consumer output, screenshots, live browser JSON,
asset hashes and headers, crawler files, the Rust 1.85 failure, and Lighthouse
JSON.

**Release decision: FAIL.** Do not approve this candidate until all six
findings are fixed, every public claim is declared and passing, and independent
verification returns zero findings and zero untested claims.
