import { Position } from 'auto-palette';
import { useEffect, useRef, useState } from 'react';

import { UUID, uuid } from '../utils/uuid.ts';
import { InputEvent, OutputEvent } from '../worker/message.ts';
import PaletteWorker from '../worker/worker?worker';

/**
 * Interface representing a color swatch.
 */
export type Swatch = {
  /**
   * The color of the swatch.
   */
  readonly color: string;
  /**
   * The position of the swatch in the image.
   */
  readonly position: Position;
  /**
   * Whether the swatch is light.
   */
  readonly isLight: boolean;
};

/**
 * The return type of the `useAutoPalette` hook.
 */
export type ReturnType = {
  /**
   * The generated swatches from the image data.
   */
  readonly swatches: Swatch[] | null;

  /**
   * The error that occurred while generating the swatches.
   */
  readonly error: WorkerError | null;
};

/**
 * Class representing an error that occurred in the worker.
 */
export class WorkerError extends Error {
  /**
   * Creates a new `WorkerError` instance.
   * @param message - The detailed error message.
   * @returns A new `WorkerError` instance.
   */
  constructor(message: string) {
    super(message);
    this.name = 'WorkerError';
  }
}

const DEFAULT_CHANNELS = 4;

/**
 * Hook that generates a color palette from an image data.
 *
 * @param imageData - The image data to generate the color palette from.
 * @returns The return type of the `useAutoPalette` hook.
 */
export function useAutoPalette(imageData?: ImageData): ReturnType {
  const workerRef = useRef<Worker | null>(null);
  const currentIDRef = useRef<UUID | null>(null);
  const [swatches, setSwatches] = useState<Swatch[] | null>(null);
  const [error, setError] = useState<WorkerError | null>(null);

  useEffect(() => {
    const worker = new PaletteWorker();
    worker.onmessage = (event: MessageEvent<OutputEvent>) => {
      const { id, type, payload } = event.data;
      // Ignore messages from previous requests.
      if (id !== currentIDRef.current) {
        return;
      }

      switch (type) {
        case 'complete': {
          setSwatches(payload.colors);
          break;
        }
        case 'error': {
          setError(new WorkerError(payload.message));
          break;
        }
      }
    };
    worker.onmessageerror = () => {
      setError(new WorkerError('Failed to decode message'));
    };
    worker.onerror = (event: ErrorEvent) => {
      setError(new WorkerError(`Received an error event from worker: ${event.message}`));
    };
    workerRef.current = worker;

    return () => {
      worker.terminate();
      workerRef.current = null;
    };
  }, []);

  useEffect(() => {
    if (imageData == null) {
      setSwatches(null);
      return;
    }

    const { width, height, data } = imageData;
    if (width <= 0 || height <= 0) {
      setSwatches(null);
      setError(new WorkerError('Image data has invalid dimensions'));
      return;
    }

    if (data.length !== width * height * DEFAULT_CHANNELS) {
      setSwatches(null);
      setError(new WorkerError('Image data has invalid length'));
      return;
    }

    const id = uuid();
    currentIDRef.current = id;

    const worker = workerRef.current;
    if (!worker) {
      setError(new WorkerError('Worker is not initialized'));
      return;
    }

    // Clone the data buffer to avoid transferring the same buffer multiple times.
    const clonedData = new Uint8ClampedArray(data);
    const message: InputEvent = {
      id,
      type: 'load',
      payload: {
        width,
        height,
        buffer: clonedData.buffer,
        channels: DEFAULT_CHANNELS,
      },
    };
    worker.postMessage(message, [clonedData.buffer]);

    return () => {
      currentIDRef.current = null;
    };
  }, [imageData]);

  return { swatches, error };
}
