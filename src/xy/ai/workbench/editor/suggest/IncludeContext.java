package xy.ai.workbench.editor.suggest;

record IncludeContext(IncludeContext.Part part, String kind, String prefix, int start, String rest) {

  enum Part {

    /**
     * Directly after {@code [}, before the keyword {@code include} is complete.
     */
    TAG,
    /**
     * After {@code [include }, while typing the kind.
     */
    KIND,
    /**
     * Between {@code (} and {@code )}.
     */
    ARGUMENT
  }
}
