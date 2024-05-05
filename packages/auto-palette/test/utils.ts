/**
 * Loads an image from a URL
 *
 * @param path - The path to the image file.
 * @returns A Promise that resolves to the loaded image as an HTMLImageElement.
 */
function loadImage(path: string): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const image = new Image();
    image.onload = () => {
      resolve(image);
    };
    image.onerror = reject;
    image.crossOrigin = 'anonymous';
    image.src = path;
  });
}

/**
 * Loads an image from a given file as ImageData object.
 *
 * @param path - The path to the image file.
 * @returns A Promise that resolves to the loaded image as ImageData object.
 */
export async function loadAsImageData(path: string): Promise<ImageData> {
  const image = await loadImage(path);
  const canvas = document.createElement('canvas');
  canvas.width = image.width;
  canvas.height = image.height;

  const context = canvas.getContext('2d', { alpha: true });
  if (!context) {
    throw new Error('Canvas 2D context is not supported in this environment');
  }
  context.drawImage(image, 0, 0, image.width, image.height);
  return context.getImageData(0, 0, image.width, image.height);
}
