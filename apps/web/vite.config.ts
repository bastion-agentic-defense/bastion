import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  define: {
    global: 'globalThis',
  },
  optimizeDeps: {
    esbuildOptions: {
      define: {
        global: 'globalThis',
      },
    },
  },
  resolve: {
    alias: {
      buffer: 'buffer/',
      '@': path.resolve(__dirname, './src'),
    },
  },
  server: {
    port: 3000,
    host: true,
    fs: {
      // The docs pages import `docs/*.md` from the repository root as the single
      // source of truth, which sits outside this app's Vite root.
      allow: [path.resolve(__dirname, '../..')],
    },
  },
});
