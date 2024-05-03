import { describe, expect, it } from 'vitest';

import { createCanvas, loadImage } from '@napi-rs/canvas';
import { Palette } from '../src';

describe('auto-palette', () => {
  describe('extract', () => {
    it('should extract a color palette from an image', async () => {
      // Arrange
      const image = await loadImage('../../crates/core/tests/assets/olympic_rings.png');
      const canvas = createCanvas(image.width, image.height);
      const context = canvas.getContext('2d', { alpha: true });
      context.drawImage(image, 0, 0, image.width, image.height);
      const imageData = context.getImageData(0, 0, image.width, image.height);

      // Act
      const actual = Palette.extract(imageData);

      // Assert
      expect(actual.length).toBe(6);
      expect(actual.isEmpty()).toBeFalsy();

      const swatches = actual.findSwatches(5);
      for (const swatch of swatches) {
        console.info(swatch.color().toRGB());
      }
    });
  });
});
