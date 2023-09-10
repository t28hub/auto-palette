import { encode } from 'blurhash';
import { useEffect, useState } from 'react';

/**
 * The options to use when encoding the image.
 */
export interface Options {
  readonly componentX: number;
  readonly componentY: number;
}

/**
 * The return type of the `useBlurHash` hook.
 */
export interface ReturnType {
  readonly hash: string | null;
  readonly error: BlurHashError | null;
}

/**
 * Class representing an error that occurred while encoding the image data.
 */
export class BlurHashError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'BlurHashError';
  }
}

/**
 * A React hook for encoding an image using BlurHash.
 *
 * @param imageData - The image data to encode.
 * @param options - The options to use when encoding the image.
 * @returns The BlurHash of the image.
 */
export function useBlurHash(imageData?: ImageData, options?: Options): ReturnType {
  const [hash, setHash] = useState<string | null>(null);
  const [error, setError] = useState<BlurHashError | null>(null);

  useEffect(() => {
    if (!imageData) {
      setHash(null);
      setError(null);
      return;
    }

    try {
      const componentX = options?.componentX ?? 4;
      const componentY = options?.componentY ?? 4;
      const encoded = encode(imageData.data, imageData.width, imageData.height, componentX, componentY);
      setHash(encoded);
    } catch (e) {
      setError(
        new BlurHashError(`Failed to encode the image data: ${e instanceof Error ? e.message : 'Unknown error'}`),
      );
    }
  }, [imageData, options]);

  return { hash, error };
}
