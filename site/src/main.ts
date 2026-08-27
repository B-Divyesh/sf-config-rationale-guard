import './style.css';
import { check, stamp } from './checker';

const PRODUCT = 'config-rationale-guard';
const API = 'https://api.sociobot.in/api/v1';
const LICENSE_KEY = `sb_license:${PRODUCT}`;
const VERDICT_KEY = `sb_license_verdict:${PRODUCT}`;
const DAY = 86_400_000;

const byId = <T extends HTMLElement>(id: string): T | null => document.getElementById(id) as T | null;

function parseJson(text: string, label: string): unknown {
  try {
    return JSON.parse(text);
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Invalid JSON';
    throw new Error(`${label}: ${message}`);
  }
}

function renderDemo(kind: 'ready' | 'pass' | 'fail' | 'empty' | 'error', title: string, detail: string) {
  const panel = byId<HTMLDivElement>('demo-result');
  if (!panel) return;
  const labels = { ready: 'READY', pass: 'PASS', fail: 'CHECK', empty: 'EMPTY', error: 'ERROR' };
  panel.dataset.state = kind;
  panel.replaceChildren();
  const stamp = document.createElement('span');
  stamp.className = 'result-stamp';
  stamp.setAttribute('aria-hidden', 'true');
  stamp.textContent = labels[kind];
  const copy = document.createElement('div');
  const strong = document.createElement('strong');
  strong.textContent = title;
  const paragraph = document.createElement('p');
  paragraph.textContent = detail;
  copy.append(strong, paragraph);
  panel.append(stamp, copy);
  panel.focus({ preventScroll: true });
}

async function runDemo() {
  const configInput = byId<HTMLTextAreaElement>('config-input');
  const rationaleInput = byId<HTMLTextAreaElement>('rationale-input');
  if (!configInput || !rationaleInput) return;
  try {
    const config = parseJson(configInput.value, 'Config');
    const rationale = parseJson(rationaleInput.value, 'Rationale file') as Record<string, unknown>;
    const result = await check(config, rationale);
    if (result.empty) {
      renderDemo('empty', 'No settings to review.', 'Add a config setting, then create a decision that targets its JSON Pointer path.');
    } else if (result.valid) {
      renderDemo('pass', 'Decision trail is current.', `${result.decisions} decision record(s); all required paths are covered. No values were included in this result.`);
    } else {
      const first = result.findings[0];
      renderDemo('fail', `${result.findings.length} finding(s) need attention.`, `${first.path || '/'} — ${first.message}`);
    }
  } catch (error) {
    renderDemo('error', 'The example could not be parsed.', error instanceof Error ? error.message : 'Check the JSON syntax and try again.');
  }
}

function setupDemo() {
  const form = byId<HTMLFormElement>('demo-form');
  const configInput = byId<HTMLTextAreaElement>('config-input');
  const rationaleInput = byId<HTMLTextAreaElement>('rationale-input');
  if (!form || !configInput || !rationaleInput) return;
  const originals = { config: configInput.value, rationale: rationaleInput.value };
  form.addEventListener('submit', event => {
    event.preventDefault();
    void runDemo();
  });
  byId<HTMLButtonElement>('stamp-demo')?.addEventListener('click', async () => {
    try {
      const config = parseJson(configInput.value, 'Config');
      const rationale = parseJson(rationaleInput.value, 'Rationale file') as Record<string, unknown>;
      rationaleInput.value = JSON.stringify(await stamp(config, rationale), null, 2);
      renderDemo('ready', 'Reviewed values stamped locally.', 'Run the local check to verify targets and coverage.');
    } catch (error) {
      renderDemo('error', 'The example could not be stamped.', error instanceof Error ? error.message : 'Check the JSON syntax and try again.');
    }
  });
  byId<HTMLButtonElement>('reset-demo')?.addEventListener('click', () => {
    configInput.value = originals.config;
    rationaleInput.value = originals.rationale;
    renderDemo('ready', 'Example reset.', 'Choose “Stamp reviewed values” and then run the local check.');
  });
}

async function copyText(text: string, button: HTMLButtonElement) {
  const original = button.textContent || 'Copy';
  try {
    await navigator.clipboard.writeText(text);
    button.textContent = 'Copied';
  } catch {
    const area = document.createElement('textarea');
    area.value = text;
    area.style.position = 'fixed';
    area.style.opacity = '0';
    document.body.append(area);
    area.select();
    document.execCommand('copy');
    area.remove();
    button.textContent = 'Copied';
  }
  window.setTimeout(() => (button.textContent = original), 1800);
}

