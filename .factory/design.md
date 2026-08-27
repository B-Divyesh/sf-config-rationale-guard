# Visual thesis: the review room as risograph collage

Config Rationale Guard should feel like a decision record assembled at a real
review table: clipped paper, registration marks, stamped status, and dense ink.
The tactile risograph treatment is specific to the product's purpose. A machine
setting is the crisp black plate; its human rationale is the slightly offset
coral plate that makes the whole record legible. It is intentionally neither a
terminal-green developer cliché nor a generic SaaS gradient.

## Palette

The light treatment is the product's explicit single mode, like warm archival
stock under a desk lamp. `paper` #F3EBD7 is the background, `paper-high`
#FFF9EA the reading surface, `ink` #17211B the primary text, `ink-soft` #4B554C
the muted text, `coral` #C83D3D the decision/rationale accent, `blue` #145D78 the
machine/config accent, `moss` #34623F success, `ochre` #8A5900 warning, and
`danger` #A4252A error. Coral and blue are never used alone to communicate
state. All body text combinations meet 4.5:1; focus uses a 3px blue/coral
double-ring against both paper surfaces.

## Type and spacing

Headlines use self-hosted **Fraunces**, an editorial variable serif whose
slightly irregular forms feel printed. UI, prose, code, and tabular output use
the system monospace stack (`ui-monospace`, `SFMono-Regular`, Consolas), keeping
the site fast and tying it to the CLI. The scale is 14, 16, 20, 28, 44, and
clamp(52–84) px. Body copy is at least 16px with 1.55 leading and a 68-character
measure. Spacing follows a 4/8px rhythm: 4, 8, 12, 16, 24, 32, 48, 72, 96.

## Composition and interaction grammar

Sections read as overlapping sheets instead of anonymous cards. Hairline
registration crosses, numbered marginal labels, torn-edge masks, halftone
shadows, and deliberately offset ink layers create depth. The primary action is
always coral with a hard 3px ink shadow; secondary actions are underlined text
or paper buttons. Pressing a control physically removes its shadow and shifts
it 2px. The live demo behaves like a review desk: editing the config or its
rationale immediately re-stamps the local result; no data leaves the browser.
At 390px, collage layers flatten, ornamental registration marks disappear, and
the demo becomes one vertical reading order.

## Motion policy

Only state changes move. Sheets settle upward by 8px over 220ms on entry;
buttons depress over 120ms; the demo stamp scales once from 0.96 to 1. Motion
uses only transform and opacity. There is no looping animation. Under
`prefers-reduced-motion: reduce`, transitions and transforms are removed and
state changes are conveyed instantly through text, borders, and status icons.

## Asset plan and provenance

The hero uses one original raster illustration, `site/public/rationale-press.webp`:
a top-down risograph collage of a strict config sheet aligned with a handwritten
rationale sheet inside a review press. It contains no legible generated text;
real labels remain HTML. Generated specifically for this product with the
factory `gen-image.sh` deployment on 2026-08-27, then locally converted to WebP.
Final prompt and deployment metadata live beside the source during generation
and are copied to `site/public/rationale-press.provenance.json`. The image is
licensed under the repository's MIT license. All small marks and icons are
hand-authored CSS/HTML shapes, not third-party assets.
