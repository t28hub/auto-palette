import {
  type Algorithm,
  Color,
  Palette,
  Swatch,
  type Theme,
} from '@auto-palette/core';
import { describe, expect } from 'vitest';
import { loadImageData } from './image';

describe('@auto-palette/wasm/palette', () => {
  describe('constructor', () => {
    it('should create a Palette instance', () => {
      // Act
      const actual = new Palette([
        new Swatch(
          Color.fromHexString('#6DE1D2'),
          { x: 120, y: 240 },
          100,
          0.25,
        ),
        new Swatch(
          Color.fromHexString('#FFD63A'),
          { x: 200, y: 300 },
          100,
          0.25,
        ),
        new Swatch(
          Color.fromHexString('#FFA955'),
          { x: 150, y: 100 },
          100,
          0.25,
        ),
        new Swatch(
          Color.fromHexString('#FF6F61'),
          { x: 300, y: 400 },
          100,
          0.25,
        ),
      ]);

      // Assert
      expect(actual).toBeDefined();
      expect(actual.isEmpty()).toBeFalsy();
      expect(actual).toHaveLength(4);
    });

    it('should create an empty Palette instance', () => {
      // Act
      const actual = new Palette([]);

      // Assert
      expect(actual).toBeDefined();
      expect(actual.isEmpty()).toBeTruthy();
      expect(actual).toHaveLength(0);
    });
  });

  describe('findSwatches', () => {
    let palette: Palette;
    beforeAll(() => {
      palette = new Palette([
        new Swatch(
          Color.fromHexString('#FF6F61'),
          { x: 120, y: 240 },
          100,
          0.25,
        ),
        new Swatch(
          Color.fromHexString('#6DE1D2'),
          { x: 200, y: 300 },
          100,
          0.25,
        ),
        new Swatch(
          Color.fromHexString('#FFD63A'),
          { x: 150, y: 100 },
          100,
          0.25,
        ),
        new Swatch(
          Color.fromHexString('#3F4F44'),
          { x: 300, y: 400 },
          100,
          0.25,
        ),
        new Swatch(
          Color.fromHexString('#A27B5C'),
          { x: 400, y: 500 },
          100,
          0.25,
        ),
        new Swatch(
          Color.fromHexString('#604652'),
          { x: 500, y: 600 },
          100,
          0.25,
        ),
        new Swatch(
          Color.fromHexString('#F7CFD8'),
          { x: 120, y: 240 },
          100,
          0.25,
        ),
        new Swatch(
          Color.fromHexString('#F4F8D3'),
          { x: 200, y: 300 },
          100,
          0.25,
        ),
        new Swatch(
          Color.fromHexString('#A6D6D6'),
          { x: 150, y: 100 },
          100,
          0.25,
        ),
        new Swatch(
          Color.fromHexString('#210F37'),
          { x: 300, y: 400 },
          100,
          0.25,
        ),
        new Swatch(
          Color.fromHexString('#2A0A1D'),
          { x: 400, y: 500 },
          100,
          0.25,
        ),
        new Swatch(
          Color.fromHexString('#4F1C51'),
          { x: 500, y: 600 },
          100,
          0.25,
        ),
      ]);
    });

    it('should find the swatches from the palette', () => {
      // Act
      const actual = palette.findSwatches(3);

      // Assert
      expect(actual).toHaveLength(3);
      expect(actual[0].color.toHexString()).toEqual('#FF6F61');
      expect(actual[1].color.toHexString()).toEqual('#FFD63A');
      expect(actual[2].color.toHexString()).toEqual('#6DE1D2');
    });

    it.each([
      { theme: 'vivid', count: 3, expected: ['#4F1C51', '#FFD63A', '#FF6F61'] },
      { theme: 'muted', count: 3, expected: ['#A27B5C', '#3F4F44', '#604652'] },
      { theme: 'light', count: 3, expected: ['#6DE1D2', '#F7CFD8', '#FF6F61'] },
      { theme: 'dark', count: 3, expected: ['#604652', '#210F37', '#3F4F44'] },
      {
        theme: 'colorful',
        count: 3,
        expected: ['#FF6F61', '#6DE1D2', '#4F1C51'],
      },
    ])(
      'should find the swatches from the palette with $theme theme',
      ({ theme, count, expected }) => {
        // Act
        const actual = palette.findSwatches(count, theme as Theme);

        // Assert
        expect(actual).toHaveLength(3);
        expect(actual[0].color.toHexString()).toEqual(expected[0]);
        expect(actual[1].color.toHexString()).toEqual(expected[1]);
        expect(actual[2].color.toHexString()).toEqual(expected[2]);
      },
    );

    it('should return an empty array if the count is less than 1', () => {
      // Act
      const actual = palette.findSwatches(0);

      // Assert
      expect(actual).toHaveLength(0);
    });

    it('should throw an error if the theme is not supported', () => {
      // Assert
      expect(() => {
        // Act
        const theme = 'unsupported' as Theme;
        palette.findSwatches(3, theme);
      }).toThrowError('Unknown theme name: unsupported');
    });
  });

  describe('extract', () => {
    let imageData: ImageData;
    beforeAll(async () => {
      imageData = await loadImageData('../../gfx/flags/za.png');
    });

    it('should extract a palette from an image', () => {
      // Act
      const actual = Palette.extract(imageData);

      // Assert
      expect(actual.isEmpty()).toBeFalsy();
      expect(actual.length).toBeGreaterThanOrEqual(6);

      const swatches = actual.findSwatches(5, 'vivid');
      expect(swatches.length).toBe(5);
      swatches.forEach((swatch) => {
        console.info('Swatch color: %s', swatch.color.toHexString());
      });
    });

    it('should extract a palette from an image with the given algorithm', () => {
      // Act
      const actual = Palette.extract(imageData, 'dbscan++');

      // Assert
      expect(actual.isEmpty()).toBeFalsy();
      expect(actual.length).toBeGreaterThanOrEqual(6);

      const swatches = actual.findSwatches(5, 'vivid');
      expect(swatches.length).toBe(5);
      swatches.forEach((swatch) => {
        console.info('Swatch color: %s', swatch.color.toHexString());
      });
    });

    it('should throw an error if the image data is empty', () => {
      // Act & Assert
      expect(() => {
        const imageData = new ImageData(0, 0);
        Palette.extract(imageData);
      }).toThrowError('ImageData is not defined');
    });

    it('should throw an error if the algorithm is not supported', () => {
      // Act & Assert
      expect(() => {
        const algorithm = 'unsupported' as Algorithm;
        Palette.extract(imageData, algorithm);
      }).toThrowError('Unknown algorithm name: unsupported');
    });
  });
});
