import { resolve } from 'node:path';
import dts from 'vite-plugin-dts';
import topLevelAwait from 'vite-plugin-top-level-await';
import wasm from 'vite-plugin-wasm';
import { defineConfig, type UserConfig } from 'vitest/config';

export default defineConfig(({ mode }): UserConfig => {
  const isDevelopment = mode === 'development';
  return {
    build: {
      lib: {
        name: 'AutoPalette',
        entry: resolve(__dirname, 'src/index.ts'),
        formats: ['cjs', 'es', 'umd'],
        fileName: (format) => {
          if (format === 'cjs') {
            return 'index.cjs';
          }
          if (format === 'es') {
            return 'index.mjs';
          }
          return 'index.js';
        },
      },
      minify: !isDevelopment,
      sourcemap: isDevelopment,
      rollupOptions: {
        plugins: [
          // Replace `import.meta.url` with `undefined` to avoid runtime error.
          {
            name: 'replace-import-meta',
            resolveImportMeta(property): string | null {
              if (property === 'url') {
                return 'undefined';
              }
              return null;
            },
          },
        ],
      },
    },
    plugins: [
      wasm(),
      topLevelAwait(),
      dts({
        include: 'src',
        rollupTypes: true,
        copyDtsFiles: false,
      }),
    ],
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
  };
});
