import { resolve } from 'node:path';
import * as AutoPaletteWasm from '@auto-palette/wasm';
import * as AutoPaletteTs from 'auto-palette';
import { bench, describe, expect } from 'vitest';
import { loadImageData } from './utils/image';

const IMAGE_PATH = resolve(
  process.cwd(),
  '../../gfx/laura-clugston-pwW2iV9TZao-unsplash.jpg',
);

describe('benchmark @auto-palette/wasm vs auto-palette-ts', () => {
  bench('extract with DBSCAN algorithm in WebAssembly', async () => {
    // Arrange
    const imageData = await loadImageData(IMAGE_PATH);

    // Act
    const palette = AutoPaletteWasm.Palette.extract(imageData, 'dbscan');
    const swatches = palette.findSwatches(6);

    // Assert
    expect(palette.isEmpty()).toBeFalsy();
    expect(palette.length).toBeGreaterThan(32);
    expect(swatches).toHaveLength(6);
  });

  bench('extract with DBSCAN++ algorithm in WebAssembly', async () => {
    // Arrange
    const imageData = await loadImageData(IMAGE_PATH);

    // Act
    const palette = AutoPaletteWasm.Palette.extract(imageData, 'dbscan++');
    const swatches = palette.findSwatches(6);

    // Assert
    expect(palette.isEmpty()).toBeFalsy();
    expect(palette.length).toBeGreaterThan(32);
    expect(swatches).toHaveLength(6);
  });

  bench('extract with Kmeans algorithm in WebAssembly', async () => {
    // Arrange
    const imageData = await loadImageData(IMAGE_PATH);

    // Act
    const palette = AutoPaletteWasm.Palette.extract(imageData, 'kmeans');
    const swatches = palette.findSwatches(6);

    // Assert
    expect(palette.isEmpty()).toBeFalsy();
    expect(palette.length).toBeGreaterThan(32);
    expect(swatches).toHaveLength(6);
  });

  bench('extract with DBSCAN algorithm in TypeScript', async () => {
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
  });

  bench('extract with DBSCAN++ algorithm in TypeScript', async () => {
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
  });

  bench('extract with Kmeans algorithm in TypeScript', async () => {
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
  });
});
