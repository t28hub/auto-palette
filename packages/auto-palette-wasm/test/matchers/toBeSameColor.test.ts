import { Color } from '@auto-palette/core';
import { describe, expect } from 'vitest';

describe('@auto-palette/wasm/matchers', () => {
  describe('.toBeSameColor', () => {
    it('should pass when colors are the same', () => {
      // Act
      const received = Color.fromHexString('#ff8000');
      const expected = Color.fromHexString('#ff8000');

      // Assert
      expect(received).toBeSameColor(expected);
    });

    it('should pass when colors are the same (hex string format)', () => {
      // Act
      const received = Color.fromHexString('#ff8000');
      const expected = '#ff8000';

      // Assert
      expect(received).toBeSameColor(expected);
    });

    it('should pass when colors are the same (int format)', () => {
      // Act
      const received = Color.fromHexString('#ff8000');
      const expected = 0xff8000;

      // Assert
      expect(received).toBeSameColor(expected);
    });

    it('should fail when colors are different', () => {
      // Act
      const received = Color.fromHexString('#ff8000');
      const expected = Color.fromHexString('#00ff80');

      // Assert
      expect(() => {
        expect(received).toBeSameColor(expected);
      }).toThrowError(/Expected color to be the same as:/);
    });

    it('should fail when expected is not a color', () => {
      // Act
      const received = Color.fromHexString('#ff8000');
      const expected = 'not a color';

      // Assert
      expect(() => {
        expect(received).toBeSameColor(expected);
      }).toThrowError(/is not a valid color/);
    });

    describe('.not.toBeSameColor', () => {
      it('should pass when colors are different', () => {
        // Act
        const received = Color.fromHexString('#ff8000');
        const expected = Color.fromHexString('#00ff80');

        // Assert
        expect(received).not.toBeSameColor(expected);
      });

      it('should fail when colors are the same', () => {
        // Act
        const received = Color.fromHexString('#ff8000');
        const expected = Color.fromHexString('#ff8000');

        // Assert
        expect(() => {
          expect(received).not.toBeSameColor(expected);
        }).toThrowError(/Expected color to not be the same as:/);
      });
    });
  });
});
