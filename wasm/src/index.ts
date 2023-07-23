import init from '../pkg';

export type { Color } from './color';
export type { Swatch, Position } from './swatch';
export type { Palette } from './palette';

/**
 * Initializes the AutoPalette WASM module.
 *
 * @returns A promise that resolves when the WASM module is initialized.
 */
export default async function initialize(): Promise<void> {
  await init();
}
