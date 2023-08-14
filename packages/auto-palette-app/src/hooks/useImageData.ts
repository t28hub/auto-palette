import { useEffect, useRef, useState } from 'react';

import { isString } from '../utils/guards.ts';

/**
 * The type of scaling to use when resizing the image.
 */
export type ScaleType = 'fill' | 'fit';

/**
 * The options to use when resizing the image.
 */
export type Options = {
  /**
   * The width to resize the image to.
   */
  readonly width?: number;
  /**
   * The height to resize the image to.
   */
  readonly height?: number;
  /**
   * The type of scaling to use when resizing the image.
   */
  readonly scaleType?: ScaleType;
};

/**
 * The return type of the `useImageData` hook.
 */
export type ReturnType = {
  /**
   * The loaded image data.
   */
  readonly imageData: ImageData | null;

  /**
   * The URL of the loading image.
   */
  readonly imageURL: string | null;

  /**
   * The error that occurred while loading the image data.
   */
  readonly error: ImageDataError | null;
};

/**
 * Class representing an error that occurred while loading the image data.
 */
export class ImageDataError extends Error {
  /**
   * Creates a new `ImageDataError` instance.
   *
   * @param message - The detailed error message.
   * @returns A new `ImageDataError` instance.
   */
  constructor(message: string) {
    super(message);
    this.name = 'ImageDataError';
  }
}

/**
 * Hook that loads the image data from a URL or a `File` object.
 *
 * @param source - The URL or `File` object to load the image data from.
 * @param options - The options to use when resizing the image.
 * @returns The return type of the `useImageData` hook.
 */
export function useImageData(source?: string | File, options?: Options): ReturnType {
  const canvasRef = useRef<HTMLCanvasElement>();

  const [image, setImage] = useState<HTMLImageElement | null>(null);
  const [imageURL, setImageURL] = useState<string | null>(null);
  const [imageData, setImageData] = useState<ImageData | null>(null);
  const [error, setError] = useState<ImageDataError | null>(null);

  // Create a canvas element on the first render.
  if (!canvasRef.current) {
    canvasRef.current = document.createElement('canvas');
  }

  useEffect(() => {
    if (!source) {
      setImageURL(null);
      return;
    }

    if (isString(source)) {
      setImageURL(source);
      return;
    }

    const objectURL = URL.createObjectURL(source);
    setImageURL(objectURL);

    return () => {
      URL.revokeObjectURL(objectURL);
    };
  }, [source]);

  useEffect(() => {
    if (!imageURL) {
      setImage(null);
      return;
    }

    let aborted = false;
    const image = new Image();
    image.src = imageURL;
    image.crossOrigin = 'anonymous';
    image.onload = () => {
      if (aborted) {
        return;
      }
      setImage(image);
      setError(null);
    };
    image.onerror = () => {
      if (aborted) {
        return;
      }
      setImage(null);
      setError(new ImageDataError(`Failed to load image from URL: ${imageURL}`));
    };

    return () => {
      aborted = true;
      image.onload = null;
      image.onerror = null;
    };
  }, [imageURL]);

  useEffect(() => {
    if (image === null) {
      setImageData(null);
      return;
    }

    const canvas = canvasRef.current;
    if (!canvas) {
      return;
    }

    const context = canvas.getContext('2d', { willReadFrequently: true });
    if (context === null) {
      setImageData(null);
      setError(new ImageDataError('Failed to get 2D context from canvas'));
      return;
    }

    if (options && options.width && options.height) {
      canvas.width = options.width;
      canvas.height = options.height;
    } else {
      canvas.width = image.naturalWidth;
      canvas.height = image.naturalHeight;
    }

    const scaleType = options?.scaleType ?? 'fit';
    if (scaleType === 'fill') {
      const scale = Math.max(canvas.width / image.naturalWidth, canvas.height / image.naturalHeight);
      const dw = image.naturalWidth * scale;
      const dh = image.naturalHeight * scale;
      const dx = (canvas.width - dw) / 2;
      const dy = (canvas.height - dh) / 2;
      context.drawImage(image, dx, dy, dw, dh);
    } else {
      const scale = Math.min(canvas.width / image.naturalWidth, canvas.height / image.naturalHeight);
      const dw = image.naturalWidth * scale;
      const dh = image.naturalHeight * scale;
      const dx = (canvas.width - dw) / 2;
      const dy = (canvas.height - dh) / 2;
      context.drawImage(image, dx, dy, dw, dh);
    }
    setImageData(context.getImageData(0, 0, canvas.width, canvas.height));
  }, [image, options]);

  return { imageData, imageURL, error };
}
