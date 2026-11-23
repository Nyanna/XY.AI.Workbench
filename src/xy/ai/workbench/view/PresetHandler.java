package xy.ai.workbench.view;

import java.io.FileWriter;
import java.io.IOException;
import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.HashMap;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import org.eclipse.core.resources.IContainer;
import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IProject;
import org.eclipse.core.resources.IResource;
import org.eclipse.core.resources.IResourceVisitor;
import org.eclipse.core.resources.ResourcesPlugin;
import org.eclipse.core.runtime.CoreException;
import org.eclipse.swt.SWT;
import org.eclipse.swt.widgets.FileDialog;
import org.eclipse.swt.widgets.Shell;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory;
import com.fasterxml.jackson.dataformat.yaml.YAMLGenerator;
import xy.ai.conscious.core.AgentProfile;
import xy.ai.conscious.core.LOG;
import xy.ai.conscious.model.CacheMode;
import xy.ai.conscious.model.Model;
import xy.ai.conscious.model.Reasoning;
import xy.ai.conscious.core.OutputMode;

public class PresetHandler {
  public static final String PROMPT_TXT = ".prompt.txt";
  private static final Pattern FRONT_MATTER = Pattern.compile("\\A---\\s*\\n(.*?\\n)?---\\s*\\n?", Pattern.DOTALL);
  private static final ObjectMapper YAML = new ObjectMapper(new YAMLFactory().disable(YAMLGenerator.Feature.WRITE_DOC_START_MARKER));
  /**
   * Top-level front matter key line, optionally commented out ("key:" or "# key:").
   */
  private static final Pattern FIELD_LINE = Pattern.compile("^(#[ ]?)?(title|tools|outputMode|inputs|rules|model|reasoning|cache|profile):");
  private static final java.util.Set<String> SETTING_KEYS = java.util.Set.of("model", "reasoning", "cache", "profile");

  public static class FrontMatter {
    public String title;
    public List<String> tools;
    public String outputMode;
    public List<String> inputs;
    public Map<String, Map<String, Object>> rules;
    public String model;
    public String reasoning;
    public String cache;
    public String profile;
  }

  /**
   * Session settings stored (initially commented out) in the front matter.
   */
  public static class Settings {
    public String model;
    public String reasoning;
    public String cache;
    public String profile;

    public static Settings of(Model model, Reasoning reasoning, CacheMode cache, AgentProfile profile) {
      Settings s = new Settings();
      s.model = model != null ? model.toString() : null;
      s.reasoning = reasoning != null ? reasoning.name() : null;
      s.cache = cache != null ? cache.name() : null;
      s.profile = profile != null ? profile.name() : null;
      return s;
    }
  }

  public static class Preset {
    public String title;
    public String[] body;
    public String[] tools;
    public OutputMode outputMode;
    public String[] inputs;
    public Map<String, Map<String, Object>> rules;
    public String model;
    public String reasoning;
    public String cache;
    public String profile;
  }

  public static IFile[] listPresetFiles() {
    List<IFile> files = new ArrayList<>();
    for (IProject project : ResourcesPlugin.getWorkspace().getRoot().getProjects()) {
      try {
        IResourceVisitor visitor = resource -> {
          if (resource instanceof IFile) {
            IFile file = (IFile) resource;
            if (file.getName().endsWith(PROMPT_TXT))
              files.add(file);
          }
          return true;
        };
        project.accept(visitor);
      } catch (CoreException e) {
        // return things like project closed
        // LOG.error(e.getMessage(), e);
      }
    }
    files.sort(Comparator.comparing(f -> f.getFullPath().toString()));
    return files.toArray(new IFile[0]);
  }

  public static Preset loadPreset(IFile file) {
    try {
      String content = new String(Files.readAllBytes(file.getLocation().toFile().toPath()), StandardCharsets.UTF_8);
      return parsePreset(content);
    } catch (IOException e) {
      throw new IllegalStateException(e);
    }
  }

