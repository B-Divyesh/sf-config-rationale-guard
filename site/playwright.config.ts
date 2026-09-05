import { defineConfig } from '@playwright/test';
import { fileURLToPath } from 'node:url';
import { resolve } from 'node:path';

const repoRoot = resolve(fileURLToPath(new URL('..', import.meta.url)));

export default defineConfig({
  testDir: resolve(repoRoot, 'site/tests'),
  fullyParallel: false,
  reporter: 'list',
  use: {
    baseURL: 'http://127.0.0.1:4173',
    browserName: 'chromium',
    viewport: { width: 1280, height: 900 }
  },
  webServer: {
    command: 'npm run build:site && node scripts/serve-site.mjs',
    cwd: repoRoot,
    url: 'http://127.0.0.1:4173/',
    reuseExistingServer: false,
    timeout: 120_000
  }
});
