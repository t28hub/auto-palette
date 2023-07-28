import { SwatchWrapper } from '../pkg';

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
 * Checks if the given value is a SwatchWrapper instance.
 *
 * @param value - The value to check.
 * @returns True if the given value is a SwatchWrapper instance, false otherwise.
 */
export function isSwatchWrapper(value: unknown): value is SwatchWrapper {
  if (value === null) {
    return false;
  }
  if (typeof value !== 'object') {
    return false;
  }
  return 'color' in value && 'position' in value && 'population' in value;
}
