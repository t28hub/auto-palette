import { ExtractionMethod } from 'auto-palette';

import { Color } from '../types.ts';
import { uuid, UUID } from '../utils';

import { AbortError, WorkerError } from './error.ts';
import { AbortMessage, ExtractMessage, ResponseMessage } from './message.ts';

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

  /**
   * The signal to use for cancellation.
   */
  readonly signal?: AbortSignal;
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
 * The default options for the extraction.
 */
const DEFAULT_OPTIONS: Required<Pick<Options, 'method' | 'colorCount'>> = {
  method: 'dbscan',
  colorCount: 6,
};

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

    const id = uuid();
    const promise = new Promise<Color[]>((resolve, reject) => {
      const callback = [resolve, reject] as [ResolutionFunction, RejectionFunction];
      this.callbacks.set(id, callback);
    });

    const onAbort = () => {
      this.sendAbortMessage(id);
      const callback = this.callbacks.get(id);
      if (callback) {
        this.callbacks.delete(id);
        const [, reject] = callback;
        reject(new AbortError(`The operation(${id}) was aborted `));
      }
      options?.signal?.removeEventListener('abort', onAbort);
    };

    options?.signal?.addEventListener('abort', onAbort, { once: true });

    this.sendExtractMessage(id, imageData, options);
    return promise;
  }

  terminate() {
    this.worker.terminate();
    this.callbacks.clear();
  }

  private sendExtractMessage(id: UUID, imageData: ImageData, options?: Options) {
    // Clone the data buffer to avoid transferring the same buffer multiple times.
    const { width, height, data } = imageData;
    const { method, colorCount } = { ...DEFAULT_OPTIONS, ...options };
    const clonedData = new Uint8ClampedArray(data);
    const message: ExtractMessage = {
      id,
      type: 'extract',
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
  }

  private sendAbortMessage(id: UUID) {
    const message: AbortMessage = {
      id,
      type: 'abort',
    };
    this.worker.postMessage(message);
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
