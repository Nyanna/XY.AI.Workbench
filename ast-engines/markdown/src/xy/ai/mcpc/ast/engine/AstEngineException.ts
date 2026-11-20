export type ErrorKind = 'NOT_FOUND' | 'CONFLICT' | 'SYNTAX';

/** A user-facing, path-free error raised by the engine; carries the HTTP status kind. */
export class AstEngineException extends Error {
  readonly kind: ErrorKind;
  readonly candidates: readonly string[];

  constructor(kind: ErrorKind, message: string, candidates: readonly string[] = []) {
    super(message);
    this.name = 'AstEngineException';
    this.kind = kind;
    this.candidates = candidates;
  }
}
