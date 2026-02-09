import { defineConfig } from 'tsdown';

export default defineConfig({
  entry: ['src/index.ts'],
  target: 'esnext',
  format: ['cjs', 'esm'],
  dts: true,
  shims: true,
  minify: true,
  hash: false,
  clean: true,
  copy: [
    {
      from: '../../crates/auto-palette-wasm/dist/*.wasm',
      to: 'dist',
    },
  ],
});
