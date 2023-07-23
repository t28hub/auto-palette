import { resolve } from 'path';

import { defineConfig } from 'vite';
import dts from 'vite-plugin-dts';
import wasm from 'vite-plugin-wasm';

export default defineConfig({
  build: {
    lib: {
      entry: resolve(__dirname, 'src/index.ts'),
      name: 'AutoPalette',
      formats: ['es', 'umd'],
      fileName: (format) => `index.${format}.js`,
    },
    sourcemap: true,
  },
  plugins: [
    wasm(),
    dts({
      include: 'src',
      rollupTypes: true,
      copyDtsFiles: false,
    }),
  ],
});
