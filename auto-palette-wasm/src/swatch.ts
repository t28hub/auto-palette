import {SwatchWrapper} from "../pkg";

import {Color} from "./color";

export type Position = { x: number, y: number };

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
  constructor(private readonly wrapper: SwatchWrapper) {
  }

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
    const {x, y} = this.wrapper.position;
    return {x, y};
  }

  /**
   * Gets the population of the swatch.
   */
  get population(): number {
    return this.wrapper.population;
  }
}

/**
 * Checks if the given value is a SwatchWrapper instance.
 *
 * @param value - The value to check.
 * @returns True if the given value is a SwatchWrapper instance, false otherwise.
 */
export function isSwatchWrapper(value: unknown): value is SwatchWrapper {
  if (value === null) {
    return false;
  }
  if (typeof value !== 'object') {
    return false;
  }
  return 'color' in value && 'position' in value && 'population' in value;
}
