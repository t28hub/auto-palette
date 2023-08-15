import { useEffect, useRef, useState } from 'react';

import { Color } from '../types.ts';
import { createClient, WorkerError } from '../worker';
import { WorkerClient } from '../worker/client.ts';

/**
 * The return type of the `useAutoPalette` hook.
 */
export type ReturnType = {
  /**
   * The generated colors from the image data.
   */
  readonly colors: Color[] | null;

  /**
   * The error that occurred while generating the swatches.
   */
  readonly error: WorkerError | null;
};

/**
 * Hook that generates a color palette from an image data.
 *
 * @param imageData - The image data to generate the color palette from.
 * @returns The return type of the `useAutoPalette` hook.
 */
export function useAutoPalette(imageData?: ImageData): ReturnType {
  const workerRef = useRef<WorkerClient | null>(null);
  const [colors, setColors] = useState<Color[] | null>(null);
  const [error, setError] = useState<WorkerError | null>(null);

  useEffect(() => {
    workerRef.current = createClient();

    return () => {
      workerRef.current?.terminate();
      workerRef.current = null;
    };
  }, []);

  useEffect(() => {
    setColors(null);
    if (imageData == null) {
      return;
    }

    const worker = workerRef.current;
    if (!worker) {
      setError(new WorkerError('Worker is not initialized'));
      return;
    }

    worker
      .extract(imageData)
      .then((colors) => {
        setColors(colors);
        setError(null);
      })
      .catch((error: WorkerError) => {
        setColors(null);
        setError(error);
      });
  }, [imageData]);

  return { colors, error };
}
