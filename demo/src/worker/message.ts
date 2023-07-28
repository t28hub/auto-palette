import { UUID } from '../utils/uuid.ts';

/**
 * Type representing a load event.
 */
export type LoadEvent = {
  readonly id: UUID;
  readonly type: 'load';
  readonly payload: {
    readonly width: number;
    readonly height: number;
    readonly buffer: ArrayBuffer;
    readonly channels: number;
  };
};

/**
 * Type representing a complete event.
 */
export type CompleteEvent = {
  readonly id: UUID;
  readonly type: 'complete';
  readonly payload: {
    readonly colors: string[];
  };
};

/**
 * Type representing an error event.
 */
export type ErrorEvent = {
  readonly id: UUID;
  readonly type: 'error';
  readonly payload: {
    readonly message: string;
  };
};

/**
 * Type representing an input event.
 */
export type InputEvent = LoadEvent;

/**
 * Type representing an output event.
 */
export type OutputEvent = CompleteEvent | ErrorEvent;
