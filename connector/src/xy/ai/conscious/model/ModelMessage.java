package xy.ai.conscious.model;

/**
 * A single message emitted by the model, kept separate from the rendered answer.
 */
public record ModelMessage(Kind kind, String text) {

  public enum Kind {
    Text, Thinking, Reasoning
  }
}
