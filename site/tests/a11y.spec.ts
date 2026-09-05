import AxeBuilder from '@axe-core/playwright';
import { expect, test } from '@playwright/test';

const routes = ['/', '/demo', '/privacy/', '/terms/', '/a-page-that-does-not-exist'];

test('public pages have one heading, a main landmark, no console errors, and no serious axe violations', async ({ page }) => {
  const consoleErrors: string[] = [];
  page.on('console', message => {
    if (message.type() === 'error') consoleErrors.push(message.text());
  });

  for (const route of routes) {
    const errorsBefore = consoleErrors.length;
    await page.goto(route);
    await expect(page.locator('html')).toHaveAttribute('lang', 'en');
    await expect(page).not.toHaveTitle('');
    await expect(page.locator('main')).toHaveCount(1);
    await expect(page.locator('h1')).toHaveCount(1);
    const results = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa']).analyze();
    expect(results.violations.filter(violation => ['serious', 'critical'].includes(violation.impact || ''))).toEqual([]);
    if (route !== '/a-page-that-does-not-exist') expect(consoleErrors).toHaveLength(errorsBefore);
  }
});

test('phone layout has no horizontal overflow and respects reduced motion', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.emulateMedia({ reducedMotion: 'reduce' });
  for (const route of routes) {
    await page.goto(route);
    const measurements = await page.evaluate(() => ({
      documentWidth: document.documentElement.scrollWidth,
      bodyWidth: document.body.scrollWidth,
      viewport: document.documentElement.clientWidth,
      transition: getComputedStyle(document.querySelector('.button') || document.body).transitionDuration
    }));
    expect(measurements.documentWidth).toBeLessThanOrEqual(measurements.viewport);
    expect(measurements.bodyWidth).toBeLessThanOrEqual(measurements.viewport);
    expect(Number.parseFloat(measurements.transition)).toBeLessThanOrEqual(0.01);
  }
});

test('a service-worker-controlled demo reload stays usable offline', async ({ browser }) => {
  const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
  const page = await context.newPage();
  const errors: string[] = [];
  page.on('pageerror', error => errors.push(String(error)));
  page.on('console', message => {
    if (message.type() === 'error') errors.push(message.text());
  });
  await page.goto('/demo');
  await page.waitForFunction(() => navigator.serviceWorker?.controller !== null);
  await page.reload();
  await context.setOffline(true);
  await page.reload();
  await expect(page.getByText('Offline. This sample still checks in this tab.')).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Check the shipped sample config.' })).toBeVisible();
  expect(errors).toEqual([]);
  await context.close();
});
