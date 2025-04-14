import { Color } from '@auto-palette/core';
import { describe, it } from 'vitest';

describe('@auto-palette/wasm/matchers', () => {
  describe('.toBeSimilarColor', () => {
    it('should pass when colors are similar', () => {
      // Act
      const received = Color.fromHexString('#ff8000');
      const expected = Color.fromHexString('#ff7f00');

      // Assert
      expect(received).toBeSimilarColor(expected);
    });

    it('should pass when colors are the same', () => {
      // Act
      const received = Color.fromHexString('#ff8000');
      const expected = Color.fromHexString('#ff8000');

      // Assert
      expect(received).toBeSimilarColor(expected);
    });

    it('should pass when colors are similar with a tolerance', () => {
      // Act
      const received = Color.fromHexString('#ff8000');
      const expected = Color.fromHexString('#ff8800');

      // Assert
      expect(received).toBeSimilarColor(expected, 5.0);
    });

    it('should fail when colors are different', () => {
      // Act
      const received = Color.fromHexString('#ff8000');
      const expected = Color.fromHexString('#00ff80');

      // Assert
      expect(() => {
        expect(received).toBeSimilarColor(expected);
      }).toThrowError(/Expected color to be similar to:/);
    });
  });

  describe('.not.toBeSimilarColor', () => {
    it('should pass when colors are different', () => {
      // Act
      const received = Color.fromHexString('#ff8000');
      const expected = Color.fromHexString('#00ff80');

      // Assert
      expect(received).not.toBeSimilarColor(expected);
    });

    it('should fail when colors are similar', () => {
      // Act
      const received = Color.fromHexString('#ff8000');
      const expected = Color.fromHexString('#ff7f00');

      // Assert
      expect(() => {
        expect(received).not.toBeSimilarColor(expected);
      }).toThrowError(/Expected color to not be similar to:/);
    });
  });
});
