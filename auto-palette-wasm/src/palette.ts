import {extractPalette, PaletteWrapper, SwatchWrapper} from "../pkg";

import {isSwatchWrapper, Swatch} from "./swatch";

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
   * @param wrapper - The PaletteWrapper instance.
   * @private
   */
  private constructor(private readonly wrapper: PaletteWrapper) {
  }

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
    return this.wrapper.swatches(count).filter((wrapper) => isSwatchWrapper(wrapper)).map((wrapper: SwatchWrapper) => new Swatch(wrapper));
  }

  /**
   * Extracts a color palette from the given image data.
   *
   * @param source - The image data to extract the color palette from.
   * @returns The extracted color palette.
   */
  static from(source: ImageData): Palette {
    const instance = extractPalette(source.data, source.width, source.height);
    return new Palette(instance);
  }
}
