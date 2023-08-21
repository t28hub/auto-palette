import { ExtractionMethod } from 'auto-palette';

import { Color, ImageObject } from '../types.ts';
import type { UUID } from '../utils';

/**
 * Interface representing an extract message.
 */
export interface ExtractMessage {
  readonly id: UUID;
  readonly type: 'extract';
  readonly payload: ImageObject & {
    readonly method: ExtractionMethod;
    readonly colorCount: number;
  };
}

/**
 * Interface representing an abort message.
 */
export interface AbortMessage {
  readonly id: UUID;
  readonly type: 'abort';
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
export type RequestMessage = AbortMessage | ExtractMessage;

/**
 * Type representing a response message.
 */
export type ResponseMessage = SuccessMessage | ErrorMessage;
