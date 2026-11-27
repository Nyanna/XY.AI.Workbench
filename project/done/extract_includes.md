Lagere das Include Handling von `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java` in einzelne IncludeCommands in das Package `/home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include` aus.
Die extrahierten Klassen sollen ein gemeinsames Interface implementieren.
Ziel ist alle Kommando oder Include Verarbeitung an einem Ort zu organisieren.
SystemInit:  id=7ac8ba7a-cb8b-4d58-af16-0a517e4afb65, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 78b3-1-0
toolName: ast_retrieve
arguments:
  outline:
  - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
  reason: Understand structure of IncludeProcessor
```
/answer 78b3-1-0 allow
Control Request:
```yaml
id: 78b3-2-1
result:
  structuredContent:
    outline:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
        nodes:
        - id: 4EqrOs|xE1LS6
          excert: import java.io.IOException;
        - id: IncludeProcessor
          excert: "public class IncludeProcessor<M> implements ISessionIncludeProcessor<M> {"
          children:
          - id: IncludeProcessor.mNpHXF|W6hBct
            excert: "public static final Pattern INCLUDE_LINE = Pattern.compile(\"^(\\\\s*\\\\[include(?:…"
          - id: IncludeProcessor.IncludeProcessor
            excert: public IncludeProcessor(IIncludeAdapter adapter)
          - id: IncludeProcessor.include
            excert: "public void include(String kind, String arg, List<Path> chain, SessionCallbacks…"
          - id: IncludeProcessor.processText
            excert: "public void processText(String text, List<Path> chain, SessionCallbacks<M> call…"
          - id: IncludeProcessor.flush
            excert: "private void flush(StringBuilder buffer, SessionCallbacks<M> call, List<M> out)"
          - id: IncludeProcessor.includePath
            excert: "private void includePath(String pathText, List<Path> chain, SessionCallbacks<M>…"
          - id: IncludeProcessor.includeDirectory
            excert: "private void includeDirectory(String dir, List<Path> chain, SessionCallbacks<M>…"
          - id: IncludeProcessor.includeContextPrompt
            excert: "private void includeContextPrompt(String dir, SessionCallbacks<M> call, List<M>…"
          - id: IncludeProcessor.includeFiles
            excert: "private void includeFiles(String arg, SessionCallbacks<M> call, List<M> out)"
          - id: IncludeProcessor.includeFile
            excert: "private void includeFile(String path, SessionCallbacks<M> call, List<M> out)"
          - id: IncludeProcessor.includeSearch
            excert: "private void includeSearch(String arg, SessionCallbacks<M> call, List<M> out)"
          - id: IncludeProcessor.renderMatches
            excert: private String renderMatches(List<IIncludeAdapter.Match> matches)
          - id: IncludeProcessor.requireAdapter
            excert: private IIncludeAdapter requireAdapter()
```
/answer 78b3-2-1 allow
Control Request:
```yaml
id: 78b3-3-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious
    pattern: IncludeProcessor
    include:
    - '*.java'
    limit: 50
  reason: Find usages of IncludeProcessor and existing commands package
