import { defineWorkspace } from 'vitest/config';

export default defineWorkspace([
  {
    extends: './vitest.config.ts',
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
    extends: './vitest.config.ts',
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
        provider: 'playwright',
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
    extends: './vitest.config.ts',
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
]);
