import {
  type Algorithm,
  Palette,
  type Theme,
  initialize,
} from '@auto-palette/wasm';
import { beforeAll, describe, expect, it } from 'vitest';

const WASM_PATH = '/dist/auto_palette_bg.wasm';
const IMAGE_URL =
  'https://images.unsplash.com/photo-1460751426469-2b744951ebee?ixlib=rb-4.0.3&q=85&fm=jpg&crop=entropy&cs=srgb&w=640';

/**
 * Load an image from a URL and return a Promise that resolves with the HTMLImageElement.
 *
 * @param src - The URL of the image to load.
 * @returns A Promise that resolves with the loaded HTMLImageElement.
 */
function loadImage(src: string): Promise<HTMLImageElement> {
  return new Promise<HTMLImageElement>((resolve, reject) => {
    const image = new Image();
    image.src = src;
    image.crossOrigin = 'anonymous';
    image.onload = () => {
      resolve(image);
    };
    image.onerror = reject;
    image.onabort = reject;
  });
}

/**
 * Create a canvas element with the specified width and height.
 *
 * @param width - The width of the canvas.
 * @param height - The height of the canvas.
 * @returns The created HTMLCanvasElement.
 */
function createCanvas(width: number, height: number): HTMLCanvasElement {
  const canvas = document.createElement('canvas');
  canvas.width = width;
  canvas.height = height;
  return canvas;
}

/**
 * Load image data from a URL and return a Promise that resolves with the ImageData.
 *
 * @param src - The URL of the image to load.
 * @returns A Promise that resolves with the loaded ImageData.
 */
async function loadImageData(src: string): Promise<ImageData> {
  const image = await loadImage(src);
  const canvas = createCanvas(image.width, image.height);
  const context = canvas.getContext('2d', { alpha: true, colorSpace: 'srgb' });
  if (!context) {
    throw new Error('Failed to get canvas context');
  }
  context.drawImage(image, 0, 0);
  return context.getImageData(0, 0, image.width, image.height);
}

describe('@auto-palette/wasm', () => {
  describe('initialize', () => {
    it('should initialize the WebAssembly module from a URL', async () => {
      // Act
      const url = new URL(WASM_PATH, window.location.href);
      await initialize(url);

      // Assert
      expect(Palette).toBeDefined();
    });

    it('should initialize the WebAssembly module from a Response', async () => {
      // Act
      const response = fetch(WASM_PATH);
      await initialize(response);

      // Assert
      expect(Palette).toBeDefined();
    });

    it('should initialize the WebAssembly module from a BufferSource', async () => {
      // Act
      const response = await fetch(WASM_PATH);
      const buffer = response.arrayBuffer();
      await initialize(buffer);

      // Assert
      expect(Palette).toBeDefined();
    });

    it('should initialize the WebAssembly module from a RequestInfo', async () => {
      // Act
      const request = new Request(WASM_PATH);
      await initialize(request);

      // Assert
      expect(Palette).toBeDefined();
    });

    it('should initialize the WebAssembly module from a WebAssembly.Module', async () => {
      // Act
      const response = fetch(WASM_PATH);
      const module = WebAssembly.compileStreaming(response);
      await initialize(module);

      // Assert
      expect(Palette).toBeDefined();
    });

    it.skip('should throw an error if the module cannot be loaded', async () => {
      // Act & Assert
      const module = fetch('/dist/unknown.wasm');
      await expect(initialize(module)).rejects.toThrowError(
        /Failed to initialize WebAssembly module: /,
      );
    });
  });

  describe('Palette', () => {
    beforeAll(async () => {
      const response = fetch(WASM_PATH);
      await initialize(response);
    });

    describe('extract', () => {
      let imageData: ImageData;
      beforeAll(async () => {
        // Arrange
        imageData = await loadImageData(IMAGE_URL);
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

    describe('findSwatches', () => {
      let palette: Palette;
      beforeAll(async () => {
        // Arrange
        const imageData = await loadImageData(IMAGE_URL);
        palette = Palette.extract(imageData, 'dbscan');
      });

      it('should find the swatches from the palette', () => {
        // Act
        const swatches = palette.findSwatches(3);

        // Assert
        expect(swatches).toHaveLength(3);
      });

      it.skip.each([
        {
          count: 3,
          theme: 'colorful',
        },
        {
          count: 3,
          theme: 'vivid',
        },
        {
          count: 3,
          theme: 'muted',
        },
        {
          count: 3,
          theme: 'light',
        },
        {
          count: 3,
          theme: 'dark',
        },
      ])(
        'should find the swatches from the palette with the $theme theme',
        ({ count, theme }) => {
          // Act
          const swatches = palette.findSwatches(count, theme as Theme);

          // Assert
          expect(swatches).toHaveLength(count);
        },
      );
    });
  });
});
