import { resolve } from 'node:path';
import wasm from 'vite-plugin-wasm';
import { defineConfig } from 'vitest/config';

export default defineConfig({
  plugins: [wasm()],
  test: {
    globals: true,
    dir: 'test',
    alias: {
      '@auto-palette/core': resolve(
        __dirname,
        '../../crates/auto-palette-wasm/dist',
      ),
      '@auto-palette/wasm': resolve(__dirname, 'src/index.ts'),
    },
    coverage: {
      all: false,
      provider: 'v8',
      include: ['src/**/*.ts'],
      exclude: ['**/*.test.ts', '**/*.d.ts'],
      reporter: ['lcov', 'html', 'json', 'text'],
      reportsDirectory: 'coverage',
      thresholds: {
        statements: 95,
        branches: 95,
        functions: 95,
        lines: 95,
      },
    },
  },
});
