import { resolve } from 'node:path';
import { playwright } from '@vitest/browser-playwright';
import wasm from 'vite-plugin-wasm';
import { defineConfig } from 'vitest/config';

export default defineConfig({
  plugins: [wasm()],
  test: {
    globals: true,
    dir: 'test',
    alias: {
      '@auto-palette/core': resolve(__dirname, '../../crates/auto-palette-wasm/dist'),
      '@auto-palette/wasm': resolve(__dirname, 'src/index.ts'),
    },
    coverage: {
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
    projects: [
      {
        extends: true,
        test: {
          name: 'unit',
          environment: 'node',
          include: ['**/*.test.ts'],
          setupFiles: ['test/setup.ts', 'test/matchers/index.ts'],
          benchmark: {
            include: [],
          },
          testTimeout: 1000,
          retry: 0,
        },
      },
      {
        extends: true,
        test: {
          name: 'e2e:browser',
          include: ['**/*.browser.e2e.ts'],
          setupFiles: ['test/matchers/index.ts'],
          benchmark: {
            include: [],
          },
          browser: {
            enabled: true,
            headless: true,
            provider: playwright(),
            instances: [
              {
                browser: 'chromium',
                screenshotFailures: false,
              },
            ],
          },
          testTimeout: 600_000,
          retry: 0,
        },
      },
      {
        extends: true,
        test: {
          name: 'e2e:node',
          environment: 'node',
          include: ['**/*.node.e2e.ts'],
          setupFiles: ['test/setup.ts', 'test/matchers/index.ts'],
          benchmark: {
            include: ['**/*.bench.ts'],
          },
          testTimeout: 600_000,
          retry: 1,
        },
      },
    ],
  },
});
