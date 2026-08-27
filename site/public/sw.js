const CACHE = 'crg-shell-v4';
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
self.addEventListener('fetch', event => {
  if (event.request.method !== 'GET') return;
  event.respondWith((async () => {
    const cached = await caches.match(event.request);
    if (cached) return cached;
    try {
      const response = await fetch(event.request);
      if (response.ok && new URL(event.request.url).origin === location.origin) {
        caches.open(CACHE).then(cache => cache.put(event.request, response.clone()));
      }
      return response;
    } catch {
      if (event.request.mode === 'navigate') return caches.match('/');
      return new Response('', { status: 204, statusText: 'Offline' });
    }
  })());
});
