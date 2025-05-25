import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import {
  type Algorithm,
  Palette,
  type Theme,
  initialize,
} from '@auto-palette/wasm';
import { beforeAll, describe, expect, it } from 'vitest';
import { loadImageData } from './utils/image';

const WASM_PATH = './dist/auto_palette_bg.wasm';
const IMAGE_PATH = resolve(
  process.cwd(),
  '../../gfx/laura-clugston-pwW2iV9TZao-unsplash.jpg',
);

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
      await expect(initialize(buffer)).rejects.toThrowError(
        /Failed to initialize WebAssembly module: /,
      );
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
      ])(
        'should extract the palette from an image with the $algorithm algorithm',
        ({ algorithm, expectedLength }) => {
          // Act
          const palette = Palette.extract(imageData, algorithm as Algorithm);

          // Assert
          expect(palette.isEmpty()).toBeFalsy();
          expect(palette.length).toBeGreaterThan(expectedLength);
        },
      );
    });

    const isLinux = process.platform === 'linux';
    describe('findSwatches', () => {
      let palette: Palette;
      beforeAll(async () => {
        // Arrange
        const imageData = await loadImageData(IMAGE_PATH);
        palette = Palette.extract(imageData, 'dbscan');
      });

      it('should find the swatches from the palette', () => {
        // Act
        const swatches = palette.findSwatches(3);

        // Assert
        expect(swatches).toHaveLength(3);
        expect(swatches[0].color).toBeSimilarColor(
          isLinux ? '#51C8FE' : '#5ECBFE',
        );
        expect(swatches[1].color).toBeSimilarColor(
          isLinux ? '#BD010C' : '#C7101E',
        );
        expect(swatches[2].color).toBeSimilarColor(
          isLinux ? '#FCDC24' : '#CFC663',
        );
      });

      it.each([
        {
          count: 3,
          theme: 'colorful',
          expected: isLinux
            ? ['#0866A7', '#9C7E14', '#C71B7A']
            : ['#01B1FC', '#A48611', '#C72C52'],
        },
        {
          count: 3,
          theme: 'vivid',
          expected: isLinux
            ? ['#B89712', '#C71B7A', '#DC3047']
            : ['#01B1FC', '#A48611', '#D6314D'],
        },
        {
          count: 3,
          theme: 'muted',
          expected: isLinux
            ? ['#03686A', '#064983', '#570513']
            : ['#04524E', '#846E15', '#CD85B7'],
        },
        {
          count: 3,
          theme: 'light',
          expected: isLinux
            ? ['#51C8FE', '#CEC26E', '#D37BAD']
            : ['#5ECBFE', '#CD85B7', '#CFC663'],
        },
        {
          count: 3,
          theme: 'dark',
          expected: isLinux
            ? ['#013764', '#023D33', '#590321']
            : ['#032F55', '#053E2D', '#4A0117'],
        },
      ])(
        'should find the swatches from the palette with the $theme theme',
        ({ count, theme, expected }) => {
          // Act
          const swatches = palette.findSwatches(count, theme as Theme);
          swatches.sort((a, b) => a.color.toInt() - b.color.toInt());

          // Assert
          expect(swatches).toHaveLength(count);

          expect(swatches[0].color).toBeSimilarColor(expected[0]);
          expect(swatches[1].color).toBeSimilarColor(expected[1]);
          expect(swatches[2].color).toBeSimilarColor(expected[2]);
        },
      );
    });
  });
});
