import { ImageSource, retrieveImageData } from './image';
import { Palette } from './palette';
import init, { Algorithm as ExtractionAlgorithm, extractPalette, InitInput } from './wasm';

export * from './color';
export * from './image';
export * from './palette';
export * from './swatch';
export * from './types';

/**
 * Options for initializing the AutoPalette library.
 */
export type InitializationOptions = {
  /**
   * The path to the auto-palette file.
   */
  readonly wasm: InitInput;
};

/**
 * The method to use for color extraction.
 */
export type ExtractionMethod = 'gmeans' | 'dbscan';

/**
 * Class representing the AutoPalette library.
 */
export class AutoPalette {
  /**
   * Extracts a color palette from the given image.
   *
   * @param source - The source of the image.
   * @param method - The method to use for color extraction.
   * @returns The extracted color palette.
   */
  public extract(source: ImageSource, method: ExtractionMethod = 'dbscan'): Palette {
    const imageData = retrieveImageData(source);
    const algorithm = method === 'gmeans' ? ExtractionAlgorithm.GMeans : ExtractionAlgorithm.DBSCAN;
    const wrapper = extractPalette(imageData.data, imageData.width, imageData.height, algorithm);
    return new Palette(wrapper);
  }

  /**
   * Initializes the AutoPalette library.
   *
   * @param options - The options for initializing the library.
   * @returns The initialized AutoPalette library.
   */
  public static async initialize(options?: InitializationOptions): Promise<AutoPalette> {
    await init(options?.wasm);
    return new AutoPalette();
  }
}
