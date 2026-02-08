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

/**
 * Check if two colors are the same.
 *
 * @param received - The received color.
 * @param expected - The expected color.
 * @returns An object containing the result of the comparison.
 */
export function toBeSameColor(this: MatcherState, received: Color, expected: unknown): ExpectationResult {
  const matcherName = this.isNot ? '.not.toBeSameColor' : '.toBeSameColor';
  const expectedColor = parseColor(expected);
  if (!expectedColor) {
    const message = matcherErrorMessage(
      matcherName,
      'Expected value is not a valid color',
      `${EXPECTED_COLOR('expected')} is not a valid color`,
      printWithType<unknown>('expected', expected, printExpected),
    );
    throw new Error(message);
  }

  const receivedHex = received.toHexString();
  const expectedHex = expectedColor.toHexString();

  return {
    pass: receivedHex.toUpperCase() === expectedHex.toUpperCase(),
    message: () => {
      return `${matcherHint(matcherName, 'received', 'expected')}
        Expected color to ${this.isNot ? 'not be' : 'be'} the same as:
          Expected: ${printExpected(expectedHex)}
          Received: ${printReceived(receivedHex)}
      `;
    },
  };
}
