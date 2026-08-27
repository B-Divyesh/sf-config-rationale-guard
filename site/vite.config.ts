import { defineConfig } from 'vite';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const siteRoot = fileURLToPath(new URL('.', import.meta.url));

export default defineConfig({
  root: siteRoot,
  publicDir: resolve(siteRoot, 'public'),
  build: {
    outDir: resolve(siteRoot, '../dist/site'),
    emptyOutDir: true,
    target: 'es2022',
    sourcemap: true,
    rollupOptions: {
      input: {
        home: resolve(siteRoot, 'index.html'),
        privacy: resolve(siteRoot, 'privacy/index.html'),
        terms: resolve(siteRoot, 'terms/index.html')
      }
    }
  }
});
