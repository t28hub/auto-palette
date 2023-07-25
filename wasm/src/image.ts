/**
 * The source of an image.
 */
export type ImageSource = HTMLImageElement | HTMLCanvasElement | ImageData | OffscreenCanvas;

/**
 * Retrieves an ImageData instance from the given source.
 *
 * @param source - The source of the image.
 * @returns The ImageData instance.
 */
export function retrieveImageData(source: ImageSource): ImageData {
  if (source instanceof HTMLImageElement) {
    return fromImageElement(source);
  }
  if (source instanceof HTMLCanvasElement) {
    return fromCanvasElement(source);
  }
  if (source instanceof OffscreenCanvas) {
    return fromOffscreenCanvas(source);
  }
  return source;
}

function fromImageElement(image: HTMLImageElement): ImageData {
  const canvas = document.createElement('canvas');
  canvas.width = image.naturalWidth;
  canvas.height = image.naturalHeight;

  const context = canvas.getContext('2d');
  if (!context) {
    throw new Error('Could not create 2d context from canvas element');
  }
  context.drawImage(image, 0, 0, canvas.width, canvas.height);
  return context.getImageData(0, 0, canvas.width, canvas.height);
}

function fromCanvasElement(canvas: HTMLCanvasElement): ImageData {
  const context = canvas.getContext('2d');
  if (!context) {
    throw new Error('Could not create 2d context from canvas element');
  }
  return context.getImageData(0, 0, canvas.width, canvas.height);
}

function fromOffscreenCanvas(canvas: OffscreenCanvas): ImageData {
  const context = canvas.getContext('2d');
  if (!context) {
    throw new Error('Could not create 2d context from OffscreenCanvas');
  }
  return context.getImageData(0, 0, canvas.width, canvas.height);
}
