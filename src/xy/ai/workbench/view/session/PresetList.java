package xy.ai.workbench.view.session;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import org.eclipse.core.resources.IFile;
import org.eclipse.core.runtime.IPath;
import xy.ai.workbench.view.PresetHandler;

/**
 * Discovers prompt preset files across the workspace, deduplicates them by
 * absolute file location and computes unique, human-readable display
 * labels, separately from any UI widget.
 */
public class PresetList {

  public static class Entry {
    public final IFile file;
    public final String label;

    Entry(IFile file, String label) {
      this.file = file;
      this.label = label;
    }
  }

  private static class Raw {
    IFile file;
    String title;
  }

  public List<Entry> load() {
    List<IFile> files = dedupe(PresetHandler.listPresetFiles());
    List<Raw> raws = new ArrayList<>();
    for (IFile file : files) {
      Raw raw = new Raw();
      raw.file = file;
      raw.title = readTitle(file);
      raws.add(raw);
    }
    Map<String, String> labels = computeLabels(raws);
    List<Entry> entries = new ArrayList<>();
    for (Raw raw : raws) entries.add(new Entry(raw.file, labels.get(key(raw.file))));
    return entries;
  }

  private String readTitle(IFile file) {
    try {
      return PresetHandler.loadPreset(file).title;
    } catch (Exception e) {
      return null;
    }
  }

  private List<IFile> dedupe(IFile[] files) {
    List<IFile> result = new ArrayList<>();
    Set<String> seen = new HashSet<>();
    for (IFile file : files) if (seen.add(key(file)))
      result.add(file);
    return result;
  }

  private String key(IFile file) {
    return file.getLocation() != null ? file.getLocation().toOSString() : file.getFullPath().toOSString();
  }

  private Map<String, String> computeLabels(List<Raw> raws) {
    Map<String, String> result = new HashMap<>();
    Map<String, List<Raw>> titled = new LinkedHashMap<>();
    List<Raw> untitled = new ArrayList<>();
    for (Raw raw : raws) {
      if (raw.title != null && !raw.title.trim().isEmpty())
        titled.computeIfAbsent(raw.title.trim(), t -> new ArrayList<>()).add(raw);
      else
        untitled.add(raw);
    }
    for (Map.Entry<String, List<Raw>> group : titled.entrySet()) {
      boolean ambiguous = group.getValue().size() > 1;
      for (Raw raw : group.getValue()) {
        String label = ambiguous ? group.getKey() + " (" + raw.file.getProject().getName() + ")" : group.getKey();
        result.put(key(raw.file), label);
      }
    }
    Map<String, List<Raw>> byBaseName = new LinkedHashMap<>();
    for (Raw raw : untitled) byBaseName.computeIfAbsent(baseName(raw.file), n -> new ArrayList<>()).add(raw);
    for (Map.Entry<String, List<Raw>> group : byBaseName.entrySet()) resolveUntitledGroup(group.getKey(), group.getValue(), result);
    return result;
  }

  private String baseName(IFile file) {
    String name = file.getName();
    if (name.endsWith(PresetHandler.PROMPT_TXT))
      name = name.substring(0, name.length() - PresetHandler.PROMPT_TXT.length());
    return name;
  }

  private void resolveUntitledGroup(String base, List<Raw> group, Map<String, String> result) {
    if (group.size() == 1) {
      result.put(key(group.get(0).file), base);
      return;
    }
    List<Raw> remaining = group;
    int level = 1;
    while (!remaining.isEmpty()) {
      Map<String, List<Raw>> byLabel = new LinkedHashMap<>();
      for (Raw raw : remaining) {
        String label = base + " (" + lastSegments(raw.file, level) + ")";
        byLabel.computeIfAbsent(label, l -> new ArrayList<>()).add(raw);
      }
      List<Raw> stillColliding = new ArrayList<>();
      for (Map.Entry<String, List<Raw>> candidate : byLabel.entrySet()) {
        List<Raw> entries = candidate.getValue();
        if (entries.size() == 1) {
          result.put(key(entries.get(0).file), candidate.getKey());
          continue;
        }
        final int currentLevel = level;
        boolean reachedProjectRoot = entries.stream().allMatch(raw -> segmentCount(raw.file) <= currentLevel);
        if (reachedProjectRoot)
          for (Raw raw : entries) result.put(key(raw.file), candidate.getKey());
        else
          stillColliding.addAll(entries);
      }
      remaining = stillColliding;
      level++;
    }
  }

  private int segmentCount(IFile file) {
    return file.getFullPath().removeLastSegments(1).segmentCount();
  }

  private String lastSegments(IFile file, int count) {
    IPath dirPath = file.getFullPath().removeLastSegments(1);
    String[] segments = dirPath.segments();
    int from = Math.max(0, segments.length - count);
    StringBuilder sb = new StringBuilder();
    for (int i = from; i < segments.length; i++) {
      if (sb.length() > 0)
        sb.append("/");
      sb.append(segments[i]);
    }
    return sb.toString();
  }
}
