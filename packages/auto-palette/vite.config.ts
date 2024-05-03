import { resolve } from 'node:path';
import { defineConfig } from 'vitest/config';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';
import { viteStaticCopy } from 'vite-plugin-static-copy';

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
  plugins: [
    viteStaticCopy({
      targets: [{ src: '../../crates/wasm/pkg/index_bg.wasm', dest: '.' }],
    }),
    wasm(),
    topLevelAwait({
      promiseExportName: 'init',
    }),
  ],
  test: {
    globals: true,
    dir: 'test',
    include: ['**/*.test.ts'],
    testTimeout: 1000,
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
