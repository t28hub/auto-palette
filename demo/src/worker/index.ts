import { InputEvent, OutputEvent } from './message.ts';
import Worker from './worker.ts?worker&inline';
import { UUID } from '../utils/uuid.ts';

/**
 * Type representing a function that can be used to resolve a promise.
 */
export interface ResolutionFunction<T> {
  (value: T | PromiseLike<T>): void;
}

/**
 * Type representing a function that can be used to reject a promise.
 */
export interface RejectionFunction {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  (reason?: any): void;
}

export class WorkerWrapper {
  private readonly callbacks: Map<UUID, [ResolutionFunction<OutputEvent>, RejectionFunction]>;

  /**
   * Creates a new WorkerWrapper instance.
   *
   * @param worker - The worker to wrap.
   * @returns The created WorkerWrapper instance.
   */
  constructor(private readonly worker: Worker = new Worker()) {
    this.callbacks = new Map();
    this.worker.onmessage = this.onMessage.bind(this);
  }

  /**
   * Posts a message to the worker.
   * @param message - The message to post.
   * @param transfer - The transferable objects to transfer.
   * @returns A promise that resolves when the worker responds.
   */
  postMessage(message: InputEvent, transfer: Transferable[] = []): Promise<OutputEvent> {
    return new Promise((resolve: ResolutionFunction<OutputEvent>, reject: RejectionFunction) => {
      this.callbacks.set(message.id, [resolve, reject]);
      this.worker.postMessage(message, transfer);
    });
  }

  /**
   * Terminates the worker.
   */
  terminate(): void {
    this.worker.terminate();
  }

  private onMessage(message: MessageEvent<OutputEvent>): void {
    const { id, type } = message.data;
    const callback = this.callbacks.get(id);
    if (!callback) {
      return;
    }

    const [resolve, reject] = callback;
    this.callbacks.delete(id);

    switch (type) {
      case 'complete':
        resolve(message.data);
        break;
      case 'error':
        reject(message.data);
        break;
      default:
        reject(new Error(`Received unknown message type: ${type}`));
        break;
    }
  }
}