```
/answer 78b3-3-0 allow
Control Request:
```yaml
id: 78b3-4-3
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious
        matches:
        - path: harness/ISessionIncludeProcessor.java
          matches:
          - "public interface ISessionIncludeProcessor<M> {"
          - "public void processText(String text, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv);"
        - path: harness/SessionProcessor.java
          matches:
          - "public class SessionProcessor<M> implements ISessionIncludeProcessor<M> {"
          - private final IncludeProcessor<M> includeProcessor;
          - this.includeProcessor = new IncludeProcessor<>(adapter);
          - "public void processText(String text, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {"
          - "if (IncludeProcessor.INCLUDE_LINE.matcher(line).matches()) {"
          - Matcher m = IncludeProcessor.INCLUDE_TAG.matcher(line);
          - return IncludeProcessor.INCLUDE_LINE.matcher(line).matches() || s.equals(IEditorInterface.USER) || s.equals(IEditorInterface.AGENT) || s.equals(IEdito…| s.equals(IEditorInterface.TEXT) || s.equals(IEditorInterface.TOOLUSE) || s.equals(IEditorInterface.TOOLRESULT) || CommandRegistry.detect(s) != null;
        - path: harness/IncludeProcessor.java
          matches:
          - "public class IncludeProcessor<M> implements ISessionIncludeProcessor<M> {"
          - "public IncludeProcessor(IIncludeAdapter adapter) {"
          - "public void include(String kind, String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {"
          - "public void processText(String text, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {"
          - "private void includePath(String pathText, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {"
          - "private void includeDirectory(String dir, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {"
        - path: connector/claudecode/CCConnector.java
          matches:
          - import xy.ai.conscious.harness.IncludeProcessor;
          - private final IncludeProcessor<String> includeProcessor;
          - this.includeProcessor = new IncludeProcessor<>(includeAdapter);
        warnings:
        - 1 match line(s) exceeded 300 characters and were shortened in the middle.
```
/answer 78b3-4-3 allow
Control Request:
```yaml
id: 78b3-5-0
toolName: ast_retrieve
arguments:
  read:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/ISessionIncludeProcessor.java
  reason: Read full source
```
/answer 78b3-5-0 allow
Control Request:
```yaml
id: 78b3-6-5
result:
  structuredContent:
    read:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
        nodes:
        - id: 4EqrOs|xE1LS6
          excert: import java.io.IOException;
        - id: IncludeProcessor
          excert: "public class IncludeProcessor<M> implements ISessionIncludeProcessor<M> {"
          children:
          - id: IncludeProcessor.mNpHXF|W6hBct
            excert: "public static final Pattern INCLUDE_LINE = Pattern.compile(\"^(\\\\s*\\\\[include(?:…"
          - id: IncludeProcessor.IncludeProcessor
            excert: public IncludeProcessor(IIncludeAdapter adapter)
          - id: IncludeProcessor.include
            excert: "public void include(String kind, String arg, List<Path> chain, SessionCallbacks…"
          - id: IncludeProcessor.processText
            excert: "public void processText(String text, List<Path> chain, SessionCallbacks<M> call…"
          - id: IncludeProcessor.flush
            excert: "private void flush(StringBuilder buffer, SessionCallbacks<M> call, List<M> out)"
          - id: IncludeProcessor.includePath
            excert: "private void includePath(String pathText, List<Path> chain, SessionCallbacks<M>…"
          - id: IncludeProcessor.includeDirectory
            excert: "private void includeDirectory(String dir, List<Path> chain, SessionCallbacks<M>…"
          - id: IncludeProcessor.includeContextPrompt
            excert: "private void includeContextPrompt(String dir, SessionCallbacks<M> call, List<M>…"
          - id: IncludeProcessor.includeFiles
            excert: "private void includeFiles(String arg, SessionCallbacks<M> call, List<M> out)"
          - id: IncludeProcessor.includeFile
            excert: "private void includeFile(String path, SessionCallbacks<M> call, List<M> out)"
          - id: IncludeProcessor.includeSearch
            excert: "private void includeSearch(String arg, SessionCallbacks<M> call, List<M> out)"
          - id: IncludeProcessor.renderMatches
            excert: private String renderMatches(List<IIncludeAdapter.Match> matches)
          - id: IncludeProcessor.requireAdapter
            excert: private IIncludeAdapter requireAdapter()
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java instead."
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/ISessionIncludeProcessor.java
        nodes:
        - id: vsldXD|VjXVVc
          excert: import java.nio.file.Path;
        - id: ISessionIncludeProcessor
          excert: "public interface ISessionIncludeProcessor<M> {"
          children:
          - id: ISessionIncludeProcessor.processText
            excert: "public abstract void processText(String text, List<Path> chain, SessionCallback…"
            docstring: "Processes {@code text} as a flat document: every {@code [include...]} line is"
        warnings:
        - "ast_read fell back to ast_outline: no ids were given; returning the full outline of /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/ISessionIncludeProcessor.java instead."
```
/answer 78b3-6-5 allow
Control Request:
```yaml
id: 78b3-7-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious
    pattern: includeProcessor|INCLUDE_TAG|INCLUDE_LINE
    include:
    - '*.java'
    limit: 50
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands
    pattern: ^(public )?(final )?(abstract )?(class|interface|enum)
    include:
    - '*.java'
    limit: 50
  reason: Find usages and see existing commands package conventions
```
/answer 78b3-7-0 allow
Control Request:
```yaml
id: 78b3-8-7
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious
        matches:
        - path: harness/SessionProcessor.java
          matches:
          - private final IncludeProcessor<M> includeProcessor;
          - this.includeProcessor = new IncludeProcessor<>(adapter);
          - "if (IncludeProcessor.INCLUDE_LINE.matcher(line).matches()) {"
          - Matcher m = IncludeProcessor.INCLUDE_TAG.matcher(line);
          - "while (m.find()) includeProcessor.include(m.group(1), m.group(2).strip(), chain, call, out, recursiv);"
          - return IncludeProcessor.INCLUDE_LINE.matcher(line).matches() || s.equals(IEditorInterface.USER) || s.equals(IEditorInterface.AGENT) || s.equals(IEdito…| s.equals(IEditorInterface.TEXT) || s.equals(IEditorInterface.TOOLUSE) || s.equals(IEditorInterface.TOOLRESULT) || CommandRegistry.detect(s) != null;
        - path: harness/IncludeProcessor.java
          matches:
          - "public static final Pattern INCLUDE_LINE = Pattern.compile(\"^(\\\\s*\\\\[include(?:\\\\s+\\\\w+)?\\\\]\\\\([^)]*\\\\)\\\\s*)+$\");"
          - "public static final Pattern INCLUDE_TAG = Pattern.compile(\"\\\\[include(?:\\\\s+(\\\\w+))?\\\\]\\\\(([^)]*)\\\\)\");"
          - "if (INCLUDE_LINE.matcher(line).matches()) {"
          - Matcher m = INCLUDE_TAG.matcher(line);
        - path: connector/claudecode/CCConnector.java
          matches:
          - private final IncludeProcessor<String> includeProcessor;
          - this.includeProcessor = new IncludeProcessor<>(includeAdapter);
          - "includeProcessor.processText(input, List.of(), callbacks, out, includeProcessor);"
        warnings:
        - 1 match line(s) exceeded 300 characters and were shortened in the middle.
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands
        matches:
        - path: ToolCommand.java
          matches:
          - "public class ToolCommand extends Command {"
        - path: CommandRegistry.java
          matches:
          - "public class CommandRegistry {"
        - path: Command.java
          matches:
          - "public abstract class Command {"
        - path: AnswerCommand.java
          matches:
          - "public class AnswerCommand extends Command {"
        - path: ExitCommand.java
          matches:
          - "public class ExitCommand extends Command {"
        - path: CallCommand.java
          matches:
          - "public class CallCommand extends Command {"
        - path: ResumeCommand.java
          matches:
          - "public class ResumeCommand extends Command {"
        - path: ControlRequestCommand.java
          matches:
          - "public class ControlRequestCommand extends Command {"
        - path: CallEditCommand.java
          matches:
          - "public class CallEditCommand extends Command {"
```
/answer 78b3-8-7 allow
Control Request:
```yaml
id: 78b3-9-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
    node_type: .
  reason: Get full source of IncludeProcessor
```
/answer 78b3-9-0 allow
Control Request:
```yaml
id: 78b3-a-9
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
        nodes:
        - id: 4EqrOs|xE1LS6
          type: ImportGroup
          code: |
            import java.io.IOException;
            import java.nio.file.Files;
            import java.nio.file.Path;
            import java.nio.file.Paths;
            import java.util.ArrayList;
            import java.util.List;
            import java.util.regex.Matcher;
            import java.util.regex.Pattern;
            import java.util.stream.Collectors;
            import java.util.stream.Stream;

            import com.fasterxml.jackson.databind.node.ArrayNode;
            import com.fasterxml.jackson.databind.node.JsonNodeFactory;
            import com.fasterxml.jackson.databind.node.ObjectNode;

            import xy.ai.conscious.connector.claudecode.YamlRenderer;
        - id: IncludeProcessor
          type: ClassOrInterfaceDeclaration
          excert: "public class IncludeProcessor<M> implements ISessionIncludeProcessor<M> {"
          children:
          - id: IncludeProcessor.mNpHXF|W6hBct
            type: StatementGroup
            code: |2
                public static final Pattern INCLUDE_LINE = Pattern.compile("^(\\s*\\[include(?:\\s+\\w+)?\\]\\([^)]*\\)\\s*)+$");
                public static final Pattern INCLUDE_TAG = Pattern.compile("\\[include(?:\\s+(\\w+))?\\]\\(([^)]*)\\)");
                private final YamlRenderer yaml = new YamlRenderer();
                private final IIncludeAdapter adapter;
          - id: IncludeProcessor.IncludeProcessor
            type: ConstructorDeclaration
            code: |2
                public IncludeProcessor(IIncludeAdapter adapter) {
                  this.adapter = adapter;
                }
          - id: IncludeProcessor.include
            type: MethodDeclaration
            code: |2
                public void include(String kind, String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
                  if (kind == null) {
                    if (arg.isBlank())
                      throw new IllegalStateException("Include path must not be empty");
                    includePath(arg, chain, call, out, recursiv);
                    return;
                  }
                  switch(kind) {
                    case "contextprompt":
                      includeContextPrompt(arg, call, out);
                      return;
                    case "directory":
                      includeDirectory(arg, chain, call, out, recursiv);
                      return;
                    case "files":
                      includeFiles(arg, call, out);
                      return;
                    case "file":
                      includeFile(arg, call, out);
                      return;
                    case "search":
                      includeSearch(arg, call, out);
                      return;
                    default:
                      throw new IllegalStateException("Unknown include kind: " + kind);
                  }
                }
          - id: IncludeProcessor.processText
            type: MethodDeclaration
            code: |2
                @Override
                public void processText(String text, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
                  String[] lines = text == null ? new String[0] : text.split("\n", -1);
                  StringBuilder buffer = new StringBuilder();
                  for (String line : lines) {
                    if (INCLUDE_LINE.matcher(line).matches()) {
                      flush(buffer, call, out);
                      Matcher m = INCLUDE_TAG.matcher(line);
                      while (m.find()) include(m.group(1), m.group(2).strip(), chain, call, out, recursiv);
                      continue;
                    }
                    if (buffer.length() > 0)
                      buffer.append("\n");
                    buffer.append(line);
                  }
                  flush(buffer, call, out);
                }
          - id: IncludeProcessor.flush
            type: MethodDeclaration
            code: |2
                private void flush(StringBuilder buffer, SessionCallbacks<M> call, List<M> out) {
                  String text = buffer.toString().strip();
                  buffer.setLength(0);
                  if (!text.isEmpty())
                    out.add(call.message(Role.User, text));
                }
          - id: IncludeProcessor.includePath
            type: MethodDeclaration
            code: |2
                private void includePath(String pathText, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
                  Path path = Paths.get(pathText).toAbsolutePath().normalize();
                  if (chain.contains(path))
                    throw new IncludeCycleException(path.toString());
                  String content;
                  try {
                    content = Files.readString(path);
                  } catch (IOException e) {
                    throw new IllegalStateException("Could not read include target: " + path, e);
                  }
                  List<Path> nested = new ArrayList<>(chain);
                  nested.add(path);
                  recursiv.processText(content, nested, call, out, recursiv);
                }
          - id: IncludeProcessor.includeDirectory
            type: MethodDeclaration
            code: |2
                private void includeDirectory(String dir, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
                  if (dir.isBlank()) {
                    for (IIncludeAdapter.Entry entry : requireAdapter().selectedDirectories()) out.add(call.toolResult(new ToolResult(entry.id, entry.content)));
                    return;
                  }
                  Path directory = Paths.get(dir).toAbsolutePath().normalize();
                  List<Path> files;
                  try (Stream<Path> stream = Files.list(directory)) {
                    files = stream.filter(Files::isRegularFile).sorted().collect(Collectors.toList());
                  } catch (IOException e) {
                    throw new IllegalStateException("Could not read include directory: " + directory, e);
                  }
                  for (Path file : files) includePath(file.toString(), chain, call, out, recursiv);
                }
          - id: IncludeProcessor.includeContextPrompt
            type: MethodDeclaration
            code: |2
                private void includeContextPrompt(String dir, SessionCallbacks<M> call, List<M> out) {
                  String text = requireAdapter().contextPrompt(dir);
                  if (text != null && !text.isBlank())
                    out.add(call.message(Role.User, text.strip()));
                }
          - id: IncludeProcessor.includeFiles
            type: MethodDeclaration
            code: |2
                private void includeFiles(String arg, SessionCallbacks<M> call, List<M> out) {
                  for (IIncludeAdapter.Entry entry : requireAdapter().files(arg)) out.add(call.toolResult(new ToolResult(entry.id, entry.content)));
                }
          - id: IncludeProcessor.includeFile
            type: MethodDeclaration
            code: |2
                private void includeFile(String path, SessionCallbacks<M> call, List<M> out) {
                  String content = requireAdapter().file(path);
                  if (content != null)
                    out.add(call.toolResult(new ToolResult(path, content)));
                }
          - id: IncludeProcessor.includeSearch
            type: MethodDeclaration
            code: |2
                private void includeSearch(String arg, SessionCallbacks<M> call, List<M> out) {
                  IIncludeAdapter a = requireAdapter();
                  if ("files".equals(arg))
                    for (IIncludeAdapter.Entry entry : a.searchFiles()) out.add(call.toolResult(new ToolResult(entry.id, entry.content)));
                  else if ("matches".equals(arg)) {
                    List<IIncludeAdapter.Match> matches = a.searchMatches();
                    out.add(call.toolResult(new ToolResult("search-matches", renderMatches(matches))));
                  } else
                    throw new IllegalStateException("Unknown search include argument: " + arg);
                }
          - id: IncludeProcessor.renderMatches
            type: MethodDeclaration
            code: |2
                private String renderMatches(List<IIncludeAdapter.Match> matches) {
                  ArrayNode arr = JsonNodeFactory.instance.arrayNode();
                  for (IIncludeAdapter.Match match : matches) {
                    ObjectNode node = arr.addObject();
                    node.put("file", match.file);
                    node.put("line", match.line);
                    node.put("text", match.text);
                  }
                  return yaml.toYaml(arr);
                }
          - id: IncludeProcessor.requireAdapter
            type: MethodDeclaration
            code: |2
                private IIncludeAdapter requireAdapter() {
                  if (adapter == null)
                    throw new IllegalStateException("No IIncludeAdapter configured for this SessionProcessor");
                  return adapter;
                }
```
/answer 78b3-a-9 allow
ReasoningToken: 25
Control Request:
```yaml
id: 78b3-b-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/ISessionIncludeProcessor.java
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/Command.java
    node_type: .
  reason: Get interface and Command base source for conventions
```
/answer 78b3-b-0 allow
Control Request:
```yaml
id: 78b3-c-b
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/ISessionIncludeProcessor.java
        nodes:
        - id: vsldXD|VjXVVc
          type: ImportGroup
          code: |
            import java.nio.file.Path;
            import java.util.List;
        - id: ISessionIncludeProcessor
          type: ClassOrInterfaceDeclaration
          excert: "public interface ISessionIncludeProcessor<M> {"
          children:
          - id: ISessionIncludeProcessor.processText
            type: MethodDeclaration
            code: "\tpublic void processText(String text, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv);\n"
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/Command.java
        nodes:
        - id: Command
          type: ClassOrInterfaceDeclaration
          excert: "public abstract class Command {"
          children:
          - id: Command.ProcessorAction
            type: EnumDeclaration
            code: "\tpublic enum ProcessorAction {\n\t\tREMOVE, TRANSFORM, IGNORE_BLOCK, NONE\n\t}\n"
          - id: Command.iKqF62|lJcluI
            type: StatementGroup
            code: "\tprotected final String[] parameters;\n"
          - id: Command.Command
            type: ConstructorDeclaration
            code: "\tprotected Command(String... parameters) {\n\t\tthis.parameters = parameters;\n\t}\n"
          - id: Command.prefix
            type: MethodDeclaration
            code: "\tpublic abstract String prefix();\n"
          - id: Command.matches
            type: MethodDeclaration
            code: "\tpublic boolean matches(String line) {\n\t\treturn line != null && line.strip().startsWith(prefix());\n\t}\n"
          - id: Command.parse
            type: MethodDeclaration
            code: "\tpublic abstract Command parse(String line);\n"
          - id: Command.processorAction
            type: MethodDeclaration
            code: "\tpublic abstract ProcessorAction processorAction();\n"
          - id: Command.parameter
            type: MethodDeclaration
            code: "\tpublic String parameter(int i) {\n\t\treturn parameters.length > i ? parameters[i] : null;\n\n\t}\n"
```
/answer 78b3-c-b allow
ReasoningToken: 88
Control Request:
```yaml
id: 78b3-d-0
toolName: ast_retrieve
arguments:
  search:
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious
    pattern: ^package |(class|interface|enum|record) (IIncludeAdapter|SessionCallbacks|Role|ToolResult|IncludeCycleException)\b
    include:
    - IIncludeAdapter.java
    - SessionCallbacks.java
    - Role.java
    - ToolResult.java
    - IncludeCycleException.java
    - IncludeProcessor.java
    limit: 30
  - directory:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness
    pattern: ^import|includeProcessor|SessionProcessor\(|adapter
    include:
    - SessionProcessor.java
    limit: 40
  reason: Verify packages of types used by IncludeProcessor and SessionProcessor usage
```
/answer 78b3-d-0 allow
Control Request:
```yaml
id: 78b3-e-d
result:
  structuredContent:
    search:
      results:
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious
        matches:
        - path: harness/SessionCallbacks.java
          matches:
          - package xy.ai.conscious.harness;
          - "public interface SessionCallbacks<M> {"
        - path: harness/ToolResult.java
          matches:
          - package xy.ai.conscious.harness;
          - "public final class ToolResult {"
        - path: harness/IncludeProcessor.java
          matches:
          - package xy.ai.conscious.harness;
        - path: harness/IncludeCycleException.java
          matches:
          - package xy.ai.conscious.harness;
          - "public class IncludeCycleException extends RuntimeException {"
        - path: harness/Role.java
          matches:
          - package xy.ai.conscious.harness;
          - "public enum Role {"
        - path: harness/IIncludeAdapter.java
          matches:
          - package xy.ai.conscious.harness;
          - "public interface IIncludeAdapter {"
      - directory:
        - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness
        matches:
        - path: SessionProcessor.java
          matches:
          - import java.nio.file.Path;
          - import java.util.ArrayList;
          - import java.util.Iterator;
          - import java.util.List;
          - import java.util.Map;
          - import java.util.regex.Matcher;
          - import com.fasterxml.jackson.databind.JsonNode;
          - import com.fasterxml.jackson.databind.ObjectMapper;
          - import com.fasterxml.jackson.databind.node.ArrayNode;
          - import com.fasterxml.jackson.databind.node.JsonNodeFactory;
          - import com.fasterxml.jackson.databind.node.NullNode;
          - import com.fasterxml.jackson.databind.node.ObjectNode;
          - import com.fasterxml.jackson.databind.node.TextNode;
          - import xy.ai.conscious.commands.Command;
          - import xy.ai.conscious.commands.CommandRegistry;
          - import xy.ai.conscious.connector.claudecode.YamlRenderer;
          - import xy.ai.conscious.core.IEditorInterface;
          - import xy.ai.conscious.model.AIAnswer;
          - private final IncludeProcessor<M> includeProcessor;
          - "public SessionProcessor(IIncludeAdapter adapter) {"
          - this.includeProcessor = new IncludeProcessor<>(adapter);
          - "while (m.find()) includeProcessor.include(m.group(1), m.group(2).strip(), chain, call, out, recursiv);"
```
/answer 78b3-e-d allow
ReasoningToken: 806
Control Request:
```yaml
id: 78b3-f-0
toolName: ast_edit
arguments:
  create:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IIncludeCommand.java
    source: |
      package xy.ai.conscious.commands.include;

      import java.nio.file.Path;
      import java.util.List;

      import xy.ai.conscious.harness.ISessionIncludeProcessor;
      import xy.ai.conscious.harness.SessionCallbacks;

      public interface IIncludeCommand<M> {

        /** Kind in {@code [include kind](arg)}; {@code null} for the kind-less path include. */
        String kind();

        void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv);
      }
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/AdapterIncludeCommand.java
    source: |
      package xy.ai.conscious.commands.include;

      import xy.ai.conscious.harness.IIncludeAdapter;

      abstract class AdapterIncludeCommand<M> implements IIncludeCommand<M> {

        private final IIncludeAdapter adapter;

        protected AdapterIncludeCommand(IIncludeAdapter adapter) {
          this.adapter = adapter;
        }

        protected IIncludeAdapter requireAdapter() {
          if (adapter == null)
            throw new IllegalStateException("No IIncludeAdapter configured for this SessionProcessor");
          return adapter;
        }
      }
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/PathIncludeCommand.java
    source: |
      package xy.ai.conscious.commands.include;

      import java.io.IOException;
      import java.nio.file.Files;
      import java.nio.file.Path;
      import java.nio.file.Paths;
      import java.util.ArrayList;
      import java.util.List;

      import xy.ai.conscious.harness.IncludeCycleException;
      import xy.ai.conscious.harness.ISessionIncludeProcessor;
      import xy.ai.conscious.harness.SessionCallbacks;

      public class PathIncludeCommand<M> implements IIncludeCommand<M> {

        @Override
        public String kind() {
          return null;
        }

        @Override
        public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
          if (arg.isBlank())
            throw new IllegalStateException("Include path must not be empty");
          includePath(arg, chain, call, out, recursiv);
        }

        void includePath(String pathText, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
          Path path = Paths.get(pathText).toAbsolutePath().normalize();
          if (chain.contains(path))
            throw new IncludeCycleException(path.toString());
          String content;
          try {
            content = Files.readString(path);
          } catch (IOException e) {
            throw new IllegalStateException("Could not read include target: " + path, e);
          }
          List<Path> nested = new ArrayList<>(chain);
          nested.add(path);
          recursiv.processText(content, nested, call, out, recursiv);
        }
      }
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/DirectoryIncludeCommand.java
    source: |
      package xy.ai.conscious.commands.include;

      import java.io.IOException;
      import java.nio.file.Files;
      import java.nio.file.Path;
      import java.nio.file.Paths;
      import java.util.List;
      import java.util.stream.Collectors;
      import java.util.stream.Stream;

      import xy.ai.conscious.harness.IIncludeAdapter;
      import xy.ai.conscious.harness.ISessionIncludeProcessor;
      import xy.ai.conscious.harness.SessionCallbacks;
      import xy.ai.conscious.harness.ToolResult;

      public class DirectoryIncludeCommand<M> extends AdapterIncludeCommand<M> {

        private final PathIncludeCommand<M> pathCommand;

        public DirectoryIncludeCommand(IIncludeAdapter adapter, PathIncludeCommand<M> pathCommand) {
          super(adapter);
          this.pathCommand = pathCommand;
        }

        @Override
        public String kind() {
          return "directory";
        }

        @Override
        public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
          if (arg.isBlank()) {
            for (IIncludeAdapter.Entry entry : requireAdapter().selectedDirectories()) out.add(call.toolResult(new ToolResult(entry.id, entry.content)));
            return;
          }
          Path directory = Paths.get(arg).toAbsolutePath().normalize();
          List<Path> files;
          try (Stream<Path> stream = Files.list(directory)) {
            files = stream.filter(Files::isRegularFile).sorted().collect(Collectors.toList());
          } catch (IOException e) {
            throw new IllegalStateException("Could not read include directory: " + directory, e);
          }
          for (Path file : files) pathCommand.includePath(file.toString(), chain, call, out, recursiv);
        }
      }
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/ContextPromptIncludeCommand.java
    source: |
      package xy.ai.conscious.commands.include;

      import java.nio.file.Path;
      import java.util.List;

      import xy.ai.conscious.harness.IIncludeAdapter;
      import xy.ai.conscious.harness.ISessionIncludeProcessor;
      import xy.ai.conscious.harness.Role;
      import xy.ai.conscious.harness.SessionCallbacks;

      public class ContextPromptIncludeCommand<M> extends AdapterIncludeCommand<M> {

        public ContextPromptIncludeCommand(IIncludeAdapter adapter) {
          super(adapter);
        }

        @Override
        public String kind() {
          return "contextprompt";
        }

        @Override
        public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
          String text = requireAdapter().contextPrompt(arg);
          if (text != null && !text.isBlank())
            out.add(call.message(Role.User, text.strip()));
        }
      }
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/FilesIncludeCommand.java
    source: |
      package xy.ai.conscious.commands.include;

      import java.nio.file.Path;
      import java.util.List;

      import xy.ai.conscious.harness.IIncludeAdapter;
      import xy.ai.conscious.harness.ISessionIncludeProcessor;
      import xy.ai.conscious.harness.SessionCallbacks;
      import xy.ai.conscious.harness.ToolResult;

      public class FilesIncludeCommand<M> extends AdapterIncludeCommand<M> {

        public FilesIncludeCommand(IIncludeAdapter adapter) {
          super(adapter);
        }

        @Override
        public String kind() {
          return "files";
        }

        @Override
        public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
          for (IIncludeAdapter.Entry entry : requireAdapter().files(arg)) out.add(call.toolResult(new ToolResult(entry.id, entry.content)));
        }
      }
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/FileIncludeCommand.java
    source: |
      package xy.ai.conscious.commands.include;

      import java.nio.file.Path;
      import java.util.List;

      import xy.ai.conscious.harness.IIncludeAdapter;
      import xy.ai.conscious.harness.ISessionIncludeProcessor;
      import xy.ai.conscious.harness.SessionCallbacks;
      import xy.ai.conscious.harness.ToolResult;

      public class FileIncludeCommand<M> extends AdapterIncludeCommand<M> {

        public FileIncludeCommand(IIncludeAdapter adapter) {
          super(adapter);
        }

        @Override
        public String kind() {
          return "file";
        }

        @Override
        public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
          String content = requireAdapter().file(arg);
          if (content != null)
            out.add(call.toolResult(new ToolResult(arg, content)));
        }
      }
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/SearchIncludeCommand.java
    source: |
      package xy.ai.conscious.commands.include;

      import java.nio.file.Path;
      import java.util.List;

      import com.fasterxml.jackson.databind.node.ArrayNode;
      import com.fasterxml.jackson.databind.node.JsonNodeFactory;
      import com.fasterxml.jackson.databind.node.ObjectNode;

      import xy.ai.conscious.connector.claudecode.YamlRenderer;
      import xy.ai.conscious.harness.IIncludeAdapter;
      import xy.ai.conscious.harness.ISessionIncludeProcessor;
      import xy.ai.conscious.harness.SessionCallbacks;
      import xy.ai.conscious.harness.ToolResult;

      public class SearchIncludeCommand<M> extends AdapterIncludeCommand<M> {

        private final YamlRenderer yaml = new YamlRenderer();

        public SearchIncludeCommand(IIncludeAdapter adapter) {
          super(adapter);
        }

        @Override
        public String kind() {
          return "search";
        }

        @Override
        public void execute(String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
          IIncludeAdapter a = requireAdapter();
          if ("files".equals(arg))
            for (IIncludeAdapter.Entry entry : a.searchFiles()) out.add(call.toolResult(new ToolResult(entry.id, entry.content)));
          else if ("matches".equals(arg)) {
            List<IIncludeAdapter.Match> matches = a.searchMatches();
            out.add(call.toolResult(new ToolResult("search-matches", renderMatches(matches))));
          } else
            throw new IllegalStateException("Unknown search include argument: " + arg);
        }

        private String renderMatches(List<IIncludeAdapter.Match> matches) {
          ArrayNode arr = JsonNodeFactory.instance.arrayNode();
          for (IIncludeAdapter.Match match : matches) {
            ObjectNode node = arr.addObject();
            node.put("file", match.file);
            node.put("line", match.line);
            node.put("text", match.text);
          }
          return yaml.toYaml(arr);
        }
      }
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeCommandRegistry.java
    source: |
      package xy.ai.conscious.commands.include;

      import java.nio.file.Path;
      import java.util.HashMap;
      import java.util.List;
      import java.util.Map;

      import xy.ai.conscious.harness.IIncludeAdapter;
      import xy.ai.conscious.harness.ISessionIncludeProcessor;
      import xy.ai.conscious.harness.SessionCallbacks;

      public class IncludeCommandRegistry<M> {

        private final Map<String, IIncludeCommand<M>> commands = new HashMap<>();

        public IncludeCommandRegistry(IIncludeAdapter adapter) {
          PathIncludeCommand<M> path = new PathIncludeCommand<>();
          register(path);
          register(new DirectoryIncludeCommand<>(adapter, path));
          register(new ContextPromptIncludeCommand<>(adapter));
          register(new FilesIncludeCommand<>(adapter));
          register(new FileIncludeCommand<>(adapter));
          register(new SearchIncludeCommand<>(adapter));
        }

        public final void register(IIncludeCommand<M> command) {
          commands.put(command.kind(), command);
        }

        public void execute(String kind, String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
          IIncludeCommand<M> command = commands.get(kind);
          if (command == null)
            throw new IllegalStateException("Unknown include kind: " + kind);
          command.execute(arg, chain, call, out, recursiv);
        }
      }
  reason: "Create include command interface, base class, commands and registry in commands.include package"
