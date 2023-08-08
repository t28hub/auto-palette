import { Position } from 'auto-palette';

import type { UUID } from '../utils/uuid.ts';

/**
 * Interface representing a load event.
 */
export interface LoadEvent {
  readonly id: UUID;
  readonly type: 'load';
  readonly payload: {
    readonly width: number;
    readonly height: number;
    readonly buffer: ArrayBuffer;
    readonly channels: number;
  };
}

/**
 * Interface representing a complete event.
 */
export interface CompleteEvent {
  readonly id: UUID;
  readonly type: 'complete';
  readonly payload: {
    readonly colors: {
      readonly color: string;
      readonly position: Position;
      readonly isLight: boolean;
    }[];
  };
}

/**
 * Interface representing an error event.
 */
export interface ErrorEvent {
  readonly id: UUID;
  readonly type: 'error';
  readonly payload: {
    readonly message: string;
  };
}

/**
 * Type representing an input event.
 */
export type InputEvent = LoadEvent;

/**
 * Type representing an output event.
 */
export type OutputEvent = CompleteEvent | ErrorEvent;
