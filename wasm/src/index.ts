import init, { extractPalette, InitInput } from '../pkg';
import { retrieveImageData, ImageSource } from './image';
import { Palette } from './palette';

export { Color } from './color';
export { type ImageSource } from './image';
export { Swatch, type Position } from './swatch';
export { Palette } from './palette';

/**
 * Options for initializing the AutoPalette library.
 */
export interface AutoPaletteOptions {
  /**
   * The path to the wasm file.
   */
  readonly wasm: InitInput;
}

/**
 * Class representing the AutoPalette library.
 */
export class AutoPalette {
  /**
   * Creates a new instance of AutoPalette.
   *
   * @private
   * @see {@link AutoPalette.initialize}
   */
  private constructor() {}

  /**
   * Extracts a color palette from the given image.
   *
   * @param source - The source of the image.
   * @returns The extracted color palette.
   */
  public extract(source: ImageSource): Palette {
    const imageData = retrieveImageData(source);
    const wrapper = extractPalette(imageData.data, imageData.width, imageData.height);
    return new Palette(wrapper);
  }

  /**
   * Initializes the AutoPalette library.
   *
   * @param options - The options for initializing the library.
   * @returns The initialized AutoPalette library.
   */
  public static async initialize(options?: AutoPaletteOptions): Promise<AutoPalette> {
    await init(options?.wasm);
    return new AutoPalette();
  }
}
