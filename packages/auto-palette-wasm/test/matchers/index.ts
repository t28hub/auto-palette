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

// Vitest 5 merges `Matchers` into both `expect()` assertions and asymmetric matchers.
declare module 'vitest' {
  interface Matchers<R extends void | Promise<void> = void | Promise<void>, T = unknown>
    extends AutoPaletteMatchers<R> {}
}

expect.extend({
  toBeSameColor,
  toBeSimilarColor,
});
