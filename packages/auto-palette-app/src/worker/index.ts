import { WorkerClient } from './client.ts';
import WorkerServer from './server.ts?worker';

export { WorkerError } from './error.ts';

/**
 * Creates a new `WorkerClient` instance.
 *
 * @returns A new `WorkerClient` instance.
 */
export function createClient(): WorkerClient {
  return new WorkerClient(new WorkerServer());
}