function setupCopyButtons() {
  document.querySelectorAll<HTMLButtonElement>('[data-copy]').forEach(button => {
    button.addEventListener('click', () => void copyText(button.dataset.copy || '', button));
  });
  const workflow = `name: Config rationale\non: [pull_request]\njobs:\n  guard:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n      - uses: dtolnay/rust-toolchain@stable\n      - run: cargo install --path cli\n      - run: crg check config/agent.json --schema config/agent.schema.json\n      - run: crg diff base/agent.json config/agent.json --json > decision-report.json`;
  const policy = JSON.stringify({ version: 1, rules: [
    { pattern: '/permissions/**', minimumCoverage: 1 },
    { pattern: '/sandbox/**', minimumCoverage: 1 },
    { pattern: '/tools/*/enabled', minimumCoverage: 0.9 }
  ], decisions: [] }, null, 2);
  const recipeButton = byId<HTMLButtonElement>('copy-team-recipe');
  recipeButton?.addEventListener('click', () => void copyText(workflow, recipeButton));
  const policyButton = byId<HTMLButtonElement>('copy-team-policy');
  policyButton?.addEventListener('click', () => void copyText(policy, policyButton));
}

type Verdict = { valid: boolean; checkedAt: number; reason?: string };

function storageGet(key: string): string | null {
  try { return localStorage.getItem(key); } catch { return null; }
}

function storageSet(key: string, value: string) {
  try { localStorage.setItem(key, value); } catch { /* Private mode can reject storage. */ }
}

function readVerdict(): Verdict | null {
  try {
    const value = storageGet(VERDICT_KEY);
    return value ? JSON.parse(value) as Verdict : null;
  } catch { return null; }
}

function setUnlocked(unlocked: boolean, message: string) {
  const content = byId<HTMLElement>('unlock-content');
  const status = byId<HTMLElement>('license-status');
  if (content) content.hidden = !unlocked;
  if (status) status.textContent = message;
}

async function verifyLicense(token: string) {
  setUnlocked(false, 'Checking license…');
  try {
    const response = await fetch(`${API}/products/${PRODUCT}/verify?license=${encodeURIComponent(token)}`, { headers: { Accept: 'application/json' } });
    if (!response.ok) throw new Error(`Verification returned ${response.status}`);
    const result = await response.json() as { valid?: boolean; reason?: string };
    const verdict: Verdict = { valid: result.valid === true, reason: result.reason, checkedAt: Date.now() };
    storageSet(VERDICT_KEY, JSON.stringify(verdict));
    if (verdict.valid) {
      setUnlocked(true, 'Team license active on this device.');
    } else {
      setUnlocked(false, 'License no longer active. You can check the token or buy a new license.');
    }
  } catch {
    const cached = readVerdict();
    if (cached?.valid) setUnlocked(true, 'Offline verification unavailable; using your last valid license result.');
    else setUnlocked(false, 'Could not verify right now. The free CLI and local demo remain available.');
  }
}

function setupLicense() {
  const url = new URL(window.location.href);
  const returned = url.searchParams.get('license');
  if (returned) {
    storageSet(LICENSE_KEY, returned);
    url.searchParams.delete('license');
    history.replaceState({}, '', `${url.pathname}${url.search}${url.hash}`);
  }
  const token = returned || storageGet(LICENSE_KEY);
  const verdict = readVerdict();
  if (token && verdict?.valid) setUnlocked(true, 'Team license active on this device.');
  if (token && (!verdict || Date.now() - verdict.checkedAt >= DAY || returned)) void verifyLicense(token);

  byId<HTMLFormElement>('license-form')?.addEventListener('submit', event => {
    event.preventDefault();
    const input = byId<HTMLInputElement>('license-input');
    const nextToken = input?.value.trim();
    if (!nextToken) return;
    storageSet(LICENSE_KEY, nextToken);
    storageSet(VERDICT_KEY, '');
    if (input) input.value = '';
    void verifyLicense(nextToken);
  });
}

function setupNetworkState() {
  const note = byId<HTMLElement>('offline-note');
  if (!note) return;
  const update = () => (note.hidden = navigator.onLine);
  window.addEventListener('online', update);
  window.addEventListener('offline', update);
  update();
}

setupDemo();
setupCopyButtons();
setupLicense();
setupNetworkState();

if ('serviceWorker' in navigator && import.meta.env.PROD) {
  window.setTimeout(() => void navigator.serviceWorker.register('/sw.js'), 0);
}
