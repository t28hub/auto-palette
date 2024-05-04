import { resolve } from 'node:path';
import { defineConfig } from 'vitest/config';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  build: {
    lib: {
      name: 'AutoPalette',
      entry: resolve(__dirname, 'src/index.ts'),
      formats: ['cjs', 'es', 'umd'],
      fileName: 'index',
    },
    minify: false,
    sourcemap: true,
  },
  plugins: [wasm(), topLevelAwait()],
  test: {
    globals: true,
    dir: 'test',
    include: ['**/*.test.ts'],
    testTimeout: 5000,
    browser: {
      enabled: true,
      name: 'chromium',
      provider: 'playwright',
      headless: true,
    },
    coverage: {
      all: false,
      provider: 'v8',
      include: ['src/**/*'],
      exclude: ['**/*.test.ts', '**/*.d.ts'],
      reporter: ['lcov', 'html', 'json'],
      reportsDirectory: 'coverage',
    },
  },
});
