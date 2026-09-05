import { expect, test } from '@playwright/test';

test.describe('public demo and site claims', () => {
  /** @claim:one-click-sample */
  test('@claim:one-click-sample opens a populated sample from the first action', async ({ page }) => {
    await page.goto('/');
    await page.getByRole('link', { name: 'Try it with sample data' }).click();
    await expect(page).toHaveURL(/\/demo$/);
    await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
    await expect(page.locator('#config-input')).toHaveValue(/"network": "allowlist"/);
    await expect(page.locator('#rationale-input')).toHaveValue(/Shell stays off for untrusted tasks\./);
  });

  /** @claim:demo-sandbox */
  test('@claim:demo-sandbox keeps sample edits in a demo-only storage namespace', async ({ page }) => {
    await page.goto('/');
    await page.evaluate(() => localStorage.setItem('crg:real-project', 'leave-this-alone'));
    await page.goto('/demo');
    await page.locator('#config-input').fill('{"permissions":{"shell":true,"network":"allowlist"},"retries":2}');
    await page.getByRole('button', { name: 'Stamp reviewed values' }).click();
    await expect(page.getByText('Reviewed values stamped locally.')).toBeVisible();
    const storage = await page.evaluate(() => Object.fromEntries(Object.keys(localStorage).map(key => [key, localStorage.getItem(key)])));
    expect(storage['crg:real-project']).toBe('leave-this-alone');
    expect(JSON.parse(storage['demo:config-rationale-guard:checker'] || '{}').config).toContain('"shell":true');
    await page.getByRole('link', { name: 'Start for real' }).click();
    await expect(page).toHaveURL(/\/$/);
    const after = await page.evaluate(() => ({ real: localStorage.getItem('crg:real-project'), demo: localStorage.getItem('demo:config-rationale-guard:checker') }));
    expect(after).toEqual({ real: 'leave-this-alone', demo: null });
  });

  /** @claim:demo-reset */
  test('@claim:demo-reset resets the sample after a realistic check', async ({ page }) => {
    await page.goto('/demo');
    await page.getByRole('button', { name: 'Stamp reviewed values' }).click();
    await page.getByRole('button', { name: 'Run local check' }).click();
    await expect(page.getByText('Decision trail is current.')).toBeVisible();
    await page.locator('#config-input').fill('{"permissions":{"shell":true},"retries":2}');
    await page.getByRole('button', { name: 'Reset demo' }).click();
    await expect(page.getByText('Sample reset.')).toBeVisible();
    await expect(page.locator('#config-input')).toHaveValue(/"network": "allowlist"/);
    await expect(page.locator('#rationale-input')).toHaveValue(/"valueHash": ""/);
    await expect(page.locator('#demo-result')).toContainText('Stamp the sample, then run the local check.');
  });

  /** @claim:browser-local */
  test('@claim:browser-local checks the browser sample without sending it to another origin', async ({ page, context }) => {
    const requests: string[] = [];
    page.on('request', request => requests.push(request.url()));
    await page.goto('/demo');
    await page.getByRole('button', { name: 'Stamp reviewed values' }).click();
    await page.getByRole('button', { name: 'Run local check' }).click();
    await expect(page.getByText('Decision trail is current.')).toBeVisible();
    const origins = new Set(requests.map(request => new URL(request).origin));
    expect([...origins]).toEqual([new URL(page.url()).origin]);
    await expect(context.pages()).toHaveLength(1);
  });

  test('serves direct routes and a designed HTTP 404', async ({ page }) => {
    const demo = await page.goto('/demo');
    expect(demo?.status()).toBe(200);
    await expect(page).toHaveTitle('Demo — Config Rationale Guard');
    await expect(page.locator('h1')).toHaveCount(1);
    const missing = await page.goto('/a-page-that-does-not-exist');
    expect(missing?.status()).toBe(404);
    await expect(page).toHaveTitle('Page not found — Config Rationale Guard');
    await expect(page.getByRole('heading', { name: 'This page does not exist.' })).toBeVisible();
  });
});
