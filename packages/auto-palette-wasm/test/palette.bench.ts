import { resolve } from 'node:path';
import * as AutoPaletteWasm from '@auto-palette/wasm';
import * as AutoPaletteTs from 'auto-palette';
import { describe, expect, test } from 'vitest';
import { loadImageData } from './utils/image';

const IMAGE_PATH = resolve(process.cwd(), '../../gfx/laura-clugston-pwW2iV9TZao-unsplash.jpg');

describe('benchmark @auto-palette/wasm vs auto-palette-ts', () => {
  test('extract with DBSCAN algorithm in WebAssembly', async ({ bench }) => {
    await bench('extract with DBSCAN algorithm in WebAssembly', async () => {
      // Arrange
      const imageData = await loadImageData(IMAGE_PATH);

      // Act
      const palette = AutoPaletteWasm.Palette.extract(imageData, 'dbscan');
      const swatches = palette.findSwatches(6);

      // Assert
      expect(palette.isEmpty()).toBeFalsy();
      expect(palette.length).toBeGreaterThan(32);
      expect(swatches).toHaveLength(6);
    }).run();
  });

  test('extract with DBSCAN++ algorithm in WebAssembly', async ({ bench }) => {
    await bench('extract with DBSCAN++ algorithm in WebAssembly', async () => {
      // Arrange
      const imageData = await loadImageData(IMAGE_PATH);

      // Act
      const palette = AutoPaletteWasm.Palette.extract(imageData, 'dbscan++');
      const swatches = palette.findSwatches(6);

      // Assert
      expect(palette.isEmpty()).toBeFalsy();
      expect(palette.length).toBeGreaterThan(32);
      expect(swatches).toHaveLength(6);
    }).run();
  });

  test('extract with Kmeans algorithm in WebAssembly', async ({ bench }) => {
    await bench('extract with Kmeans algorithm in WebAssembly', async () => {
      // Arrange
      const imageData = await loadImageData(IMAGE_PATH);

      // Act
      const palette = AutoPaletteWasm.Palette.extract(imageData, 'kmeans');
      const swatches = palette.findSwatches(6);

      // Assert
      expect(palette.isEmpty()).toBeFalsy();
      expect(palette.length).toBeGreaterThan(32);
      expect(swatches).toHaveLength(6);
    }).run();
  });

  test('extract with SLIC algorithm in WebAssembly', async ({ bench }) => {
    await bench('extract with SLIC algorithm in WebAssembly', async () => {
      // Arrange
      const imageData = await loadImageData(IMAGE_PATH);

      // Act
      const palette = AutoPaletteWasm.Palette.extract(imageData, 'slic');
      const swatches = palette.findSwatches(6);

      // Assert
      expect(palette.isEmpty()).toBeFalsy();
      expect(palette.length).toBeGreaterThan(32);
      expect(swatches).toHaveLength(6);
    }).run();
  });

  test('extract with SNIC algorithm in WebAssembly', async ({ bench }) => {
    await bench('extract with SNIC algorithm in WebAssembly', async () => {
      // Arrange
      const imageData = await loadImageData(IMAGE_PATH);

      // Act
      const palette = AutoPaletteWasm.Palette.extract(imageData, 'snic');
      const swatches = palette.findSwatches(6);

      // Assert
      expect(palette.isEmpty()).toBeFalsy();
      expect(palette.length).toBeGreaterThan(32);
      expect(swatches).toHaveLength(6);
    }).run();
  });

  test('extract with DBSCAN algorithm in TypeScript', async ({ bench }) => {
    await bench('extract with DBSCAN algorithm in TypeScript', async () => {
      // Arrange
      const imageData = await loadImageData(IMAGE_PATH);

      // Act
      const palette = AutoPaletteTs.Palette.extract(imageData, {
        algorithm: 'dbscan',
      });
      const swatches = palette.findSwatches(6);

      // Assert
      expect(palette.isEmpty()).toBeFalsy();
      expect(palette.size()).toBeGreaterThan(32);
      expect(swatches).toHaveLength(6);
    }).run();
  });

  test('extract with DBSCAN++ algorithm in TypeScript', async ({ bench }) => {
    await bench('extract with DBSCAN++ algorithm in TypeScript', async () => {
      // Arrange
      const imageData = await loadImageData(IMAGE_PATH);

      // Act
      const palette = AutoPaletteTs.Palette.extract(imageData, {
        algorithm: 'dbscan++',
      });
      const swatches = palette.findSwatches(6);

      // Assert
      expect(palette.isEmpty()).toBeFalsy();
      expect(palette.size()).toBeGreaterThan(32);
      expect(swatches).toHaveLength(6);
    }).run();
  });

  test('extract with Kmeans algorithm in TypeScript', async ({ bench }) => {
    await bench('extract with Kmeans algorithm in TypeScript', async () => {
      // Arrange
      const imageData = await loadImageData(IMAGE_PATH);

      // Act
      const palette = AutoPaletteTs.Palette.extract(imageData, {
        algorithm: 'kmeans',
      });
      const swatches = palette.findSwatches(6);

      // Assert
      expect(palette.isEmpty()).toBeFalsy();
      expect(palette.size()).toBeGreaterThan(32);
      expect(swatches).toHaveLength(6);
    }).run();
  });
});
