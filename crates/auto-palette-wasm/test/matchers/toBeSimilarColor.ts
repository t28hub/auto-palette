import type { Color } from '@auto-palette/wasm';
import {
  EXPECTED_COLOR,
  matcherErrorMessage,
  matcherHint,
  printExpected,
  printReceived,
  printWithType,
} from 'jest-matcher-utils';
import { parseColor } from '../utils/color';
import type { ExpectationResult, MatcherState } from './types';

const DEFAULT_EPSILON = 1.0;

/**
 * Check if two colors are similar.
 *
 * @param received - The received color.
 * @param expected - The expected color.
 * @param epsilon - The tolerance for similarity (default is 0.1).
 * @returns An object containing the result of the comparison.
 */
export function toBeSimilarColor(
  this: MatcherState,
  received: Color,
  expected: unknown,
  epsilon = DEFAULT_EPSILON,
): ExpectationResult {
  const matcherName = this.isNot
    ? '.not.toBeSimilarColor'
    : '.toBeSimilarColor';
  const expectedColor = parseColor(expected);
  if (!expectedColor) {
    const message = matcherErrorMessage(
      matcherHint(matcherName, 'received', 'expected'),
      'Expected value is not a valid color',
      `${EXPECTED_COLOR('expected')} is not a valid color`,
      printWithType('expected', expected, printExpected),
    );
    throw new Error(message);
  }

  const { l: l1, a: a1, b: b1 } = received.toLab();
  const { l: l2, a: a2, b: b2 } = expectedColor.toLab();
  const deltaE = Math.sqrt((l1 - l2) ** 2 + (a1 - a2) ** 2 + (b1 - b2) ** 2);

  return {
    pass: deltaE < epsilon,
    message: () => {
      return `${matcherHint(matcherName, 'received', 'expected')}
        Expected color to ${this.isNot ? 'not be' : 'be'} similar to:
          Expected: ${printExpected(expectedColor.toHexString())}
          Received: ${printReceived(received.toHexString())}

          Delta E: ${printReceived(deltaE)}
          Epsilon: ${printReceived(epsilon)}`;
    },
  };
}
