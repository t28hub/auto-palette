export interface MatcherResult {
  pass: boolean;
  message: () => string;
  actual?: unknown;
  expected?: unknown;
}

export type ExpectationResult = MatcherResult | Promise<MatcherResult>;

export interface MatcherState {
  isNot: boolean;
  promise: string;
}
