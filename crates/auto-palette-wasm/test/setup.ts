import { initialize } from '@auto-palette/wasm';
import { readFile } from 'node:fs/promises';

const module = readFile('./pkg/auto_palette_bg.wasm');
await initialize(module);
