/**
 * Interface representing a 2-dimensional position.
 */
export interface Position {
  readonly x: number;
  readonly y: number;
}

/**
 * Interface representing a RGB color.
 */
export interface RGB {
  readonly r: number;
  readonly g: number;
  readonly b: number;
}

/**
 * Interface representing a CIE L*a*b* color.
 */
export interface Lab {
  readonly l: number;
  readonly a: number;
  readonly b: number;
}
