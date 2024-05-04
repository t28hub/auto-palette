import init, { extract, AlgorithmWrapper, InitInput } from './wasm';
import { Palette } from './palette';

export { Color } from './color';
export { Swatch } from './swatch';
export { Palette };

/**
 * The options for initializing the AutoPalette module.
 */
export interface InitOptions {
  /**
   * The initialization input for the WebAssembly module.
   */
  readonly wasm?: InitInput;
}

/**
 * The supported algorithm names for palette extraction.
 *
 * - `kmeans`: K-means clustering algorithm. This algorithm is faster but less accurate.
 * - `dbscan`: Density-based spatial clustering of applications with noise (DBSCAN) algorithm. This algorithm is slower but more accurate.
 * - `dbscan++`: DBSCAN++ clustering algorithm. This algorithm is faster than DBSCAN and more accurate than K-means.
 */
export type AlgorithmName = 'kmeans' | 'dbscan' | 'dbscan++';

/**
 * The image source representation.
 */
export interface ImageSource {
  /**
   * The width of the image.
   */
  readonly width: number;

  /**
   * The height of the image.
   */
  readonly height: number;

  /**
   * The image data as a Uint8ClampedArray.
   */
  readonly data: Uint8ClampedArray;
}

/**
 * Class representing the AutoPalette module.
 */
export class AutoPalette {
  /**
   * Extracts a color palette from the given image source.
   *
   * @param source - The image source to extract the palette from.
   * @param algorithmName - The algorithm name to use for extraction.
   * @returns The extracted `Palette` instance.
   */
  public extract(source: ImageSource, algorithmName: AlgorithmName = 'dbscan'): Palette {
    const algorithm = AlgorithmWrapper.fromString(algorithmName);
    const wrapper = extract(source.width, source.height, source.data, algorithm);
    return new Palette(wrapper);
  }

  /**
   * Initializes the `AutoPalette` module.
   *
   * @param options - The options for initializing the module.
   * @returns The initialized `AutoPalette` module.
   */
  public static async initialize(options?: InitOptions): Promise<AutoPalette> {
    await init(options?.wasm);
    return new AutoPalette();
  }
}
