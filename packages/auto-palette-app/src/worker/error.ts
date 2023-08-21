/**
 * Class representing an error that occurred when aborting an operation.
 */
export class AbortError extends Error {
  /**
   * Creates a new `AbortError` instance.
   *
   * @param message - The detailed error message.
   * @returns A new `AbortError` instance.
   */
  constructor(message: string) {
    super(message);
    this.name = 'AbortError';
  }
}

/**
 * Class representing an error that occurred in the worker.
 */
export class WorkerError extends Error {
  /**
   * Creates a new `WorkerError` instance.
   *
   * @param message - The detailed error message.
   * @returns A new `WorkerError` instance.
   */
  constructor(message: string) {
    super(message);
    this.name = 'WorkerError';
  }
}
