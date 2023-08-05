declare const validUUID: unique symbol;

/**
 * Type representing an UUID.
 */
export type UUID = string & {
  readonly [validUUID]: true;
};

/**
 * Generates a new UUID v4.
 *
 * @returns The generated UUID.
 */
export function uuid(): UUID {
  return crypto.randomUUID() as UUID;
}
