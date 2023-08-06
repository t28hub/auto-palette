import { Lab, Position, RGB } from './types';
import { SwatchWrapper } from './wasm';

/**
 * Checks if the given value is an object.
 *
 * @param value - The value to check.
 * @returns true if the given value is an object, false otherwise.
 */
export function isObject(value: unknown): value is object {
  if (value === null) {
    return false;
  }
  return typeof value === 'object';
}

/**
 * Check if the given value is undefined.
 *
 * @param value - The value to check.
 * @returns true if the value is undefined, false otherwise.
 */
export function isUndefined(value: unknown): value is undefined {
  return typeof value === 'undefined';
}

/**
 * Check if the given value is a 2-dimensional position.
 *
 * @param value - The value to check.
 * @returns true if the given value is a 2-dimensional position, false otherwise.
 */
export function isPosition(value: unknown): value is Position {
  if (!isObject(value)) {
    return false;
  }
  return 'x' in value && 'y' in value;
}

/**
 * Checks if the given value is a CIE L*a*b* color.
 *
 * @param value - The value to check.
 * @returns true if the given value is a CIE L*a*b* color, false otherwise.
 */
export function isLab(value: unknown): value is Lab {
  if (!isObject(value)) {
    return false;
  }
  return 'l' in value && 'a' in value && 'b' in value;
}

/**
 * Checks if the given value is a RGB color.
 *
 * @param value - The value to check.
 * @returns true if the given value is a RGB color, false otherwise.
 */
export function isRGB(value: unknown): value is RGB {
  if (!isObject(value)) {
    return false;
  }
  return 'r' in value && 'g' in value && 'b' in value;
}

/**
 * Checks if the given value is a SwatchWrapper instance.
 *
 * @param value - The value to check.
 * @returns True if the given value is a SwatchWrapper instance, false otherwise.
 */
export function isSwatchWrapper(value: unknown): value is SwatchWrapper {
  if (!isObject(value)) {
    return false;
  }
  return 'color' in value && 'position' in value && 'population' in value;
}
