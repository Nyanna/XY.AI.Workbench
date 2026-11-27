package xy.ai.workbench.editor.suggest;

import java.util.List;
import java.util.Map;
import java.util.Set;
import xy.ai.conscious.commands.include.IncludeCommandRegistry;

/**
 * What the editor knows about the arguments of each include kind.
 */
class IncludeSyntax {

  enum Argument {

    FILE, DIRECTORY, FIXED
  }
  private static final Map<String, Argument> ARGUMENTS = Map.of("file", Argument.FILE, "files", Argument.FILE, "directory", Argument.DIRECTORY, "contextprompt", Argument.DIRECTORY, "search", Argument.FIXED);
  private static final Map<String, List<String>> FIXED_VALUES = Map.of("search", List.of("files", "matches"));
  private final Set<String> kinds = new IncludeCommandRegistry<Object>(null).kinds();

  Set<String> kinds() {
    return kinds;
  }

  Argument argument(String kind) {
    return kind == null ? Argument.FILE : ARGUMENTS.getOrDefault(kind, Argument.FILE);
  }

  List<String> fixedValues(String kind) {
    return kind == null ? List.of() : FIXED_VALUES.getOrDefault(kind, List.of());
  }
}
