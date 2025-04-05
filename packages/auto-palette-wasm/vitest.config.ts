import { resolve } from 'node:path';
import wasm from 'vite-plugin-wasm';
import { defineConfig } from 'vitest/config';

export default defineConfig({
  plugins: [wasm()],
  test: {
    globals: true,
    dir: 'test',
    include: ['**/*.test.ts'],
    setupFiles: ['test/setup.ts'],
    alias: {
      '@auto-palette/core': resolve(
        __dirname,
        '../../crates/auto-palette-wasm/dist',
      ),
      '@auto-palette/wasm': resolve(__dirname, 'src/index.ts'),
    },
    environment: 'node',
    testTimeout: 5000,
    retry: 0,
  },
});
