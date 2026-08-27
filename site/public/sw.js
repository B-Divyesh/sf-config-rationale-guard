// Bump this whenever the shell contract changes so existing clients cannot
// retain a worker that rethrows offline subresource failures.
const CACHE = 'crg-shell-v5';
const STATIC_SHELL = ['/privacy/', '/terms/', '/rationale-press.webp', '/mark.svg', '/fraunces-latin-600.woff2'];
self.addEventListener('install', event => {
  event.waitUntil((async () => {
    const homepage = await fetch('/', { cache: 'no-store' });
    const html = await homepage.clone().text();
    const assets = [...html.matchAll(/(?:src|href)="(\/assets\/[^"?]+)"/g)]
      .map((match) => match[1]);
    const cache = await caches.open(CACHE);
    await cache.put('/', homepage);
    await cache.addAll([...STATIC_SHELL, ...assets]);
  })());
  self.skipWaiting();
});
self.addEventListener('activate', event => {
  event.waitUntil(caches.keys().then(keys => Promise.all(keys.filter(key => key !== CACHE).map(key => caches.delete(key)))));
  self.clients.claim();
});

function offlineSubresource(url) {
  const path = url.pathname;
  const type = path.endsWith('.css') ? 'text/css' :
    path.endsWith('.js') ? 'application/javascript' :
    path.endsWith('.svg') ? 'image/svg+xml' :
    'text/plain';
  // A successful empty response keeps a stale, uncached subresource from
  // becoming a browser-level failed request. The current shell is precached,
  // so this is a last-resort recovery path rather than normal rendering.
  return new Response('', { status: 200, headers: { 'Content-Type': type } });
}

self.addEventListener('fetch', event => {
  if (event.request.method !== 'GET') return;
  event.respondWith((async () => {
    const cached = await caches.match(event.request);
    if (cached) return cached;
    try {
      const response = await fetch(event.request);
      if (response.ok && new URL(event.request.url).origin === location.origin) {
        void caches.open(CACHE)
          .then(cache => cache.put(event.request, response.clone()))
          .catch(() => undefined);
      }
      return response;
    } catch {
      if (event.request.mode === 'navigate') return caches.match('/');
      return offlineSubresource(new URL(event.request.url));
    }
  })());
});
