import { readFile } from 'node:fs/promises';
import { initialize } from '@auto-palette/wasm';

const module = await readFile(
  '../../crates/auto-palette-wasm/dist/auto_palette_bg.wasm',
);
await initialize(module);
