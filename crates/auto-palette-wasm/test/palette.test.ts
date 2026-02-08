import { resolve } from 'node:path';
import {
  type Algorithm,
  Color,
  Palette,
  Swatch,
  type Theme,
} from '@auto-palette/core';
import { describe, expect } from 'vitest';
import { loadImageData } from './utils/image';

const IMAGE_PATH = resolve(process.cwd(), '../../gfx/flags/za.png');

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
          150,
          0.15,
        ),
        new Swatch(
          Color.fromHexString('#6DE1D2'),
          { x: 200, y: 300 },
          360,
          0.36,
        ),
        new Swatch(
          Color.fromHexString('#FFD63A'),
          { x: 150, y: 100 },
          300,
          0.3,
        ),
        new Swatch(
          Color.fromHexString('#3F4F44'),
          { x: 300, y: 400 },
          80,
          0.08,
        ),
        new Swatch(
          Color.fromHexString('#A27B5C'),
          { x: 400, y: 500 },
          60,
          0.06,
        ),
        new Swatch(
          Color.fromHexString('#604652'),
          { x: 500, y: 600 },
          40,
          0.04,
        ),
        new Swatch(
          Color.fromHexString('#F7CFD8'),
          { x: 120, y: 240 },
          30,
          0.03,
        ),
        new Swatch(
          Color.fromHexString('#F4F8D3'),
          { x: 200, y: 300 },
          30,
          0.03,
        ),
        new Swatch(
          Color.fromHexString('#A6D6D6'),
          { x: 150, y: 100 },
          20,
          0.02,
        ),
        new Swatch(
          Color.fromHexString('#210F37'),
          { x: 300, y: 400 },
          20,
          0.02,
        ),
        new Swatch(
          Color.fromHexString('#2A0A1D'),
          { x: 400, y: 500 },
          10,
          0.01,
        ),
        new Swatch(
          Color.fromHexString('#4F1C51'),
          { x: 500, y: 600 },
          10,
          0.01,
        ),
      ]);
    });

    it('should find the swatches from the palette', () => {
      // Act
      const actual = palette.findSwatches(3);

      // Assert
      expect(actual).toHaveLength(3);
      expect(actual[0].color).toBeSameColor('#6DE1D2');
      expect(actual[1].color).toBeSameColor('#FFD63A');
      expect(actual[2].color).toBeSameColor('#FF6F61');
    });

    it.each([
      { theme: 'vivid', count: 3, expected: ['#6DE1D2', '#FFD63A', '#FF6F61'] },
      { theme: 'muted', count: 3, expected: ['#604652', '#A27B5C', '#3F4F44'] },
      { theme: 'light', count: 3, expected: ['#6DE1D2', '#FFD63A', '#FF6F61'] },
      { theme: 'dark', count: 3, expected: ['#3F4F44', '#210F37', '#2A0A1D'] },
      {
        theme: 'colorful',
        count: 3,
        expected: ['#6DE1D2', '#FFD63A', '#FF6F61'],
      },
    ])(
      'should find the swatches from the palette with $theme theme',
      ({ theme, count, expected }) => {
        // Act
        const actual = palette.findSwatches(count, theme as Theme);

        // Assert
        expect(actual).toHaveLength(3);
        const actualColors = actual.map((swatch) => swatch.color.toHexString());
        expect(actualColors).toEqual(expect.arrayContaining(expected));
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

  describe(
    'extract',
    () => {
      let imageData: ImageData;
      beforeAll(async () => {
        imageData = await loadImageData(IMAGE_PATH);
      });

      it('should extract a palette from an image', () => {
        // Act
        const actual = Palette.extract(imageData);

        // Assert
        expect(actual.isEmpty()).toBeFalsy();
        expect(actual.length).toBeGreaterThanOrEqual(6);

        const swatches = actual.findSwatches(6).sort((a, b) => {
          return b.color.toInt() - a.color.toInt();
        });
        expect(swatches.length).toBe(6);
        expect(swatches[0].color).toBeSimilarColor('#FFFFFF');
        expect(swatches[1].color).toBeSimilarColor('#FFB916');
        expect(swatches[2].color).toBeSimilarColor('#E1392D');
        expect(swatches[3].color).toBeSimilarColor('#007847');
        expect(swatches[4].color).toBeSimilarColor('#000C8A');
        expect(swatches[5].color).toBeSimilarColor('#000000');
      });

      it.each([
        { algorithm: 'dbscan' },
        { algorithm: 'dbscan++' },
        { algorithm: 'kmeans' },
        { algorithm: 'slic' },
        { algorithm: 'snic' },
      ])(
        'should extract a palette from an image with the $algorithm algorithm',
        ({ algorithm }) => {
          // Act
          const actual = Palette.extract(imageData, algorithm as Algorithm);

          // Assert
          expect(actual.isEmpty()).toBeFalsy();
          expect(actual.length).toBeGreaterThanOrEqual(6);

          const swatches = actual.findSwatches(6);
          expect(swatches.length).toBe(6);
        },
      );

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
    },
    { timeout: 10_000 },
  );
});
