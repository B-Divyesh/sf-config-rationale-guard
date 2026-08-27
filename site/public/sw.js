const CACHE = 'crg-shell-v2';
const SHELL = ['/', '/privacy/', '/terms/', '/rationale-press.webp', '/fraunces-latin-600.woff2', '/mark.svg'];
self.addEventListener('install', event => {
  event.waitUntil(caches.open(CACHE).then(cache => cache.addAll(SHELL)));
  self.skipWaiting();
});
self.addEventListener('activate', event => {
  event.waitUntil(caches.keys().then(keys => Promise.all(keys.filter(key => key !== CACHE).map(key => caches.delete(key)))));
  self.clients.claim();
});
self.addEventListener('fetch', event => {
  if (event.request.method !== 'GET') return;
  event.respondWith(caches.match(event.request).then(cached => cached || fetch(event.request).then(response => {
    if (response.ok && new URL(event.request.url).origin === location.origin) {
      caches.open(CACHE).then(cache => cache.put(event.request, response.clone()));
    }
    return response;
  }).catch(() => {
    if (event.request.mode === 'navigate') return caches.match('/');
    return new Response('', { status: 204, statusText: 'Offline' });
  })));
});
