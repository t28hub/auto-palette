import { Color } from '@auto-palette/wasm';
import { describe } from 'vitest';

describe('@auto-palette/wasm/color', () => {
  describe('fromString', () => {
    it('should create a color from a hex string', () => {
      // Act
      const actual = Color.fromString('#ff0080');

      // Assert
      expect(actual.toHexString()).toEqual('#FF0080');
    });

    it('should create a color from a shortened hex string', () => {
      // Act
      const actual = Color.fromString('#f80');

      // Assert
      expect(actual.toHexString()).toEqual('#FF8800');
    });

    it('should create a color from a hex string with alpha', () => {
      // Act
      const actual = Color.fromString('#ff008080');

      // Assert
      expect(actual.toHexString()).toEqual('#FF0080');
    });
  });

  describe('isLight', () => {
    it('should return true for a light color', () => {
      // Act
      const color = Color.fromString('#ffffff');
      const actual = color.isLight();

      // Assert
      expect(actual).toBeTruthy();
    });

    it('should return false for a dark color', () => {
      // Act
      const color = Color.fromString('#000000');
      const actual = color.isLight();

      // Assert
      expect(actual).toBeFalsy();
    });
  });
});
