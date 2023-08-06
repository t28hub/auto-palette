import { isLab, isRGB } from './guards';
import { Lab, RGB } from './types';
import { ColorWrapper } from './wasm';

/**
 * Class representing a color.
 *
 * @public
 * @class
 */
export class Color {
  /**
   * Creates a new instance of Color.
   *
   * @param wrapper - The ColorWrapper instance.
   */
  constructor(private readonly wrapper: ColorWrapper) {}

  /**
   * Checks whether this color is dark.
   *
   * @returns `true` if this color is dark, `false` otherwise.
   */
  readonly isDark: boolean = this.wrapper.isDark();

  /**
   * Checks whether this color is light.
   *
   * @returns `true` if this color is light, `false` otherwise.
   */
  readonly isLight: boolean = this.wrapper.isLight();

  /**
   * Returns the RGB representation of the color.
   *
   * @returns The RGB representation of the color.
   */
  toRGB(): RGB {
    const json: unknown = this.wrapper.toRGB();
    if (!isRGB(json)) {
      throw new Error(`Invalid RGB color: ${JSON.stringify(json)}`);
    }
    return json;
  }

  toLab(): Lab {
    const json: unknown = this.wrapper.toLab();
    if (!isLab(json)) {
      throw new Error(`Invalid Lab color: ${JSON.stringify(json)}`);
    }
    return json;
  }

  /**
   * Returns the hex string representation of the color.
   *
   * @returns The hex string representation of the color.
   */
  toString(): string {
    return this.wrapper.toHexString();
  }
}
