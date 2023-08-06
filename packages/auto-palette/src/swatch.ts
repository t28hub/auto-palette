import { Color } from './color';
import { isPosition } from './guards';
import { Position } from './types';
import { SwatchWrapper } from './wasm';

/**
 * Class representing a color swatch.
 *
 * @public
 * @class
 */
export class Swatch {
  /**
   * Creates a new instance of Swatch.
   *
   * @param wrapper - The SwatchWrapper instance.
   */
  constructor(private readonly wrapper: SwatchWrapper) {}

  /**
   * Gets the color of the swatch.
   */
  get color(): Color {
    return new Color(this.wrapper.color);
  }

  /**
   * Gets the position of the swatch.
   */
  get position(): Position {
    const position: unknown = this.wrapper.position;
    if (!isPosition(position)) {
      throw new Error(`Invalid position: ${JSON.stringify(position)}`);
    }
    return position;
  }

  /**
   * Gets the population of the swatch.
   */
  get population(): number {
    return this.wrapper.population;
  }
}
