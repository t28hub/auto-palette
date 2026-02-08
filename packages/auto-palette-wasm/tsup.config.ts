import { copy } from 'esbuild-plugin-copy';
import { defineConfig } from 'tsup';

export default defineConfig({
  target: 'esnext',
  entry: ['src/index.ts'],
  format: ['cjs', 'esm'],
  dts: true,
  sourcemap: false,
  shims: true,
  minify: true,
  clean: false,
  esbuildPlugins: [
    copy({
      resolveFrom: 'cwd',
      assets: [
        {
          from: '../../crates/auto-palette-wasm/dist/*.wasm',
          to: './dist',
        },
      ],
    }),
  ],
});
