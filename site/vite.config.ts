import { defineConfig, type Plugin } from 'vite';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const siteRoot = fileURLToPath(new URL('.', import.meta.url));

/** Precache the hashed build assets so a controlled offline reload is quiet. */
function offlineShell(): Plugin {
  return {
    name: 'crg-offline-shell',
    generateBundle(_, bundle) {
      const assets = Object.keys(bundle)
        .filter((file) => /\.(?:css|js)$/.test(file))
        .map((file) => `/${file}`);
      this.emitFile({
        type: 'asset',
        fileName: 'shell-assets.json',
        source: `${JSON.stringify({ assets }, null, 2)}\n`
      });
    }
  };
}

export default defineConfig({
  root: siteRoot,
  publicDir: resolve(siteRoot, 'public'),
  build: {
    outDir: resolve(siteRoot, '../dist/site'),
    emptyOutDir: true,
    target: 'es2022',
    sourcemap: false,
    rollupOptions: {
      input: {
        home: resolve(siteRoot, 'index.html'),
        privacy: resolve(siteRoot, 'privacy/index.html'),
        terms: resolve(siteRoot, 'terms/index.html')
      }
    }
  },
  plugins: [offlineShell()]
});
