import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { describe, expect, it, vi } from 'vitest';

type Listener = (event: any) => void;

async function loadWorker(options: { homepage?: string; cacheMatch?: Response } = {}) {
  const source = await readFile(resolve(process.cwd(), 'site/public/sw.js'), 'utf8');
  const listeners = new Map<string, Listener>();
  const cache = {
    addAll: vi.fn().mockResolvedValue(undefined),
    put: vi.fn().mockResolvedValue(undefined)
  };
  const cacheApi = {
    open: vi.fn().mockResolvedValue(cache),
    match: vi.fn().mockResolvedValue(options.cacheMatch),
    keys: vi.fn().mockResolvedValue([]),
    delete: vi.fn().mockResolvedValue(true)
  };
  const fetchMock = vi.fn(async (input: RequestInfo | URL) => {
    const url = typeof input === 'string' ? input : input.toString();
    if (url === '/' && options.homepage !== undefined) return new Response(options.homepage);
    throw new TypeError('network unavailable');
  });
  const scope = {
    addEventListener: (type: string, listener: Listener) => listeners.set(type, listener),
    skipWaiting: vi.fn(),
    clients: { claim: vi.fn() },
    location: { origin: 'https://guard.test' }
  };

  new Function('self', 'caches', 'fetch', 'Response', 'URL', source)(scope, cacheApi, fetchMock, Response, URL);
  return { listeners, cache, cacheApi, fetchMock };
}

describe('offline shell worker', () => {
  it('precaches hashed JS and CSS emitted by the HTML shell', async () => {
    const worker = await loadWorker({
      homepage: '<script src="/assets/main-a1.js"></script><link href="/assets/main-b2.css">'
    });
    let installed: Promise<unknown> | undefined;
    worker.listeners.get('install')?.({ waitUntil: (promise: Promise<unknown>) => { installed = promise; } });
    await installed;

    expect(worker.cache.addAll).toHaveBeenCalledWith(expect.arrayContaining([
      '/assets/main-a1.js',
      '/assets/main-b2.css',
      '/fraunces-latin-600.woff2'
    ]));
  });

  it('returns a successful empty response when an uncached offline subresource fails', async () => {
    const worker = await loadWorker();
    let response: Promise<Response> | undefined;
    worker.listeners.get('fetch')?.({
      request: new Request('https://guard.test/assets/previous-release.css'),
      respondWith: (promise: Promise<Response>) => { response = promise; }
    });

    expect(response).toBeDefined();
    const recovered = await response!;
    expect(recovered.status).toBe(200);
    expect(recovered.headers.get('Content-Type')).toContain('text/css');
    await expect(recovered.text()).resolves.toBe('');
  });
});