  public static Preset parsePreset(String content) {
    Preset preset = new Preset();
    String body = content;
    Matcher matcher = FRONT_MATTER.matcher(content);
    if (matcher.find() && matcher.start() == 0) {
      String yaml = matcher.group(1);
      body = content.substring(matcher.end());
      if (yaml != null && !yaml.isBlank())
        try {
          FrontMatter fm = YAML.readValue(yaml, FrontMatter.class);
          if (fm.title != null && !fm.title.trim().isEmpty())
            preset.title = fm.title.trim();
          if (fm.tools != null)
            preset.tools = fm.tools.toArray(new String[0]);
          if (fm.outputMode != null && !fm.outputMode.isBlank())
            preset.outputMode = OutputMode.valueOf(fm.outputMode.trim());
          if (fm.inputs != null)
            preset.inputs = fm.inputs.toArray(new String[0]);
          if (fm.rules != null)
            preset.rules = fm.rules;
          preset.model = fm.model;
          preset.reasoning = fm.reasoning;
          preset.cache = fm.cache;
          preset.profile = fm.profile;
        } catch (Exception e) {
          LOG.error(e.getMessage(), e);
        }
    }
    if (body.endsWith("\n"))
      body = body.substring(0, body.length() - 1);
    preset.body = body.split("\n", -1);
    return preset;
  }

  /**
   * Asks for a target file and writes the preset.
   *
   * @return true if a file was written
   */
  public static boolean writePreset(String[] body, String[] tools, OutputMode outputMode, String[] inputs, Map<String, Map<String, Object>> rules, Settings settings, Shell shell) {
    FileDialog dialog = new FileDialog(shell, SWT.SAVE);
    dialog.setFilterPath(resolveDefaultSaveDir());
    dialog.setFilterExtensions(new String[] { "*.prompt.txt" });
    dialog.setFilterNames(new String[] { "Prompt Files (*prompt.txt)" });
    String filePath = dialog.open();
    if (filePath == null)
      return false;
    String existing = null;
    java.nio.file.Path target = java.nio.file.Paths.get(filePath);
    if (Files.isRegularFile(target))
      try {
        existing = new String(Files.readAllBytes(target), StandardCharsets.UTF_8);
      } catch (IOException e) {
        LOG.error(e.getMessage(), e);
      }
    String content = serializePreset(body, tools, outputMode, inputs, rules, "", settings, existing);
    try (PrintWriter writer = new PrintWriter(new FileWriter(filePath, StandardCharsets.UTF_8))) {
      writer.print(content);
    } catch (IOException e) {
      throw new IllegalStateException(e);
    }
    refreshWorkspace(filePath);
    return true;
  }

  /**
   * Makes the workspace aware of a file written outside of the resource API.
   */
  private static void refreshWorkspace(String filePath) {
    IFile[] files = ResourcesPlugin.getWorkspace().getRoot().findFilesForLocationURI(new java.io.File(filePath).toURI());
    for (IFile file : files) {
      try {
        IContainer parent = file.getParent();
        IContainer known = parent;
        while (known != null && !known.exists()) known = known.getParent();
        if (known == null)
          continue;
        known.refreshLocal(known == parent ? IResource.DEPTH_ONE : IResource.DEPTH_INFINITE, null);
      } catch (CoreException e) {
        LOG.error(e.getMessage(), e);
      }
    }
  }

  private static String resolveDefaultSaveDir() {
    try {
      java.nio.file.Path projectPath = xy.ai.workbench.Activator.getDefault().editorListener.resolveProjectPath();
      if (projectPath != null)
        return projectPath.toString();
    } catch (Exception e) {
      // fall back to first project below
    }
    IProject[] projects = ResourcesPlugin.getWorkspace().getRoot().getProjects();
    return projects.length > 0 ? projects[0].getLocation().toOSString() : "";
  }

