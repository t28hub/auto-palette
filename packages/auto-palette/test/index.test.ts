import { describe, expect, it } from 'vitest';

import { ColorWrapper } from '../src';

describe('ColorWrapper', () => {
  describe('fromString', () => {
    it('should create a new instance from a hex string', () => {
      // Act
      const actual = ColorWrapper.fromString('#e74d1d');

      // Assert
      expect(actual.toRGB()).toEqual({ r: 231, g: 77, b: 29 });
    });

    it('should throw an error when the hex string is invalid', () => {
      // Act & Assert
      expect(() => {
        ColorWrapper.fromString('#e74d1');
      }).toThrowError(/Failed to parse color/);
    });
  });
});
