import {ColorWrapper} from "../pkg";

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
  constructor(private readonly wrapper: ColorWrapper) {
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
