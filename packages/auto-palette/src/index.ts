import { retrieveImageData, ImageSource } from './image';
import { Palette } from './palette';
import init, { extractPalette, InitInput } from './wasm';

export * from './color';
export * from './image';
export * from './swatch';
export * from './palette';

/**
 * Options for initializing the AutoPalette library.
 */
export interface AutoPaletteOptions {
  /**
   * The path to the auto-palette file.
   */
  readonly wasm: InitInput;
}

/**
 * Class representing the AutoPalette library.
 */
export class AutoPalette {
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
