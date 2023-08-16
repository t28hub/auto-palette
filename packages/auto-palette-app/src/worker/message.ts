import { ExtractionMethod } from 'auto-palette';

import { Color, ImageObject } from '../types.ts';
import type { UUID } from '../utils';

/**
 * Interface representing a load message.
 */
export interface LoadMessage {
  readonly id: UUID;
  readonly type: 'load';
  readonly payload: ImageObject & {
    readonly method: ExtractionMethod;
    readonly colorCount: number;
  };
}

/**
 * Interface representing a complete message.
 */
export interface SuccessMessage {
  readonly id: UUID;
  readonly type: 'success';
  readonly payload: {
    readonly colors: Color[];
  };
}

/**
 * Interface representing an error message.
 */
export interface ErrorMessage {
  readonly id: UUID;
  readonly type: 'error';
  readonly payload: {
    readonly message: string;
  };
}

/**
 * Type representing a request message.
 */
export type RequestMessage = LoadMessage;

/**
 * Type representing a response message.
 */
export type ResponseMessage = SuccessMessage | ErrorMessage;
