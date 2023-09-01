import { useEffect, useRef, useState } from 'react';

import { Color } from '../types.ts';
import { createClient, WorkerError } from '../worker';
import { Options, WorkerClient } from '../worker/client.ts';

export { type Options } from '../worker/client.ts';

/**
 * The return type of the `useAutoPalette` hook.
 */
export type ReturnType = {
  /**
   * The generated colors from the image data.
   */
  readonly colors?: Color[];

  /**
   * The error that occurred while generating the swatches.
   */
  readonly error?: WorkerError;
};

/**
 * Hook that generates a color palette from an image data.
 *
 * @param imageData - The image data to generate the color palette from.
 * @param options - The options for generating the color palette.
 * @returns The return type of the `useAutoPalette` hook.
 */
export function useAutoPalette(imageData?: ImageData, options?: Options): ReturnType {
  const workerRef = useRef<WorkerClient | null>(null);
  const [colors, setColors] = useState<Color[]>();
  const [error, setError] = useState<WorkerError>();

  useEffect(() => {
    workerRef.current = createClient();

    return () => {
      workerRef.current?.terminate();
      workerRef.current = null;
    };
  }, []);

  useEffect(() => {
    setColors(undefined);
    if (imageData == null) {
      return;
    }

    const worker = workerRef.current;
    if (!worker) {
      setError(new WorkerError('Worker is not initialized'));
      return;
    }

    worker
      .extract(imageData, options)
      .then((colors) => {
        setColors(colors);
        setError(undefined);
      })
      .catch((error: WorkerError) => {
        setColors(undefined);
        setError(error);
      });
  }, [imageData, options]);

  return { colors, error };
}
