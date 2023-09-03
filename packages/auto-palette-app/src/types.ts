import { Position } from 'auto-palette';

/**
 * Interface representing an image object.
 */
export interface ImageObject {
  /**
   * The width of the image.
   */
  readonly width: number;
  /**
   * The height of the image.
   */
  readonly height: number;
  /**
   * The data buffer of the image.
   */
  readonly buffer: ArrayBuffer;
  /**
   * The number of channels in the image.
   */
  readonly channels: number;
}

/**
 * Interface representing a color.
 */
export interface Color {
  /**
   * The hex value of the color.
   */
  readonly hex: string;
  /**
   * Whether the color is light.
   */
  readonly isLight: boolean;
  /**
   * The position of the color in the image.
   */
  readonly position: Position;
}

/**
 * Interface representing a size of an element.
 */
export interface Size {
  /**
   * The width of the element.
   */
  readonly width: number;
  /**
   * The height of the element.
   */
  readonly height: number;
}
