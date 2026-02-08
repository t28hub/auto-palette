import { defineConfig } from 'tsup';
import { copy } from 'esbuild-plugin-copy';

export default defineConfig({
  target: 'esnext',
  entry: ['ts/index.ts'],
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
          from: './pkg/*.wasm',
          to: './dist',
        },
      ],
    }),
  ],
});
