import type { SwatchWrapper } from './wasm';
import { Color } from './color';

export class Swatch {
  constructor(private readonly wrapper: SwatchWrapper) {}

  public color(): Color {
    return new Color(this.wrapper.color());
  }
}
