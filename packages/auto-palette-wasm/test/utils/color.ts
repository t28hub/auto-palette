import { Color } from '@auto-palette/wasm';

/**
 * Parses a value into a Color object.
 *
 * @param value - The value to parse.
 * @return The parsed Color object, or undefined if the value is not a valid color.
 */
export function parseColor(value: unknown): Color | undefined {
  if (value instanceof Color) {
    return value;
  }
  if (typeof value === 'string') {
    try {
      return Color.fromHexString(value);
    } catch (_) {
      // Ignore the error and return undefined
      return undefined;
    }
  }
  if (typeof value === 'number') {
    return Color.fromInt(value);
  }
  return undefined;
}
