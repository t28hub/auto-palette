import { ExtractionMethod } from 'auto-palette';

import { Color } from '../types.ts';
import { uuid, UUID } from '../utils';

import { WorkerError } from './error.ts';
import { LoadMessage, ResponseMessage } from './message.ts';

/**
 * The options for the `useAutoPalette` hook.
 */
export type Options = {
  /**
   * The method to use for color extraction.
   */
  readonly method?: ExtractionMethod;

  /**
   * The number of colors to extract.
   */
  readonly colorCount?: number;
};

/**
 * Type alias for a resolution function for a promise.
 */
type ResolutionFunction = (colors: Color[]) => void;

/**
 * Type alias for a rejection function for a promise.
 */
type RejectionFunction = (error: WorkerError) => void;

/**
 * The default number of channels in an image.
 */
const DEFAULT_CHANNELS = 4;

/**
 * Class representing a worker client.
 */
export class WorkerClient {
  private readonly callbacks: Map<UUID, [ResolutionFunction, RejectionFunction]>;

  /**
   * Creates a new `WorkerClient` instance.
   *
   * @param worker - The worker to communicate with.
   * @returns A new `WorkerClient` instance.
   */
  constructor(private readonly worker: Worker) {
    worker.onmessage = this.onMessage.bind(this);
    worker.onmessageerror = this.onMessageError.bind(this);
    worker.onerror = this.onError.bind(this);
    this.callbacks = new Map();
  }

  /**
   * Extracts the color palette from the given image data.
   *
   * @param imageData - The image data to extract the color palette from.
   * @param options - The options for the color extraction.
   * @returns A promise that resolves when the color palette has been extracted.
   */
  extract(imageData: ImageData, options?: Options): Promise<Color[]> {
    const { width, height, data } = imageData;
    if (width === 0 || height === 0) {
      return Promise.reject(new WorkerError(`Image dimensions are invalid: ${width}x${height}`));
    }

    if (data.length !== width * height * DEFAULT_CHANNELS) {
      return Promise.reject(new WorkerError(`Image data length is invalid: ${data.length}`));
    }

    const method = options?.method ?? 'gmeans';
    const colorCount = options?.colorCount ?? 5;

    const id = uuid();
    const promise = new Promise<Color[]>((resolve, reject) => {
      const callback = [resolve, reject] as [ResolutionFunction, RejectionFunction];
      this.callbacks.set(id, callback);
    });

    // Clone the data buffer to avoid transferring the same buffer multiple times.
    const clonedData = new Uint8ClampedArray(data);
    const message: LoadMessage = {
      id,
      type: 'load',
      payload: {
        width,
        height,
        buffer: clonedData.buffer,
        channels: DEFAULT_CHANNELS,
        method,
        colorCount,
      },
    };
    this.worker.postMessage(message, [clonedData.buffer]);
    return promise;
  }

  terminate() {
    this.worker.terminate();
    this.callbacks.clear();
  }

  private onMessage(message: MessageEvent<ResponseMessage>) {
    const { id, type, payload } = message.data;
    const callback = this.callbacks.get(id);
    if (!callback) {
      return;
    }

    this.callbacks.delete(id);

    const [resolve, reject] = callback;
    switch (type) {
      case 'success':
        resolve(payload.colors);
        break;
      case 'error':
        reject(new WorkerError(payload.message));
        break;
    }
  }

  private onMessageError() {
    this.callbacks.forEach(([, reject]) => {
      reject(new WorkerError('Failed to decode message'));
    });
    this.callbacks.clear();
  }

  private onError(error: ErrorEvent) {
    this.callbacks.forEach(([, reject]) => {
      reject(new WorkerError(`Received an error event from worker: ${error.message}`));
    });
    this.callbacks.clear();
  }
}
