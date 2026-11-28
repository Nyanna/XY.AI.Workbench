package xy.ai.workbench.editor.drag;

import java.nio.file.Files;
import java.nio.file.InvalidPathException;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

/**
 * Builds include tags for dropped file system entries: {@code [include](path)} for files and
 * {@code [include directory](path)} for directories.
 */
class IncludeTags {

  static List<String> of(String[] paths) {
    List<String> tags = new ArrayList<>();
    for (String raw : paths) {
      try {
        tags.add(of(Path.of(raw).toAbsolutePath().normalize()));
      } catch (InvalidPathException e) {
        // not a usable path, skip it
      }
    }
    return tags;
  }

  static String of(Path path) {
    String text = path.toString();
    if (text.chars().anyMatch(c -> Character.isWhitespace(c) || c == ')'))
      text = "`" + text + "`";
    String kind = Files.isDirectory(path) ? "include directory" : "include";
    return "[" + kind + "](" + text + ")";
  }
}
