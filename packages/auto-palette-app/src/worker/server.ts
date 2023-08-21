import { AutoPalette, ExtractionMethod, Palette } from 'auto-palette';

import { Color } from '../types.ts';
import { isUndefined, UUID } from '../utils';

import type { RequestMessage, ResponseMessage } from './message.ts';

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
  private readonly aborted: Set<UUID>;

  /**
   * Creates a new `WorkerServer` instance.
   *
   * @returns A new `WorkerServer` instance.
   */
  constructor(private readonly dependency: Promise<AutoPalette>) {
    self.onmessage = this.onMessage.bind(this);
    this.aborted = new Set();
  }

  private async onMessage(message: MessageEvent<RequestMessage>) {
    const { id, type } = message.data;
    switch (type) {
      case 'abort': {
        this.abort(id);
        break;
      }
      case 'extract': {
        const { payload } = message.data;
        const imageData = new ImageData(new Uint8ClampedArray(payload.buffer), payload.width, payload.height);
        await this.extract(id, imageData, payload.method, payload.colorCount);
        break;
      }
      default: {
        this.sendErrorMessage(id, `Unknown message type: ${type}`);
        break;
      }
    }
  }

  private async extract(id: UUID, imageData: ImageData, method: ExtractionMethod, colorCount: number) {
    try {
      const instance = await this.dependency;
      if (this.aborted.has(id)) {
        return;
      }
      const palette = instance.extract(imageData, method);
      this.sendSuccessMessage(id, palette, colorCount);
    } catch (e) {
      this.sendErrorMessage(id, e);
    } finally {
      this.aborted.delete(id);
    }
  }

  private abort(id: UUID) {
    this.aborted.add(id);
  }

  private sendSuccessMessage(id: UUID, palette: Palette, colorCount: number) {
    const colors = palette.findSwatches(colorCount).map((swatch): Color => {
      const { color, position } = swatch;
      const isLight = color.isLight();
      return { hex: color.toString(), position, isLight };
    });

    this.sendMessage({
      id,
      type: 'success',
      payload: { colors },
    });
  }

  private sendErrorMessage(id: UUID, error: unknown) {
    let payload: { message: string };
    if (error instanceof Error) {
      payload = { message: error.message };
    } else {
      payload = { message: String(error) };
    }

    this.sendMessage({
      id,
      type: 'error',
      payload,
    });
  }

  private sendMessage(message: ResponseMessage) {
    if (this.aborted.has(message.id)) {
      return;
    }
    self.postMessage(message);
  }
}

// Initialize the worker server if the script is running in a worker.
if (!isUndefined(self) && self instanceof DedicatedWorkerGlobalScope) {
  new WorkerServer(AutoPalette.initialize());
}