```
/answer 78b3-f-0 allow
Control Request:
```yaml
id: 78b3-10-0
toolName: ast_edit
arguments:
  replace:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
    id: IncludeProcessor.mNpHXF|W6hBct
    source: |2
        public static final Pattern INCLUDE_LINE = Pattern.compile("^(\\s*\\[include(?:\\s+\\w+)?\\]\\([^)]*\\)\\s*)+$");
        public static final Pattern INCLUDE_TAG = Pattern.compile("\\[include(?:\\s+(\\w+))?\\]\\(([^)]*)\\)");
        private final IncludeCommandRegistry<M> commands;
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
    id: IncludeProcessor.IncludeProcessor
    source: |2
        public IncludeProcessor(IIncludeAdapter adapter) {
          this.commands = new IncludeCommandRegistry<>(adapter);
        }
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
    id: IncludeProcessor.include
    source: |2
        public void include(String kind, String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
          commands.execute(kind, arg, chain, call, out, recursiv);
        }
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
    id: 4EqrOs|xE1LS6
    source: |
      import java.nio.file.Path;
      import java.util.List;
      import java.util.regex.Matcher;
      import java.util.regex.Pattern;

      import xy.ai.conscious.commands.include.IncludeCommandRegistry;
  reason: Slim IncludeProcessor to text parsing and delegate to IncludeCommandRegistry
