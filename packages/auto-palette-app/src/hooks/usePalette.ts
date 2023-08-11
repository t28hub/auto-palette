import { Position } from 'auto-palette';
import { useEffect, useRef, useState } from 'react';

import { uuid } from '../utils/uuid.ts';
import { InputEvent, OutputEvent } from '../worker/message.ts';
import PaletteWorker from '../worker/worker?worker';

export type Swatch = {
  readonly color: string;
  readonly position: Position;
  readonly isLight: boolean;
};

export type State = {
  readonly result?: Swatch[];
  readonly error?: string;
};

export function usePalette(imageData: ImageData | null): State {
  const [state, setState] = useState<State>({});
  const workerRef = useRef<Worker>();

  useEffect(() => {
    const worker = new PaletteWorker();
    worker.onmessage = (event: MessageEvent<OutputEvent>) => {
      const { id, type, payload } = event.data;
      console.info('Received message:', id, type, payload);
      switch (type) {
        case 'complete': {
          setState({ result: payload.colors });
          break;
        }
        case 'error': {
          setState({ error: payload.message });
          break;
        }
      }
    };
    worker.onmessageerror = (event: MessageEvent) => {
      setState({ error: `Received message error: ${event}` });
    };
    worker.onerror = (event: ErrorEvent) => {
      setState({ error: `Received error: ${event}` });
    };
    workerRef.current = worker;

    return () => {
      console.info('Terminating worker...');
      workerRef.current?.terminate();
    };
  }, []);

  useEffect(() => {
    if (imageData == null) {
      setState({});
      return;
    }

    if (imageData.data.length === 0) {
      setState({});
      return;
    }

    const message: InputEvent = {
      id: uuid(),
      type: 'load',
      payload: {
        width: imageData.width,
        height: imageData.height,
        buffer: imageData.data.buffer,
        channels: 4,
      },
    };
    workerRef.current?.postMessage(message, [imageData.data.buffer]);
  }, [imageData]);

  return state;
}
