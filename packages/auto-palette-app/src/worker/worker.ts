import { AutoPalette } from 'auto-palette';

import type { CompleteEvent, ErrorEvent, InputEvent } from './message.ts';

/**
 * Declare the property of the {@link WorkerGlobalScope} for TypeScript
 *
 * @see [WorkerGlobalScope.self](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/self)
 */
declare const self: DedicatedWorkerGlobalScope;

const autoPalette: Promise<AutoPalette> = AutoPalette.initialize();

self.addEventListener('message', (event: MessageEvent<InputEvent>) => {
  const { id, type, payload } = event.data;
  switch (type) {
    case 'load': {
      const { width, height, buffer } = payload;
      const imageData = new ImageData(new Uint8ClampedArray(buffer), width, height);
      autoPalette
        .then((autoPalette) => {
          try {
            const palette = autoPalette.extract(imageData);
            const colors = palette.findSwatches(6).map((swatch) => swatch.color.toString());
            const event: CompleteEvent = {
              id,
              type: 'complete',
              payload: {
                colors,
              },
            };
            self.postMessage(event);
          } catch (e) {
            const error: ErrorEvent = {
              id,
              type: 'error',
              payload: {
                message: `${e}`,
              },
            };
            self.postMessage(error);
          }
        })
        .catch(() => {
          const event: ErrorEvent = {
            id,
            type: 'error',
            payload: {
              message: 'Failed to initialize AutoPalette.',
            },
          };
          self.postMessage(event);
        });
      break;
    }
    default: {
      const event: ErrorEvent = {
        id,
        type: 'error',
        payload: {
          message: `Received unknown message type: ${type}`,
        },
      };
      self.postMessage(event);
      break;
    }
  }
});
