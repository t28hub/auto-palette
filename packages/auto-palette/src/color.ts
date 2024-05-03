import type { ColorWrapper } from './wasm';

/**
 * The RGB color space.
 */
export interface RGB {
  r: number;
  g: number;
  b: number;
}

/**
 * The CIE XYZ color space.
 */
export interface XYZ {
  x: number;
  y: number;
  z: number;
}

/**
 * The CIE L*a*b* color space.
 */
export interface Lab {
  l: number;
  a: number;
  b: number;
}

/**
 * Class representing a color.
 */
export class Color {
  /**
   * Create a new `Color` instance.
   *
   * @internal
   * @param wrapper - The `ColorWrapper` instance.
   * @returns A new `Color` instance.
   */
  constructor(private readonly wrapper: ColorWrapper) {}

  /**
   * Check if the color is light.
   *
   * @returns `true` if the color is light, `false` otherwise.
   */
  public isLight(): boolean {
    return this.wrapper.isLight();
  }

  /**
   * Check if the color is dark.
   *
   * @returns `true` if the color is dark, `false` otherwise.
   */
  public isDark(): boolean {
    return this.wrapper.isDark();
  }

  /**
   * Returns the lightness of the color.
   *
   * @returns The lightness of the color.
   */
  public lightness(): number {
    return this.wrapper.lightness();
  }

  /**
   * Returns the chroma of the color.
   *
   * @returns The chroma of the color.
   */
  public chroma(): number {
    return this.wrapper.chroma();
  }

  /**
   * Returns the hue of the color.
   *
   * @returns The hue of the color.
   */
  public hue(): number {
    return this.wrapper.hue();
  }

  /**
   * Returns the hex string representation of the color.
   *
   * @returns The hex string representation of the color.
   */
  public toHexString(): string {
    return this.wrapper.toHexString();
  }

  /**
   * Returns the RGB representation of the color.
   *
   * @returns The RGB representation of the color.
   */
  public toRGB(): RGB {
    const { r, g, b } = this.wrapper.toRGB();
    return { r, g, b };
  }

  /**
   * Returns the CIE XYZ representation of the color.
   *
   * @returns The CIE XYZ representation of the color.
   */
  public toXYZ(): XYZ {
    const { x, y, z } = this.wrapper.toXYZ();
    return { x, y, z };
  }

  /**
   * Returns the CIE L*a*b* representation of the color.
   *
   * @returns The CIE L*a*b* representation of the color.
   */
  public toLab(): Lab {
    const { l, a, b } = this.wrapper.toLab();
    return { l, a, b };
  }
}
