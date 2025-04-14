import { expect } from 'vitest';
import { toBeSameColor } from './toBeSameColor';
import { toBeSimilarColor } from './toBeSimilarColor';

/**
 * Custom matchers for AutoPalette.
 *
 * These matchers are automatically extended to the global scope.
 */
interface AutoPaletteMatchers<R = unknown> {
  /**
   * Check whether the received color is the same as the expected color.
   *
   * @param expected - The expected color.
   * @return The matcher result.
   */
  toBeSameColor(expected: unknown): R;

  /**
   * Check whether the received color is similar to the expected color.
   *
   * @param expected - The expected color.
   * @param threshold - The allowed threshold.
   * @return The matcher result.
   */
  toBeSimilarColor(expected: unknown, threshold?: number): R;
}

declare module 'vitest' {
  // biome-ignore lint/suspicious/noExplicitAny: The generic type 'T' could be any type.
  interface Assertion<T = any> extends AutoPaletteMatchers<T> {}

  interface AsymmetricMatchersContaining extends AutoPaletteMatchers {}
}

expect.extend({
  toBeSameColor,
  toBeSimilarColor,
});
