import { AutoPalette, Palette } from 'auto-palette';

import { Color } from '../types.ts';
import { isUndefined, UUID } from '../utils';

import type { ErrorMessage, RequestMessage, SuccessMessage } from './message.ts';

/**
 * Declare the property of the {@link WorkerGlobalScope} for TypeScript
 *
 * @see [WorkerGlobalScope.self](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/self)
 */
declare const self: DedicatedWorkerGlobalScope;

/**
 * Class representing the worker server.
 * @internal
 */
export class WorkerServer {
  /**
   * Creates a new `WorkerServer` instance.
   *
   * @returns A new `WorkerServer` instance.
   */
  constructor(private readonly dependency: Promise<AutoPalette>) {
    self.onmessage = this.onMessage.bind(this);
  }

  private async onMessage(message: MessageEvent<RequestMessage>) {
    const { id, type, payload } = message.data;
    if (type !== 'load') {
      this.sendErrorMessage(id, `Unknown message type: ${type}`);
      return;
    }

    const imageData = new ImageData(new Uint8ClampedArray(payload.buffer), payload.width, payload.height);
    try {
      const instance = await this.dependency;
      const palette = instance.extract(imageData);
      this.sendSuccessMessage(id, palette);
    } catch (e) {
      this.sendErrorMessage(id, e);
    }
  }

  private sendSuccessMessage(id: UUID, palette: Palette) {
    const colors = palette.findSwatches(6).map((swatch): Color => {
      const { color, position } = swatch;
      const isLight = color.isLight();
      return { hex: color.toString(), position, isLight };
    });

    const message: SuccessMessage = {
      id,
      type: 'success',
      payload: { colors },
    };
    self.postMessage(message);
  }

  private sendErrorMessage(id: UUID, error: unknown) {
    let payload: { message: string };
    if (error instanceof Error) {
      payload = { message: error.message };
    } else {
      payload = { message: String(error) };
    }

    const message: ErrorMessage = {
      id,
      type: 'error',
      payload,
    };
    self.postMessage(message);
  }
}

// Initialize the worker server if the script is running in a worker.
if (!isUndefined(self) && self instanceof DedicatedWorkerGlobalScope) {
  new WorkerServer(AutoPalette.initialize());
}