```
/answer 78b3-10-0 allow
ReasoningToken: 120
Control Request:
```yaml
id: 78b3-11-0
toolName: ast_edit
arguments:
  delete:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
    id: IncludeProcessor.includePath
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
    id: IncludeProcessor.includeDirectory
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
    id: IncludeProcessor.includeContextPrompt
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
    id: IncludeProcessor.includeFiles
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
    id: IncludeProcessor.includeFile
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
    id: IncludeProcessor.includeSearch
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
    id: IncludeProcessor.renderMatches
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
    id: IncludeProcessor.requireAdapter
  reason: Remove include logic moved into command classes
```
/answer 78b3-11-0 allow
Text:
 Ich prüfe das Ergebnis.
Control Request:
```yaml
id: 78b3-12-0
toolName: ast_retrieve
arguments:
  find:
  - paths:
    - /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
    node_type: .
  reason: Verify final IncludeProcessor
```
/answer 78b3-12-0 allow
Control Request:
```yaml
id: 78b3-13-12
result:
  structuredContent:
    find:
      results:
      - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/harness/IncludeProcessor.java
        nodes:
        - id: CJTOAQ|jn3SR2
          type: ImportGroup
          code: |
            import java.nio.file.Path;
            import java.util.List;
            import java.util.regex.Matcher;
            import java.util.regex.Pattern;
            import xy.ai.conscious.commands.include.IncludeCommandRegistry;
        - id: IncludeProcessor
          type: ClassOrInterfaceDeclaration
          excert: "public class IncludeProcessor<M> implements ISessionIncludeProcessor<M> {"
          children:
          - id: IncludeProcessor.9ULz91|4YRM35
            type: StatementGroup
            code: |2
                public static final Pattern INCLUDE_LINE = Pattern.compile("^(\\s*\\[include(?:\\s+\\w+)?\\]\\([^)]*\\)\\s*)+$");
                public static final Pattern INCLUDE_TAG = Pattern.compile("\\[include(?:\\s+(\\w+))?\\]\\(([^)]*)\\)");
                private final IncludeCommandRegistry<M> commands;
          - id: IncludeProcessor.IncludeProcessor
            type: ConstructorDeclaration
            code: |2
                public IncludeProcessor(IIncludeAdapter adapter) {
                  this.commands = new IncludeCommandRegistry<>(adapter);
                }
          - id: IncludeProcessor.include
            type: MethodDeclaration
            code: |2
                public void include(String kind, String arg, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
                  commands.execute(kind, arg, chain, call, out, recursiv);
                }
          - id: IncludeProcessor.processText
            type: MethodDeclaration
            code: |2
                @Override
                public void processText(String text, List<Path> chain, SessionCallbacks<M> call, List<M> out, ISessionIncludeProcessor<M> recursiv) {
                  String[] lines = text == null ? new String[0] : text.split("\n", -1);
                  StringBuilder buffer = new StringBuilder();
                  for (String line : lines) {
                    if (INCLUDE_LINE.matcher(line).matches()) {
                      flush(buffer, call, out);
                      Matcher m = INCLUDE_TAG.matcher(line);
                      while (m.find()) include(m.group(1), m.group(2).strip(), chain, call, out, recursiv);
                      continue;
                    }
                    if (buffer.length() > 0)
                      buffer.append("\n");
                    buffer.append(line);
                  }
                  flush(buffer, call, out);
                }
          - id: IncludeProcessor.flush
            type: MethodDeclaration
            code: |2
                private void flush(StringBuilder buffer, SessionCallbacks<M> call, List<M> out) {
                  String text = buffer.toString().strip();
                  buffer.setLength(0);
                  if (!text.isEmpty())
                    out.add(call.message(Role.User, text));
                }
