import { createReadStream, existsSync } from 'node:fs';
import { stat } from 'node:fs/promises';
import { createServer } from 'node:http';
import { extname, resolve } from 'node:path';

const root = resolve('dist/site');
const port = Number(process.env.PORT || 4173);
const mime = {
  '.css': 'text/css; charset=utf-8',
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
  '.svg': 'image/svg+xml',
  '.png': 'image/png',
  '.webp': 'image/webp',
  '.woff2': 'font/woff2',
  '.xml': 'application/xml; charset=utf-8',
  '.txt': 'text/plain; charset=utf-8'
};
const routeFiles = new Map([
  ['/', '/index.html'],
  ['/demo', '/demo/index.html'],
  ['/demo/', '/demo/index.html'],
  ['/privacy', '/privacy/index.html'],
  ['/privacy/', '/privacy/index.html'],
  ['/terms', '/terms/index.html'],
  ['/terms/', '/terms/index.html']
]);

function fileFor(pathname) {
  const mapped = routeFiles.get(pathname) || pathname;
  const candidate = resolve(root, `.${mapped}`);
  if (!candidate.startsWith(`${root}/`) && candidate !== root) return null;
  return candidate;
}

createServer(async (request, response) => {
  const url = new URL(request.url || '/', `http://${request.headers.host || '127.0.0.1'}`);
  let file = fileFor(decodeURIComponent(url.pathname));
  let status = 200;
  try {
    if (!file || !existsSync(file) || (await stat(file)).isDirectory()) throw new Error('not found');
  } catch {
    file = resolve(root, '404.html');
    status = 404;
  }
  response.writeHead(status, {
    'Content-Type': mime[extname(file)] || 'application/octet-stream',
    'Cache-Control': file.endsWith('/sw.js') ? 'no-cache' : 'no-store',
    'X-Content-Type-Options': 'nosniff'
  });
  createReadStream(file).pipe(response);
}).listen(port, '127.0.0.1', () => {
  process.stdout.write(`Serving ${root} at http://127.0.0.1:${port}\n`);
});
