import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { type Algorithm, initialize, Palette, type Theme } from '@auto-palette/wasm';
import { beforeAll, describe, expect, it } from 'vitest';
import { loadImageData } from './utils/image';

const WASM_PATH = './dist/auto_palette_bg.wasm';
const IMAGE_PATH = resolve(process.cwd(), '../../gfx/laura-clugston-pwW2iV9TZao-unsplash.jpg');

describe('@auto-palette/wasm', () => {
  describe('initialize', () => {
    it('should initialize the WebAssembly module from a buffer', async () => {
      // Act
      const buffer = await readFile(WASM_PATH);
      await initialize(buffer);

      // Assert
      expect(Palette).toBeDefined();
    });

    it.skip('should throw an error if the module cannot be loaded', async () => {
      // Act & Assert
      const buffer = readFile('/dist/unknown.wasm');
      await expect(initialize(buffer)).rejects.toThrowError(/Failed to initialize WebAssembly module: /);
    });
  });

  describe('Palette', () => {
    beforeAll(async () => {
      const buffer = await readFile(WASM_PATH);
      await initialize(buffer);
    });

    describe('extract', () => {
      let imageData: ImageData;
      beforeAll(async () => {
        // Arrange
        imageData = await loadImageData(IMAGE_PATH);
      });

      it('should extract the palette from an image', () => {
        // Act
        const palette = Palette.extract(imageData);

        // Assert
        expect(palette.isEmpty()).toBeFalsy();
        expect(palette.length).toBeGreaterThan(32);
      });

      it.each([
        { algorithm: 'dbscan', expectedLength: 48 },
        { algorithm: 'dbscan++', expectedLength: 72 },
        { algorithm: 'kmeans', expectedLength: 24 },
        { algorithm: 'slic', expectedLength: 64 },
        { algorithm: 'snic', expectedLength: 64 },
      ])('should extract the palette from an image with the $algorithm algorithm', ({ algorithm, expectedLength }) => {
        // Act
        const palette = Palette.extract(imageData, algorithm as Algorithm);

        // Assert
        expect(palette.isEmpty()).toBeFalsy();
        expect(palette.length).toBeGreaterThan(expectedLength);
      });
    });

    const isLinux = process.platform === 'linux';
    describe('findSwatches', () => {
      let palette: Palette;
      beforeAll(async () => {
        // Arrange
        const imageData = await loadImageData(IMAGE_PATH);
        palette = Palette.extract(imageData, 'dbscan');
      });

      it.skipIf(isLinux)('should find the swatches from the palette', () => {
        // Act
        const swatches = palette.findSwatches(3);

        // Assert
        expect(swatches).toHaveLength(3);
        expect(swatches[0].color).toBeSimilarColor('#5ECBFE');
        expect(swatches[1].color).toBeSimilarColor('#BF010D');
        expect(swatches[2].color).toBeSimilarColor('#FCDC24');
      });

      it.skipIf(isLinux).each([
        {
          count: 3,
          theme: 'colorful',
          expected: ['#27A9DA', '#B18A0A', '#C71B7A'],
        },
        {
          count: 3,
          theme: 'vivid',
          expected: ['#1FB0E4', '#DDAB02', '#E6263E'],
        },
        {
          count: 3,
          theme: 'muted',
          expected: ['#3A3616', '#711978', '#9D9E59'],
        },
        {
          count: 3,
          theme: 'light',
          expected: ['#80AEEB', '#D0C353', '#E96894'],
        },
        {
          count: 3,
          theme: 'dark',
          expected: ['#02203A', '#042B20', '#48040E'],
        },
      ])('should find the swatches from the palette with the $theme theme', ({ count, theme, expected }) => {
        // Act
        const swatches = palette.findSwatches(count, theme as Theme);
        swatches.sort((a, b) => a.color.toInt() - b.color.toInt());

        // Assert
        expect(swatches).toHaveLength(count);

        expect(swatches[0].color).toBeSimilarColor(expected[0]);
        expect(swatches[1].color).toBeSimilarColor(expected[1]);
        expect(swatches[2].color).toBeSimilarColor(expected[2]);
      });
    });
  });
});