```
/answer 78b3-13-12 allow
Result Stats: id=7ac8ba7a-cb8b-4d58-af16-0a517e4afb65, total: 38751, in: 16, out: 10396, reason: 0, read: 110427, write: 28339, start: 1795812158048, end: 1795812164191, time: 6079, total: 58161, io: 0, control: 64, sent: 0, recv: 158
Ich habe das Include-Handling aus `IncludeProcessor` in einzelne Include-Commands im Package `xy.ai.conscious.commands.include` ausgelagert. Kompiliert oder getestet habe ich nichts. Bitte baue das Projekt einmal.

**Neue Klassen**
- `IIncludeCommand<M>` ist das gemeinsame Interface. Es hat `kind()` und `execute(arg, chain, call, out, recursiv)`. Für `[include](pfad)` ohne Kind liefert `kind()` `null`.
- `AdapterIncludeCommand<M>` ist eine package-private Basisklasse. Sie enthält `requireAdapter()` mit derselben Fehlermeldung wie vorher.
- Die einzelnen Commands:
  - `PathIncludeCommand`: Kind `null`, mit Zyklus-Check und rekursiver Verarbeitung.
  - `DirectoryIncludeCommand`: Kind `directory`.
  - `ContextPromptIncludeCommand`: Kind `contextprompt`.
  - `FilesIncludeCommand`: Kind `files`.
  - `FileIncludeCommand`: Kind `file`.
  - `SearchIncludeCommand`: Kind `search`, inklusive YAML-Rendering der Treffer.
- `IncludeCommandRegistry<M>` registriert alle Commands und verteilt nach Kind. Ein unbekanntes Kind wirft weiterhin `Unknown include kind: …`. Neue Commands lassen sich mit `register(...)` ergänzen.

**Geändert**
- `IncludeProcessor` macht nur noch das Zeilen-Parsing (`INCLUDE_LINE`, `INCLUDE_TAG`, `processText`, `flush`) und delegiert `include(...)` an die Registry.
- Die öffentliche API von `IncludeProcessor` ist unverändert. `SessionProcessor` und `CCConnector` mussten deshalb nicht angepasst werden.

Das Verhalten sollte unverändert sein. Die Logik habe ich 1:1 übernommen.

`IncludeProcessor` liegt weiterhin in `harness`, weil `SessionProcessor` und `CCConnector` ihn dort importieren und `ISessionIncludeProcessor` ebenfalls dort liegt. Wenn auch er ins Package `include` soll, müssen diese Imports angepasst werden.
Die Parameter der IncludeCommands für Dateien oder Verzeichnisse sollen einen Pfad der Form `/home/user/ToolResult.java` verarbeiten können und die Führende und schließende "`" entfernen.
SystemInit:  id=7ac8ba7a-cb8b-4d58-af16-0a517e4afb65, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 78b3-14-0
toolName: ast_edit
arguments:
  create:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/IncludeArgs.java
    source: |
      package xy.ai.conscious.commands.include;

      final class IncludeArgs {

        private IncludeArgs() {
        }

        /** Removes surrounding whitespace and a leading and trailing backtick, e.g. {@code `/home/user/ToolResult.java`}. */
        static String path(String arg) {
          String s = arg.strip();
          if (s.startsWith("`"))
            s = s.substring(1);
          if (s.endsWith("`"))
            s = s.substring(0, s.length() - 1);
          return s.strip();
        }
      }
  reason: Add backtick-stripping helper and apply it in path-based include commands
