import { Swatch } from './swatch';
import { AlgorithmWrapper, extract, type PaletteWrapper, ThemeWrapper } from './wasm';

/**
 * The supported algorithm names for palette extraction.
 *
 * - `kmeans`: K-means clustering algorithm. This algorithm is faster but less accurate.
 * - `dbscan`: Density-based spatial clustering of applications with noise (DBSCAN) algorithm. This algorithm is slower but more accurate.
 * - `dbscan++`: DBSCAN++ clustering algorithm. This algorithm is faster than DBSCAN and more accurate than K-means.
 */
export type AlgorithmName = 'kmeans' | 'dbscan' | 'dbscan++';

/**
 * The supported theme names for swatch selection.
 *
 * - `basic`: Basic theme. This theme is suitable for general use.
 * - `vivid`: Vivid theme. This theme is suitable for vivid colors.
 * - `muted`: Muted theme. This theme is suitable for muted colors.
 * - `light`: Light theme. This theme is suitable for light colors.
 * - `dark`: Dark theme. This theme is suitable for dark colors.
 */
export type ThemeName = 'basic' | 'vivid' | 'muted' | 'light' | 'dark';

/**
 * Palette class represents a color palette.
 */
export class Palette {
  /**
   * Creates a new `Palette` instance.
   *
   * @internal
   * @param wrapper - The `PaletteWrapper` instance.
   * @returns A new `Palette` instance.
   */
  constructor(private readonly wrapper: PaletteWrapper) {}

  /**
   * Returns the number of swatches in the palette.
   *
   * @returns The number of swatches in the palette.
   */
  public get length(): number {
    return this.wrapper.length;
  }

  /**
   * Checks whether the palette is empty.
   *
   * @returns `true` if the palette is empty, `false` otherwise.
   */
  public isEmpty(): boolean {
    return this.wrapper.isEmpty();
  }

  /**
   * Finds the best `n` swatches from the palette with the given theme.
   *
   * @param n - The number of swatches to find.
   * @param themeName - The theme name to use for swatch selection.
   * @returns The best `n` swatches. If the candidate swatches are less than `n`, the returned array may contain fewer swatches.
   */
  public findSwatches(n: number, themeName: ThemeName = 'basic'): Swatch[] {
    const theme = ThemeWrapper.fromString(themeName);
    return this.wrapper.findSwatches(n, theme).map((wrapper) => new Swatch(wrapper));
  }

  /**
   * Extracts a color palette from the given image data.
   * @param imageData - The image data to extract the palette from.
   * @param algorithmName - The algorithm name to use for extraction.
   * @returns The extracted `Palette` instance.
   */
  public static extract(imageData: ImageData, algorithmName: AlgorithmName = 'dbscan'): Palette {
    const algorithm = AlgorithmWrapper.fromString(algorithmName);
    const wrapper = extract(imageData.width, imageData.height, imageData.data, algorithm);
    return new Palette(wrapper);
  }
}
