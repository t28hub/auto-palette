import { Color, Swatch } from '@auto-palette/wasm';
import { describe, expect } from 'vitest';

describe('@auto-palette/wasm/swatch', () => {
  describe('constructor', () => {
    it('should create a Swatch from a color, position, population, and ratio', () => {
      // Arrange
      const color = Color.fromHexString('#ff0080');
      const position = { x: 128, y: 256 };
      const population = 100;
      const ratio = 0.25;

      // Act
      const actual = new Swatch(color, position, population, ratio);

      // Assert
      expect(actual).toBeInstanceOf(Swatch);
      expect(actual.color.toHexString()).toEqual('#FF0080');
      expect(actual.position).toEqual(position);
      expect(actual.population).toEqual(population);
      expect(actual.ratio).toBeCloseTo(ratio);
    });

    it('should throw an error if population is zero', () => {
      // Arrange
      const color = Color.fromHexString('#ff0080');
      const position = { x: 128, y: 256 };
      const population = 0;
      const ratio = 0.25;

      // Act & Assert
      expect(() => {
        new Swatch(color, position, population, ratio);
      }).toThrowError(new Error('Population cannot be zero'));
    });

    it('should throw an error if ratio is not between 0 and 1', () => {
      // Arrange
      const color = Color.fromHexString('#ff0080');
      const position = { x: 128, y: 256 };
      const population = 100;
      const ratio = 1.5;

      // Act & Assert
      expect(() => {
        new Swatch(color, position, population, ratio);
      }).toThrowError(new Error('Ratio must be between 0 and 1'));
    });
  });
});