  /**
   * Serializes a preset. If <code>existing</code> (content of the file being
   * overwritten) is given, its title and the comment state of its front matter
   * fields are kept. The settings fields (model, reasoning, cache, profile) are
   * commented out unless the existing file has them active.
   */
  public static String serializePreset(String[] body, String[] tools, OutputMode outputMode, String[] inputs, Map<String, Map<String, Object>> rules, String title, Settings settings, String existing) {
    Map<String, Boolean> commented = new HashMap<>();
    String effectiveTitle = title != null ? title : "";
    if (existing != null) {
      String existingYaml = frontMatterYaml(existing);
      if (existingYaml != null) {
        commented = readCommentState(existingYaml);
        String existingTitle = readExistingTitle(existingYaml);
        if (existingTitle != null)
          effectiveTitle = existingTitle;
      }
    }
    Map<String, Object> fm = new LinkedHashMap<>();
    fm.put("title", effectiveTitle);
    if (tools != null && tools.length > 0)
      fm.put("tools", Arrays.asList(tools));
    if (outputMode != null)
      fm.put("outputMode", outputMode.name());
    if (inputs != null && inputs.length > 0)
      fm.put("inputs", Arrays.asList(inputs));
    if (rules != null && !rules.isEmpty())
      fm.put("rules", rules);
    if (settings != null) {
      if (settings.model != null)
        fm.put("model", settings.model);
      if (settings.reasoning != null)
        fm.put("reasoning", settings.reasoning);
      if (settings.cache != null)
        fm.put("cache", settings.cache);
      if (settings.profile != null)
        fm.put("profile", settings.profile);
    }
    StringBuilder sb = new StringBuilder();
    try {
      String yaml = applyComments(YAML.writeValueAsString(fm), commented);
      sb.append("---\n");
      sb.append(yaml);
      if (!yaml.endsWith("\n"))
        sb.append("\n");
      sb.append("---\n");
    } catch (Exception e) {
      LOG.error(e.getMessage(), e);
    }
    sb.append(String.join("\n", body));
    return sb.toString();
  }

  private static String frontMatterYaml(String content) {
    Matcher matcher = FRONT_MATTER.matcher(content);
    if (matcher.find() && matcher.start() == 0)
      return matcher.group(1);
    return null;
  }

  /**
   * Comment state per top-level front matter key of an existing file (true =
   * commented out). A key counts as commented only if all its occurrences are.
   */
  private static Map<String, Boolean> readCommentState(String yaml) {
    Map<String, Boolean> state = new HashMap<>();
    for (String line : yaml.split("\n")) {
      Matcher m = FIELD_LINE.matcher(line);
      if (m.find())
        state.merge(m.group(2), m.group(1) != null, (a, b) -> a && b);
    }
    return state;
  }

  /**
   * Title of an existing file; an active title wins over a commented one.
   */
  private static String readExistingTitle(String yaml) {
    String commentedTitle = null;
    for (String line : yaml.split("\n")) {
      Matcher m = FIELD_LINE.matcher(line);
      if (!m.find() || !"title".equals(m.group(2)))
        continue;
      String value = parseTitle(line.substring(m.end() - "title:".length()));
      if (value == null)
        continue;
      if (m.group(1) == null)
        return value;
      if (commentedTitle == null)
        commentedTitle = value;
    }
    return commentedTitle;
  }

  private static String parseTitle(String titleLine) {
    try {
      Object value = YAML.readValue(titleLine, Map.class).get("title");
      if (value == null)
        return null;
      String text = value.toString().trim();
      return text.isEmpty() ? null : text;
    } catch (Exception e) {
      return null;
    }
  }

  /**
   * Comments out whole field blocks line by line. Fields without a known state
   * are commented out if they are settings fields, active otherwise.
   */
  private static String applyComments(String yaml, Map<String, Boolean> commented) {
    StringBuilder out = new StringBuilder();
    boolean comment = false;
    for (String line : yaml.split("\n")) {
      Matcher m = FIELD_LINE.matcher(line);
      if (m.find()) {
        String key = m.group(2);
        comment = commented.containsKey(key) ? commented.get(key) : SETTING_KEYS.contains(key);
      }
      out.append(comment ? "# " + line : line).append('\n');
    }
    return out.toString();
  }
}
