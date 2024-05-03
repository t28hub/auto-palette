import { beforeAll, describe, expect, it } from 'vitest';

import { AlgorithmName, AutoPalette } from '../src';
import { loadAsImageData } from './utils';

describe('auto-palette', () => {
  let instance: AutoPalette;
  beforeAll(async () => {
    instance = await AutoPalette.initialize();
  });

  describe('extract', () => {
    let imageData: ImageData;
    beforeAll(async () => {
      imageData = await loadAsImageData('https://picsum.photos/id/360/640/360/');
    });

    it('should extract a color palette from an image', () => {
      // Act
      const actual = instance.extract(imageData);

      // Assert
      expect(actual.isEmpty()).toBeFalsy();
      expect(actual.length).toBeGreaterThan(16);

      const swatches = actual.findSwatches(5);
      for (const swatch of swatches) {
        const { color, position, population } = swatch;
        console.info({
          color: color.toHexString(),
          population,
          position,
        });
      }
    });

    it.each(['kmeans', 'dbscan', 'dbscan++'])(
      'should extract a color palette from an image using %s algorithm',
      (algorithm: AlgorithmName) => {
        // Act
        const actual = instance.extract(imageData, algorithm);

        // Assert
        expect(actual.isEmpty()).toBeFalsy();
        expect(actual.length).toBeGreaterThan(16);
      },
    );
  });
});
