import { describe, expect, it } from 'vitest';
import { check, fingerprint, patternMatches, pointer, stamp } from './checker';

describe('browser decision checker', () => {
  it('matches pointer globs without exposing values', () => {
    expect(patternMatches('/permissions/**', '/permissions/shell')).toBe(true);
    expect(patternMatches('/permissions/*', '/permissions/nested/shell')).toBe(false);
    expect(pointer({ 'a/b': { '~key': true } }, '/a~1b/~0key')).toEqual({ found: true, value: true });
  });

  it('detects stale decisions and passes after an explicit stamp', async () => {
    const config = { permissions: { shell: false } };
    const sidecar = {
      rules: [{ pattern: '/permissions/**' }],
      decisions: [{ path: '/permissions/shell', rationale: 'Disabled for untrusted tasks.', valueHash: await fingerprint(true) }]
    };
    expect((await check(config, sidecar)).findings.map(item => item.code)).toContain('stale');
    await stamp(config, sidecar);
    expect((await check(config, sidecar)).valid).toBe(true);
  });

  it('reports empty config as a first-class state', async () => {
    expect((await check({}, { decisions: [] })).empty).toBe(true);
  });
});
