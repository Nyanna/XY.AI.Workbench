package xy.ai.mcpc.ast.engine;

import java.util.List;

/**
 * A user-facing error, path-free, raised by the engine; carries the HTTP status kind.
 */
public final class AstEngineException extends RuntimeException {
  private static final long serialVersionUID = 1L;

  public enum Kind {

    NOT_FOUND, CONFLICT, SYNTAX
  }
  private final Kind kind;
  private final List<String> candidates;

  public AstEngineException(Kind kind, String message) {
    this(kind, message, List.of());
  }

  public AstEngineException(Kind kind, String message, List<String> candidates) {
    super(message);
    this.kind = kind;
    this.candidates = candidates;
  }

  public Kind kind() {
    return kind;
  }

  public List<String> candidates() {
    return candidates;
  }
}
