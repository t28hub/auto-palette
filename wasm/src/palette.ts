import { PaletteWrapper, SwatchWrapper } from '../pkg';

import { isSwatchWrapper, Swatch } from './swatch';

/**
 * Class representing a color palette.
 *
 * @public
 * @class
 */
export class Palette {
  /**
   * Creates a new instance of Palette.
   *
   * @internal
   * @param wrapper - The PaletteWrapper instance.
   */
  constructor(private readonly wrapper: PaletteWrapper) {}

  /**
   * Gets the number of swatches in the palette.
   */
  get length(): number {
    return this.wrapper.length;
  }

  /**
   * Finds the number of swatches in the palette.
   *
   * @param count - The number of swatches to find.
   * @returns The swatches in the palette.
   */
  findSwatches(count: number): Swatch[] {
    return this.wrapper
      .swatches(count)
      .filter((wrapper) => isSwatchWrapper(wrapper))
      .map((wrapper: SwatchWrapper) => new Swatch(wrapper));
  }
}
