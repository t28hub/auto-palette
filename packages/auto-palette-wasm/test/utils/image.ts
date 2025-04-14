import { createCanvas, loadImage } from '@napi-rs/canvas';

/**
 * Load an image from the specified source.
 *
 * @param source - The source of the image.
 * @returns The loaded image.
 */
export async function loadImageData(source: string): Promise<ImageData> {
  const image = await loadImage(source);
  const canvas = createCanvas(image.width, image.height);
  const context = canvas.getContext('2d', { alpha: true, colorSpace: 'srgb' });
  context.drawImage(image, 0, 0);
  return context.getImageData(0, 0, image.width, image.height) as ImageData;
}
