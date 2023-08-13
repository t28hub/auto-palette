/**
 * Checks if the value is a string.
 *
 * @param value - The value to check.
 * @returns `true` if the value is a string, `false` otherwise.
 */
export function isString(value: unknown): value is string {
  return typeof value === 'string';
}

/**
 * Checks if the value is undefined.
 *
 * @param value - The value to check.
 * @returns `true` if the value is undefined, `false` otherwise.
 */
export function isUndefined(value: unknown): value is undefined {
  return typeof value === 'undefined';
}
