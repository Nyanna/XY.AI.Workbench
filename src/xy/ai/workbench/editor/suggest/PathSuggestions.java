package xy.ai.workbench.editor.suggest;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.InvalidPathException;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Comparator;
import java.util.List;
import java.util.Locale;
import java.util.stream.Collectors;
import java.util.stream.Stream;

/** Lists file system entries for a partially typed include path. */
class PathSuggestions {

  private static final int MAX_ENTRIES = 200;

  record Entry(Path path, boolean directory) {
    String name() {
      return path.getFileName() == null ? path.toString() : path.getFileName().toString();
    }
  }

  /**
   * @param typed path text typed so far, without a leading backtick
   * @param baseDirectory directory for relative input, may be {@code null}
   * @param directoriesOnly whether files are excluded
   */
  List<Entry> list(String typed, Path baseDirectory, boolean directoriesOnly) {
    int separator = Math.max(typed.lastIndexOf('/'), typed.lastIndexOf('\\'));
    String directoryText = typed.substring(0, separator + 1);
    String namePrefix = typed.substring(separator + 1).toLowerCase(Locale.ROOT);
    Path directory = resolve(directoryText, baseDirectory);
    if (directory == null || !Files.isDirectory(directory))
      return List.of();
    boolean hidden = namePrefix.startsWith(".");
    try (Stream<Path> children = Files.list(directory)) {
      return children.map(p -> new Entry(p.toAbsolutePath().normalize(), Files.isDirectory(p)))
          .filter(e -> !directoriesOnly || e.directory())
          .filter(e -> e.name().toLowerCase(Locale.ROOT).startsWith(namePrefix))
          .filter(e -> hidden || !e.name().startsWith("."))
          .sorted(Comparator.comparing((Entry e) -> !e.directory()).thenComparing(e -> e.name().toLowerCase(Locale.ROOT)))
          .limit(MAX_ENTRIES)
          .collect(Collectors.toList());
    } catch (IOException e) {
      return List.of();
    }
  }

  private Path resolve(String directoryText, Path baseDirectory) {
    try {
      if (directoryText.isEmpty())
        return baseDirectory;
      Path directory = Paths.get(directoryText);
      if (directory.isAbsolute())
        return directory;
      return baseDirectory == null ? null : baseDirectory.resolve(directory);
    } catch (InvalidPathException e) {
      return null;
    }
  }
}
