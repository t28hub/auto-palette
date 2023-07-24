import init from '../pkg';

export { Color } from './color';
export { Swatch, type Position } from './swatch';
export { Palette } from './palette';
export async function initialize() {
  await init();
}
