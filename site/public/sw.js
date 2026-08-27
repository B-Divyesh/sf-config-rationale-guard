const CACHE = 'crg-shell-v2';
const STATIC_SHELL = ['/', '/privacy/', '/terms/', '/rationale-press.webp', '/mark.svg', '/fraunces-latin-600.woff2'];
self.addEventListener('install', event => {
  event.waitUntil((async () => {
    const response = await fetch('/shell-assets.json', { cache: 'no-store' });
    const { assets = [] } = await response.json();
    const cache = await caches.open(CACHE);
    await cache.addAll([...STATIC_SHELL, '/shell-assets.json', ...assets]);
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
    } catch (error) {
      if (event.request.mode === 'navigate') return caches.match('/');
      throw error;
    }
  })());
});