```
/answer 78b3-14-0 allow
ReasoningToken: 407
Control Request:
```yaml
id: 78b3-15-0
toolName: ast_edit
arguments:
  edit_block:
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/PathIncludeCommand.java
    id: PathIncludeCommand.execute
    old_text: |2-
          if (arg.isBlank())
            throw new IllegalStateException("Include path must not be empty");
          includePath(arg, chain, call, out, recursiv);
    new_text: |2-
          String path = IncludeArgs.path(arg);
          if (path.isBlank())
            throw new IllegalStateException("Include path must not be empty");
          includePath(path, chain, call, out, recursiv);
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/DirectoryIncludeCommand.java
    id: DirectoryIncludeCommand.execute
    old_text: "    if (arg.isBlank()) {"
    new_text: |2-
          String dir = IncludeArgs.path(arg);
          if (dir.isBlank()) {
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/DirectoryIncludeCommand.java
    id: DirectoryIncludeCommand.execute
    old_text: Paths.get(arg).toAbsolutePath()
    new_text: Paths.get(dir).toAbsolutePath()
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/FileIncludeCommand.java
    id: FileIncludeCommand.execute
    old_text: |2-
          String content = requireAdapter().file(arg);
          if (content != null)
            out.add(call.toolResult(new ToolResult(arg, content)));
    new_text: |2-
          String path = IncludeArgs.path(arg);
          String content = requireAdapter().file(path);
          if (content != null)
            out.add(call.toolResult(new ToolResult(path, content)));
  - path: /home/user/xyan/xy.ai.workbench/connector/src/xy/ai/conscious/commands/include/FilesIncludeCommand.java
    id: FilesIncludeCommand.execute
    old_text: requireAdapter().files(arg)
    new_text: requireAdapter().files(IncludeArgs.path(arg))
  reason: "Apply path normalization in the path, directory, file and files commands"
```
/answer 78b3-15-0 allow
Result Stats: id=7ac8ba7a-cb8b-4d58-af16-0a517e4afb65, total: 44368, in: 20, out: 12673, reason: 0, read: 168067, write: 31675, start: 1795812303043, end: 1795812307619, time: 4520, total: 69324, io: 0, control: 56, sent: 0, recv: 111
Die Include-Commands für Dateien und Verzeichnisse entfernen jetzt führende und schließende Backticks. Ein Argument wie `` `/home/user/ToolResult.java` `` wird so zu `/home/user/ToolResult.java`. Kompiliert und getestet habe ich das nicht.

Dafür gibt es die neue package-private Hilfsklasse `IncludeArgs` mit der Methode `path(String)`. Sie entfernt umgebende Whitespaces und danach je einen führenden und einen schließenden Backtick. Backticks mitten im Pfad bleiben erhalten.

Die Methode wird in diesen Commands angewendet:
- **`PathIncludeCommand`:** Die Prüfung auf einen leeren Pfad läuft jetzt nach dem Entfernen der Backticks. Ein leeres Argument wie ` `` ` führt also weiterhin zu "Include path must not be empty".
- **`DirectoryIncludeCommand`:** Ein leeres Argument nach dem Entfernen, etwa ` `` `, wählt wie bisher die ausgewählten Verzeichnisse des Adapters.
- **`FileIncludeCommand`:** Der bereinigte Pfad geht an `adapter.file(...)` und wird auch als ID im `ToolResult` verwendet.
- **`FilesIncludeCommand`:** Das bereinigte Argument geht an `adapter.files(...)`.

`ContextPromptIncludeCommand` und `SearchIncludeCommand` habe ich nicht angepasst. Ihre Argumente sind keine Dateipfade.
