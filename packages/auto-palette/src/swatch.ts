import type { SwatchWrapper } from './wasm';
import { Color } from './color';

export interface Position {
  x: number;
  y: number;
}

export class Swatch {
  constructor(private readonly wrapper: SwatchWrapper) {}

  public get color(): Color {
    return new Color(this.wrapper.color());
  }

  public get position(): Position {
    const { x, y } = this.wrapper.position;
    return { x, y };
  }

  public get population(): number {
    return this.wrapper.population;
  }
}
