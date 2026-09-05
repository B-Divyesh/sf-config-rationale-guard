import './style.css';
import { check, stamp } from './checker';

const PRODUCT = 'config-rationale-guard';
const DEMO_KEY = `demo:${PRODUCT}:checker`;

const byId = <T extends HTMLElement>(id: string): T | null => document.getElementById(id) as T | null;

function parseJson(text: string, label: string): unknown {
  try {
    return JSON.parse(text);
  } catch (error) {
    const message = error instanceof Error ? error.message : 'Invalid JSON';
    throw new Error(`${label}: ${message}`);
  }
}

function storageGet(key: string): string | null {
  try { return localStorage.getItem(key); } catch { return null; }
}

function storageSet(key: string, value: string) {
  try { localStorage.setItem(key, value); } catch { /* Browser privacy mode can reject storage. */ }
}

function storageRemove(key: string) {
  try { localStorage.removeItem(key); } catch { /* Browser privacy mode can reject storage. */ }
}

function renderDemo(kind: 'ready' | 'pass' | 'fail' | 'empty' | 'error', title: string, detail: string) {
  const panel = byId<HTMLDivElement>('demo-result');
  if (!panel) return;
  const labels = { ready: 'READY', pass: 'PASS', fail: 'CHECK', empty: 'EMPTY', error: 'ERROR' };
  panel.dataset.state = kind;
  panel.replaceChildren();
  const stampLabel = document.createElement('span');
  stampLabel.className = 'result-stamp';
  stampLabel.setAttribute('aria-hidden', 'true');
  stampLabel.textContent = labels[kind];
  const copy = document.createElement('div');
  const strong = document.createElement('strong');
  strong.textContent = title;
  const paragraph = document.createElement('p');
  paragraph.textContent = detail;
  copy.append(strong, paragraph);
  panel.append(stampLabel, copy);
  panel.focus({ preventScroll: true });
}

function isDemoPage() {
  return document.body.dataset.demo === 'true';
}

function loadDemoInput(configInput: HTMLTextAreaElement, rationaleInput: HTMLTextAreaElement) {
  if (!isDemoPage()) return;
  const saved = storageGet(DEMO_KEY);
  if (!saved) return;
  try {
    const values = JSON.parse(saved) as { config?: unknown; rationale?: unknown };
    if (typeof values.config === 'string') configInput.value = values.config;
    if (typeof values.rationale === 'string') rationaleInput.value = values.rationale;
  } catch {
    storageRemove(DEMO_KEY);
  }
}

function saveDemoInput(configInput: HTMLTextAreaElement, rationaleInput: HTMLTextAreaElement) {
  if (!isDemoPage()) return;
  storageSet(DEMO_KEY, JSON.stringify({ config: configInput.value, rationale: rationaleInput.value }));
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
    renderDemo('error', 'The sample could not be parsed.', error instanceof Error ? error.message : 'Check the JSON syntax and try again.');
  }
}

function setupDemo() {
  const form = byId<HTMLFormElement>('demo-form');
  const configInput = byId<HTMLTextAreaElement>('config-input');
  const rationaleInput = byId<HTMLTextAreaElement>('rationale-input');
  if (!form || !configInput || !rationaleInput) return;
  const originals = { config: configInput.value, rationale: rationaleInput.value };
  loadDemoInput(configInput, rationaleInput);
  const save = () => saveDemoInput(configInput, rationaleInput);
  configInput.addEventListener('input', save);
  rationaleInput.addEventListener('input', save);
  form.addEventListener('submit', event => {
    event.preventDefault();
    save();
    void runDemo();
  });
  byId<HTMLButtonElement>('stamp-demo')?.addEventListener('click', async () => {
    try {
      const config = parseJson(configInput.value, 'Config');
      const rationale = parseJson(rationaleInput.value, 'Rationale file') as Record<string, unknown>;
      rationaleInput.value = JSON.stringify(await stamp(config, rationale), null, 2);
      save();
      renderDemo('ready', 'Reviewed values stamped locally.', 'Run the local check to verify targets and coverage.');
    } catch (error) {
      renderDemo('error', 'The sample could not be stamped.', error instanceof Error ? error.message : 'Check the JSON syntax and try again.');
    }
  });
  byId<HTMLButtonElement>('reset-demo')?.addEventListener('click', () => {
    configInput.value = originals.config;
    rationaleInput.value = originals.rationale;
    storageRemove(DEMO_KEY);
    renderDemo('ready', 'Sample reset.', 'Stamp the sample, then run the local check.');
  });
  byId<HTMLAnchorElement>('leave-demo')?.addEventListener('click', () => storageRemove(DEMO_KEY));
}

async function copyText(text: string, button: HTMLButtonElement) {
  const original = button.textContent || 'Copy';
  try {
    await navigator.clipboard.writeText(text);
  } catch {
    const area = document.createElement('textarea');
    area.value = text;
    area.style.position = 'fixed';
    area.style.opacity = '0';
    document.body.append(area);
    area.select();
    document.execCommand('copy');
    area.remove();
  }
  button.textContent = 'Copied';
  window.setTimeout(() => (button.textContent = original), 1800);
}

function setupCopyButtons() {
  document.querySelectorAll<HTMLButtonElement>('[data-copy]').forEach(button => {
    button.addEventListener('click', () => void copyText(button.dataset.copy || '', button));
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
setupNetworkState();

if ('serviceWorker' in navigator && import.meta.env.PROD) {
  window.setTimeout(() => void navigator.serviceWorker.register('/sw.js'), 0);
}
