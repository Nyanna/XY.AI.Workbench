Ich benötige, eine englischsprachige Auflistung und kompakte Dokumentation der folgenden Aspekte:

1. Labels: "Label:" -> Segmente mit Metadaten
2. Slash-Kommandos
3. Include Anweisungen

Sourcen:
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java` (Labels)
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java` (Includes und Prefixzuordnung)
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandRegistry.java` (Commands)
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/Elements.java` (Semantik, Struktur, Prefixzuordnung)
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands` (Kommandoimplementierung)

Die Dokumentation richtet sich an Verwender nicht Entwickler.
Result Stats: id=1695247572, total: 2625, in: 2306, out: 319, reason: 53, read: 0, write: 0
Thinking:
The user wants me to analyze source files and produce English-language documentation of: Labels ("Label:" -> segments with metadata), Slash commands, Include statements. Sources given. Let me read the files.

Let me start by listing the relevant directories and reading the files.
Thinking Meta: {"type":"reasoning","id":"2a8985bc-3ab8-45bd-a550-e0ea3283d866","status":"completed","content":[{"type":"reasoning_text","text":"{{reasoning-text-0}}"}],"summary":[],"encrypted_content":"65b86de2-1a6c-4ce7-8614-40fedf5c1363-0"}
Tool:
```yaml
id: call_00_7cwTNP2uWrYjIURkkRPa6076
tool: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands
    pattern: .*
  reason: List command implementations to document slash commands.
```
/call call_00_7cwTNP2uWrYjIURkkRPa6076
Tool:
```yaml
id: call_01_SEhSaEMAD5pVHm0V0TeG6625
tool: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandRegistry.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/Elements.java
  reason: "Outline the four source files to locate label, include, and command logic."
```
/call call_01_SEhSaEMAD5pVHm0V0TeG6625
Result Stats: id=23dd7d38-c399-42a4-81cb-5b9e4c5cdba0, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_00_7cwTNP2uWrYjIURkkRPa6076",
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands",
      "entries" : [ ".:", "AnswerCommand.java", "CallCommand.java", "CallEditCommand.java", "Command.java", "CommandRegistry.java", "ControlRequestCommand.java", "ExitCommand.java", "ResumeCommand.java", "ToolCommand.java" ]
    } ]
  }
}
```
Result Stats: id=fc8e2554-e4e8-4c25-9e09-d1c54955cc9a, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_01_SEhSaEMAD5pVHm0V0TeG6625",
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java",
      "nodes" : [ {
        "id" : "eIZpM6|crr53s",
        "signature" : "package xy.ai.workbench;"
      }, {
        "id" : "YvzJYV|dN3kdP",
        "signature" : "import org.eclipse.core.runtime.Path;"
      }, {
        "id" : "Tu3Bgf|nccVUE",
        "signature" : "import org.eclipse.ui.ide.IDE;"
      }, {
        "id" : "EditorInterface",
        "signature" : "public class EditorInterface {",
        "children" : [ {
          "id" : "EditorInterface.ZPGd8R|jLT0m7",
          "signature" : "public static final String USER = \"User:\";"
        }, {
          "id" : "EditorInterface.EditorInterface",
          "signature" : "public EditorInterface(ActiveEditorListener editorListener, AdaptingConnector c…"
        }, {
          "id" : "EditorInterface.insertTag",
          "signature" : "public void insertTag(Display display, IModelRequest req, IProgressMonitor mon)…"
        }, {
          "id" : "EditorInterface.generateTag",
          "signature" : "private String generateTag(IModelRequest req) {"
        }, {
          "id" : "EditorInterface.replaceTag",
          "signature" : "public void replaceTag(Display display, AIAnswer ans, IProgressMonitor mon) {"
        } ]
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java",
      "nodes" : [ {
        "id" : "51Pa2S|dW7DdD",
        "signature" : "package xy.ai.workbench.connector.harness;"
      }, {
        "id" : "dci6aP|Ly4sqL",
        "signature" : "import com.fasterxml.jackson.databind.node.JsonNodeFactory;"
      }, {
        "id" : "fJudos|JiYyPi",
        "signature" : "/**"
      }, {
        "id" : "SessionProcessor",
        "signature" : "public class SessionProcessor {",
        "children" : [ {
          "id" : "SessionProcessor.9qssKV|rTisyg",
          "signature" : "private static final Pattern INCLUDE_LINE = Pattern.compile(\"^(\\\\s*\\\\[include(?…"
        }, {
          "id" : "SessionProcessor.setAdapter",
          "signature" : "public void setAdapter(IIncludeAdapter adapter) {"
        }, {
          "id" : "SessionProcessor.VrihTU|pKgxTe",
          "signature" : "/**"
        }, {
          "id" : "SessionProcessor.process",
          "signature" : "public <M> List<M> process(List<String> inputs, boolean enabled, SessionCallbac…"
        }, {
          "id" : "SessionProcessor.0KtdD8|9vtVBC",
          "signature" : "/**"
        }, {
          "id" : "SessionProcessor.process_1",
          "signature" : "public <M> List<M> process(String input, boolean enabled, Path rootPath, Sessio…"
        }, {
          "id" : "SessionProcessor.8yVAz2|wjBfjM",
          "signature" : "/**"
        }, {
          "id" : "SessionProcessor.Run",
          "signature" : "private final class Run<M> {",
          "children" : [ {
            "id" : "SessionProcessor.Run.dCNsrf|wxJNjN",
            "signature" : "private final SessionCallbacks<M> callbacks;"
          }, {
            "id" : "SessionProcessor.Run.Run",
            "signature" : "Run(SessionCallbacks<M> callbacks, List<M> out) {"
          }, {
            "id" : "SessionProcessor.Run.run",
            "signature" : "void run(String text, List<Path> chain) {"
          }, {
            "id" : "SessionProcessor.Run.flush",
            "signature" : "private void flush() {"
          }, {
            "id" : "SessionProcessor.Run.isMarkerLine",
            "signature" : "private boolean isMarkerLine(String line) {"
          }, {
            "id" : "SessionProcessor.Run.Oa4lRK|wV8ykx",
            "signature" : "/**"
          }, {
            "id" : "SessionProcessor.Run.consumeReasoning",
            "signature" : "private int consumeReasoning(String[] lines, int start) {"
          }, {
            "id" : "SessionProcessor.Run.VwEpxx|RGi5jR",
            "signature" : "/**"
          }, {
            "id" : "SessionProcessor.Run.substituteReasoningPlaceholders",
            "signature" : "private String substituteReasoningPlaceholders(String meta, List<String> texts)…"
          }, {
            "id" : "SessionProcessor.Run.substituteInPlace",
            "signature" : "private void substituteInPlace(JsonNode node, List<String> texts) {"
          }, {
            "id" : "SessionProcessor.Run.placeholderIndex",
            "signature" : "private int placeholderIndex(JsonNode value, List<String> texts) {"
          }, {
            "id" : "SessionProcessor.Run.consumeText",
            "signature" : "private int consumeText(String[] lines, int start) {"
          }, {
            "id" : "SessionProcessor.Run.consumeToolCall",
            "signature" : "private int consumeToolCall(String[] lines, int start) {"
          }, {
            "id" : "SessionProcessor.Run.consumeToolResult",
            "signature" : "private int consumeToolResult(String[] lines, int start) {"
          }, {
            "id" : "SessionProcessor.Run.compact",
            "signature" : "private String compact(JsonNode result) {"
          }, {
            "id" : "SessionProcessor.Run.OLksgp|aiET5e",
            "signature" : "/**"
          }, {
            "id" : "SessionProcessor.Run.fenceRange",
            "signature" : "private int[] fenceRange(String[] lines, int from) {"
          }, {
            "id" : "SessionProcessor.Run.readYaml",
            "signature" : "private JsonNode readYaml(String[] lines, int[] range) {"
          }, {
            "id" : "SessionProcessor.Run.fenceSpecifier",
            "signature" : "private String fenceSpecifier(String[] lines, int[] range) {"
          }, {
            "id" : "SessionProcessor.Run.readJson",
            "signature" : "private JsonNode readJson(String[] lines, int[] range) {"
          }, {
            "id" : "SessionProcessor.Run.include",
            "signature" : "private void include(String kind, String arg, List<Path> chain) {"
          }, {
            "id" : "SessionProcessor.Run.gIPyik|Yy3tak",
            "signature" : "/**"
          }, {
            "id" : "SessionProcessor.Run.includePath",
            "signature" : "private void includePath(String pathText, List<Path> chain) {"
          }, {
            "id" : "SessionProcessor.Run.includeContextPrompt",
            "signature" : "private void includeContextPrompt(String dir) {"
          }, {
            "id" : "SessionProcessor.Run.includeFiles",
            "signature" : "private void includeFiles(String arg) {"
          }, {
            "id" : "SessionProcessor.Run.includeFile",
            "signature" : "private void includeFile(String path) {"
          }, {
            "id" : "SessionProcessor.Run.includeSearch",
            "signature" : "private void includeSearch(String arg) {"
          }, {
            "id" : "SessionProcessor.Run.renderMatches",
            "signature" : "private String renderMatches(List<IIncludeAdapter.Match> matches) {"
          }, {
            "id" : "SessionProcessor.Run.requireAdapter",
            "signature" : "private IIncludeAdapter requireAdapter() {"
          } ]
        } ]
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandRegistry.java",
      "nodes" : [ {
        "id" : "b6Xn8Z|PmHdMh",
        "signature" : "package xy.ai.workbench.commands;"
      }, {
        "id" : "CommandRegistry",
        "signature" : "public class CommandRegistry {",
        "children" : [ {
          "id" : "CommandRegistry.X4aQ5i|crcu8h",
          "signature" : "private static final List<Command> REGISTRY = List.of( //"
        }, {
          "id" : "CommandRegistry.detect",
          "signature" : "public static Command detect(String line) {"
        } ]
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/Elements.java",
      "nodes" : [ {
        "id" : "xDWfsD|a4ljj9",
        "signature" : "package xy.ai.workbench.editor.mdast.nodes;"
      }, {
        "id" : "PIRoA7|mseedO",
        "signature" : "/*"
      }, {
        "id" : "Elements",
        "signature" : "public class Elements {",
        "children" : [ {
          "id" : "Elements.Y9EXjM|HiUGIG",
          "signature" : "public static final AbstractNode[] NONE = new AbstractNode[0];"
        }, {
          "id" : "Elements.Basics",
          "signature" : "public static class Basics {"
        }, {
          "id" : "Elements.Headings",
          "signature" : "public static class Headings {"
        }, {
          "id" : "Elements.Agent",
          "signature" : "public static class Agent {"
        }, {
          "id" : "Elements.Tools",
          "signature" : "public static class Tools {"
        }, {
          "id" : "Elements.Page",
          "signature" : "public static class Page {"
        }, {
          "id" : "Elements.Chat",
          "signature" : "public static class Chat {"
        }, {
          "id" : "Elements.Roots",
          "signature" : "public static class Roots {"
        }, {
          "id" : "Elements.xGT0VH|2VA8hK",
          "signature" : "public static final Root ROOT = Roots.ROOT;"
        }, {
          "id" : "Elements.of",
          "signature" : "private static AbstractNode[] of(AbstractNode... nodes) {"
        }, {
          "id" : "Elements.concat",
          "signature" : "private static AbstractNode[] concat(AbstractNode[]... s) {"
        } ]
      } ]
    } ]
  }
}
```
Result Stats: id=1509216393, total: 5247, in: 1953, out: 734, reason: 22, read: 2560, write: 0
Thinking:
I need to read the actual source code of these files to document behavior. Let me read the key nodes.
Thinking Meta: {"type":"reasoning","id":"0982821f-fd8f-4f96-935e-f0117680981b","status":"completed","content":[{"type":"reasoning_text","text":"{{reasoning-text-0}}"}],"summary":[],"encrypted_content":"a5aa698f-63a5-4e6e-9d61-09579015112d-0"}
Tool:
```yaml
id: call_00_aWuY2yH34tWlN2q4CJDa4217
tool: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
    ids:
    - EditorInterface.ZPGd8R|jLT0m7
    - EditorInterface.insertTag
    - EditorInterface.generateTag
    - EditorInterface.replaceTag
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java
    ids:
    - SessionProcessor.9qssKV|rTisyg
    - SessionProcessor.Run.run
    - SessionProcessor.Run.include
    - SessionProcessor.Run.includePath
    - SessionProcessor.Run.includeContextPrompt
    - SessionProcessor.Run.includeFiles
    - SessionProcessor.Run.includeFile
    - SessionProcessor.Run.includeSearch
    - SessionProcessor.Run.renderMatches
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandRegistry.java
    ids:
    - CommandRegistry.X4aQ5i|crcu8h
    - CommandRegistry.detect
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/Elements.java
    ids:
    - Elements.Basics
    - Elements.Headings
    - Elements.Agent
    - Elements.Tools
    - Elements.Page
    - Elements.Chat
    - Elements.Roots
    - Elements.ROOT
  reason: "Read source of label generation, include processing, command registry and element definitions."
```
/call call_00_aWuY2yH34tWlN2q4CJDa4217
Tool:
```yaml
id: call_01_rPyaCOZibp3pUTQSAxco5727
tool: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallCommand.java
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallEditCommand.java
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/Command.java
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ControlRequestCommand.java
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ExitCommand.java
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ResumeCommand.java
    ids: []
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java
    ids: []
  reason: Read all command implementations to document each slash command's behavior.
```
/call call_01_rPyaCOZibp3pUTQSAxco5727
Result Stats: id=9454e36c-4910-4430-9673-881a0ca8293d, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_00_aWuY2yH34tWlN2q4CJDa4217",
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java",
      "nodes" : [ {
        "id" : "EditorInterface.ZPGd8R|jLT0m7",
        "type" : "statements",
        "code" : "public static final String USER = \"User:\";\n\tpublic static final String AGENT = \"Agent:\";\n\n\tpublic static final String TEXT = \"Text:\";\n\tpublic static final String THINKING = \"Thinking:\";\n\tpublic static final String THINKING_META = \"Thinking Meta:\";\n\tpublic static final String TOOLUSE = \"Tool:\";\n\tpublic static final String TOOLRESULT = \"ToolResult:\";\n\tpublic static final String CONTROL_REQUEST = \"Control Request:\";\n\n\tprivate final ActiveEditorListener editorListener;\n\tprivate final AdaptingConnector connector;"
      }, {
        "id" : "EditorInterface.insertTag",
        "type" : "method_declaration",
        "code" : "public void insertTag(Display display, IModelRequest req, IProgressMonitor mon) {\n\t\tdisplay.syncExec(() -> {\n\t\t\tITextEditor textEditor = editorListener.getLastTextEditor();\n\t\t\tOutputMode outputMode = req.getPrompt().config.outputMode;\n\t\t\tif (OutputMode.New_File.equals(outputMode)) {\n\n\t\t\t\tIEditorInput editorInput = textEditor.getEditorInput();\n\t\t\t\tIFile currentFile;\n\t\t\t\tif (editorInput instanceof IFileEditorInput)\n\t\t\t\t\tcurrentFile = ((IFileEditorInput) editorInput).getFile();\n\t\t\t\telse if (editorInput instanceof IURIEditorInput) {\n\t\t\t\t\tURI uri = ((IURIEditorInput) editorInput).getURI();\n\t\t\t\t\tString fileName = new Path(uri.getPath()).lastSegment();\n\t\t\t\t\tcurrentFile = ResourcesPlugin.getWorkspace().getRoot().getProject(\"ExternalFiles\")\n\t\t\t\t\t\t\t.getFile(fileName);\n\n\t\t\t\t\tif (!currentFile.exists())\n\t\t\t\t\t\ttry {\n\t\t\t\t\t\t\tcurrentFile.createLink(uri, IResource.ALLOW_MISSING_LOCAL, mon);\n\t\t\t\t\t\t} catch (CoreException e) {\n\t\t\t\t\t\t\tthrow new IllegalStateException(\"Could not link external file\", e);\n\t\t\t\t\t\t}\n\t\t\t\t} else\n\t\t\t\t\tthrow new IllegalArgumentException(\"Editor type not supported for new file output mode\");\n\n\t\t\t\tIContainer parent = currentFile.getParent();\n\n\t\t\t\tString timestamp = LocalDateTime.now().format(DateTimeFormatter.ofPattern(\"yyyyMMdd.HHmmss\"));\n\t\t\t\tIFile newFile = parent.getFile(new Path(timestamp + \".md\"));\n\t\t\t\tString tag = generateTag(req);\n\t\t\t\ttry {\n\t\t\t\t\tInputStream source = new ByteArrayInputStream(tag.getBytes(\"UTF-8\"));\n\n\t\t\t\t\tif (!newFile.exists()) {\n\t\t\t\t\t\tnewFile.create(source, true, null);\n\t\t\t\t\t} else {\n\t\t\t\t\t\tnewFile.setContents(source, true, true, null);\n\t\t\t\t\t}\n\t\t\t\t\tnewFile.touch(null);\n\n\t\t\t\t\tIWorkbenchPage page = PlatformUI.getWorkbench().getActiveWorkbenchWindow().getActivePage();\n\t\t\t\t\tIDE.openEditor(page, newFile);\n\t\t\t\t} catch (PartInitException e) {\n\t\t\t\t\tLOG.info(\"Error opening new editor file\", e);\n\t\t\t\t} catch (CoreException e) {\n\t\t\t\t\tLOG.info(\"Error writting file\", e);\n\t\t\t\t} catch (UnsupportedEncodingException e) {\n\t\t\t\t\tLOG.info(\"Error unsupported encoding\", e);\n\t\t\t\t}\n\n\t\t\t} else {\n\n\t\t\t\tIDocument doc = textEditor.getDocumentProvider().getDocument(textEditor.getEditorInput());\n\t\t\t\tISelection selection = textEditor.getSelectionProvider().getSelection();\n\t\t\t\tITextSelection tsel = selection instanceof ITextSelection ? (ITextSelection) selection : null;\n\n\t\t\t\ttry {\n\t\t\t\t\tString tag = generateTag(req);\n\t\t\t\t\tswitch (outputMode) {\n\t\t\t\t\tcase Chat:\n\t\t\t\t\t\tString replace = String.format(\"\\n%s\\n%s\\n%s\\n\", AGENT, tag, USER);\n\t\t\t\t\t\tdoc.replace(doc.getLength(), 0, replace);\n\t\t\t\t\t\tdisplay.asyncExec(() -> textEditor.selectAndReveal(doc.getLength(), 0));\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase Append:\n\t\t\t\t\t\tdoc.replace(doc.getLength(), 0, \"\\n\" + tag);\n\t\t\t\t\t\tdisplay.asyncExec(() -> textEditor.selectAndReveal(doc.getLength(), 0));\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase Replace:\n\t\t\t\t\t\tif (tsel != null)\n\t\t\t\t\t\t\tdoc.replace(tsel.getOffset(), tsel.getLength(), tag);\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase Cursor:\n\t\t\t\t\t\tif (tsel != null)\n\t\t\t\t\t\t\tdoc.replace(tsel.getOffset(), 0, tag);\n\t\t\t\t\t\tbreak;\n\t\t\t\t\tcase New_File:\n\t\t\t\t\t\tthrow new UnsupportedOperationException();\n\t\t\t\t\t}\n\t\t\t\t\ttextEditor.doSave(mon);\n\t\t\t\t} catch (BadLocationException e) {\n\t\t\t\t\tLOG.info(\"Error adding text\", e);\n\t\t\t\t}\n\t\t\t}\n\t\t});\n\t}"
      }, {
        "id" : "EditorInterface.generateTag",
        "type" : "method_declaration",
        "code" : "private String generateTag(IModelRequest req) {\n\t\tKeyPattern pattern = connector.getConnector(req).getSupportedKeyPattern();\n\t\treturn MarkerRessourceScanner.getPromptTag(pattern.name(), req.getID());\n\t}"
      }, {
        "id" : "EditorInterface.replaceTag",
        "type" : "method_declaration",
        "code" : "public void replaceTag(Display display, AIAnswer ans, IProgressMonitor mon) {\n\t\tITextEditor hint = ans.prompt != null ? ans.prompt.arg.getEditor() : null;\n\t\tOutputMode mode = ans.prompt != null ? ans.prompt.config.outputMode : null;\n\t\tif (OutputMode.Chat.equals(mode) || OutputMode.Append.equals(mode))\n\t\t\tans.answer = ans.print() + \"\\n\" + ans.answer;\n\t\tif (!Activator.getDefault().markerScanner.findAndReplaceMarkers(ans, hint))\n\t\t\tLOG.info(\"Error: wasn't able to replace prompt marker with answer:\\n\" + ans.answer);\n\t}"
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java",
      "nodes" : [ {
        "id" : "SessionProcessor.9qssKV|rTisyg",
        "type" : "statements",
        "code" : "private static final Pattern INCLUDE_LINE = Pattern.compile(\"^(\\\\s*\\\\[include(?:\\\\s+\\\\w+)?\\\\]\\\\([^)]+\\\\)\\\\s*)+$\");\n\tprivate static final Pattern INCLUDE_TAG = Pattern.compile(\"\\\\[include(?:\\\\s+(\\\\w+))?\\\\]\\\\(([^)]+)\\\\)\");\n\n\tprivate static final ObjectMapper JSON = new ObjectMapper();\n\tprivate final YamlRenderer yaml = new YamlRenderer();\n\tprivate IIncludeAdapter adapter;"
      }, {
        "id" : "SessionProcessor.Run.run",
        "type" : "method_declaration",
        "code" : "void run(String text, List<Path> chain) {\n\t\t\tString[] lines = text == null ? new String[0] : text.split(\"\\n\", -1);\n\t\t\tint i = 0;\n\t\t\twhile (i < lines.length) {\n\t\t\t\tString line = lines[i];\n\t\t\t\tString stripped = line.strip();\n\n\t\t\t\tCommand command = CommandRegistry.detect(line);\n\t\t\t\tif (command != null) {\n\t\t\t\t\tswitch (command.processorAction()) {\n\t\t\t\t\tcase IGNORE_BLOCK: {\n\t\t\t\t\t\tflush();\n\t\t\t\t\t\tint[] range = fenceRange(lines, i + 1);\n\t\t\t\t\t\ti = range != null ? range[1] + 1 : i + 1;\n\t\t\t\t\t\tcontinue;\n\t\t\t\t\t}\n\t\t\t\t\tcase TRANSFORM: {\n\t\t\t\t\t\tflush();\n\t\t\t\t\t\tString rest = command.parameter(2);\n\t\t\t\t\t\tif (rest != null && !rest.isBlank()) {\n\t\t\t\t\t\t\tif (buffer.length() > 0)\n\t\t\t\t\t\t\t\tbuffer.append(\"\\n\");\n\t\t\t\t\t\t\tbuffer.append(rest);\n\t\t\t\t\t\t}\n\t\t\t\t\t\ti++;\n\t\t\t\t\t\tcontinue;\n\t\t\t\t\t}\n\t\t\t\t\tcase REMOVE: {\n\t\t\t\t\t\tflush();\n\t\t\t\t\t\ti++;\n\t\t\t\t\t\tcontinue;\n\t\t\t\t\t}\n\t\t\t\t\tdefault:\n\t\t\t\t\t\tbreak;\n\t\t\t\t\t}\n\t\t\t\t}\n\n\t\t\t\tif (INCLUDE_LINE.matcher(line).matches()) {\n\t\t\t\t\tflush();\n\t\t\t\t\tMatcher m = INCLUDE_TAG.matcher(line);\n\t\t\t\t\twhile (m.find())\n\t\t\t\t\t\tinclude(m.group(1), m.group(2).strip(), chain);\n\t\t\t\t\ti++;\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.USER)) {\n\t\t\t\t\tflush();\n\t\t\t\t\trole = Role.User;\n\t\t\t\t\ti++;\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.AGENT)) {\n\t\t\t\t\tflush();\n\t\t\t\t\trole = Role.Agent;\n\t\t\t\t\ti++;\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.THINKING)) {\n\t\t\t\t\tflush();\n\t\t\t\t\ti = consumeReasoning(lines, i + 1);\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.TEXT)) {\n\t\t\t\t\tflush();\n\t\t\t\t\ti = consumeText(lines, i + 1);\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.TOOLUSE)) {\n\t\t\t\t\tflush();\n\t\t\t\t\ti = consumeToolCall(lines, i + 1);\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.equals(EditorInterface.TOOLRESULT)) {\n\t\t\t\t\tflush();\n\t\t\t\t\ti = consumeToolResult(lines, i + 1);\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tif (stripped.startsWith(AIAnswer.RESULT)) {\n\t\t\t\t\tflush();\n\t\t\t\t\ti++;\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\n\t\t\t\tif (buffer.length() > 0)\n\t\t\t\t\tbuffer.append(\"\\n\");\n\t\t\t\tbuffer.append(line);\n\t\t\t\ti++;\n\t\t\t}\n\t\t\tflush();\n\t\t}"
      }, {
        "id" : "SessionProcessor.Run.include",
        "type" : "method_declaration",
        "code" : "private void include(String kind, String arg, List<Path> chain) {\n\t\t\tif (kind == null) {\n\t\t\t\tincludePath(arg, chain);\n\t\t\t\treturn;\n\t\t\t}\n\t\t\tswitch (kind) {\n\t\t\tcase \"contextprompt\":\n\t\t\t\tincludeContextPrompt(arg);\n\t\t\t\treturn;\n\t\t\tcase \"files\":\n\t\t\t\tincludeFiles(arg);\n\t\t\t\treturn;\n\t\t\tcase \"file\":\n\t\t\t\tincludeFile(arg);\n\t\t\t\treturn;\n\t\t\tcase \"search\":\n\t\t\t\tincludeSearch(arg);\n\t\t\t\treturn;\n\t\t\tdefault:\n\t\t\t\tthrow new IllegalStateException(\"Unknown include kind: \" + kind);\n\t\t\t}\n\t\t}"
      }, {
        "id" : "SessionProcessor.Run.includePath",
        "type" : "method_declaration",
        "code" : "private void includePath(String pathText, List<Path> chain) {\n\t\t\tPath path = Paths.get(pathText).toAbsolutePath().normalize();\n\t\t\tif (chain.contains(path))\n\t\t\t\tthrow new IncludeCycleException(path.toString());\n\t\t\tString content;\n\t\t\ttry {\n\t\t\t\tcontent = Files.readString(path);\n\t\t\t} catch (IOException e) {\n\t\t\t\tthrow new IllegalStateException(\"Could not read include target: \" + path, e);\n\t\t\t}\n\t\t\tList<Path> nested = new ArrayList<>(chain);\n\t\t\tnested.add(path);\n\t\t\tnew Run<>(callbacks, out).run(content, nested);\n\t\t}"
      }, {
        "id" : "SessionProcessor.Run.includeContextPrompt",
        "type" : "method_declaration",
        "code" : "private void includeContextPrompt(String dir) {\n\t\t\tString text = requireAdapter().contextPrompt(dir);\n\t\t\tif (text != null && !text.isBlank())\n\t\t\t\tout.add(callbacks.message(Role.User, text.strip()));\n\t\t}"
      }, {
        "id" : "SessionProcessor.Run.includeFiles",
        "type" : "method_declaration",
        "code" : "private void includeFiles(String arg) {\n\t\t\tfor (IIncludeAdapter.Entry entry : requireAdapter().files(arg))\n\t\t\t\tout.add(callbacks.toolResult(new ToolResult(entry.id, entry.content)));\n\t\t}"
      }, {
        "id" : "SessionProcessor.Run.includeFile",
        "type" : "method_declaration",
        "code" : "private void includeFile(String path) {\n\t\t\tString content = requireAdapter().file(path);\n\t\t\tif (content != null)\n\t\t\t\tout.add(callbacks.toolResult(new ToolResult(path, content)));\n\t\t}"
      }, {
        "id" : "SessionProcessor.Run.includeSearch",
        "type" : "method_declaration",
        "code" : "private void includeSearch(String arg) {\n\t\t\tIIncludeAdapter a = requireAdapter();\n\t\t\tif (\"files\".equals(arg)) {\n\t\t\t\tfor (IIncludeAdapter.Entry entry : a.searchFiles())\n\t\t\t\t\tout.add(callbacks.toolResult(new ToolResult(entry.id, entry.content)));\n\t\t\t} else if (\"matches\".equals(arg)) {\n\t\t\t\tList<IIncludeAdapter.Match> matches = a.searchMatches();\n\t\t\t\tout.add(callbacks.toolResult(new ToolResult(\"search-matches\", renderMatches(matches))));\n\t\t\t} else\n\t\t\t\tthrow new IllegalStateException(\"Unknown search include argument: \" + arg);\n\t\t}"
      }, {
        "id" : "SessionProcessor.Run.renderMatches",
        "type" : "method_declaration",
        "code" : "private String renderMatches(List<IIncludeAdapter.Match> matches) {\n\t\t\tArrayNode arr = JsonNodeFactory.instance.arrayNode();\n\t\t\tfor (IIncludeAdapter.Match match : matches) {\n\t\t\t\tcom.fasterxml.jackson.databind.node.ObjectNode node = arr.addObject();\n\t\t\t\tnode.put(\"file\", match.file);\n\t\t\t\tnode.put(\"line\", match.line);\n\t\t\t\tnode.put(\"text\", match.text);\n\t\t\t}\n\t\t\treturn yaml.toYaml(arr);\n\t\t}"
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CommandRegistry.java",
      "nodes" : [ {
        "id" : "CommandRegistry.X4aQ5i|crcu8h",
        "type" : "statements",
        "code" : "private static final List<Command> REGISTRY = List.of( //\n\t\t\tnew AnswerCommand(), //\n\t\t\tnew ControlRequestCommand(), //\n\t\t\tnew ResumeCommand(), //\n\t\t\tnew ExitCommand(), //\n\t\t\tnew ToolCommand(), //\n\t\t\tnew CallCommand(), //\n\t\t\tnew CallEditCommand());"
      }, {
        "id" : "CommandRegistry.detect",
        "type" : "method_declaration",
        "code" : "public static Command detect(String line) {\n\t\tfor (Command prototype : REGISTRY)\n\t\t\tif (prototype.matches(line))\n\t\t\t\treturn prototype.parse(line);\n\t\treturn null;\n\t}"
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/nodes/Elements.java",
      "nodes" : [ {
        "id" : "Elements.Basics",
        "type" : "class_declaration",
        "code" : "public static class Basics {\n\t\tpublic static final ScriptBlock SCRIPTBLOCK = new ScriptBlock();\n\t\tpublic static final LineCommentBlock LINE_COMMENT = new LineCommentBlock(AbstractRule.LINE_COMMENT);\n\t\tpublic static final Paragraph PARAGRAPH = new Paragraph(NONE); // replaced later\n\t\tpublic static final AbstractNode[] ALL = of(SCRIPTBLOCK, LINE_COMMENT, PARAGRAPH);\n\t}"
      }, {
        "id" : "Elements.Headings",
        "type" : "class_declaration",
        "code" : "public static class Headings {\n\t\tpublic static final HeadingSection[] HEADINGS = new HeadingSection[HeadingSection.MAX_ORDER];\n\t\tstatic {\n\t\t\tfor (int i = 0; i < HEADINGS.length; i++)\n\t\t\t\tHEADINGS[i] = new HeadingSection(HeadingSection.MAX_ORDER - i, NONE, HEADINGS); // None will be replaced\n\n\t\t\tfor (int i = 0; i < HEADINGS.length; i++) {\n\t\t\t\tAbstractNode[] childNodes = new AbstractNode[i];\n\t\t\t\tfor (int j = 0; j < i; j++)\n\t\t\t\t\tchildNodes[j] = HEADINGS[j];\n\n\t\t\t\tHEADINGS[i].childNodes = concat( //\n\t\t\t\t\t\tchildNodes, //\n\t\t\t\t\t\tof( //\n\t\t\t\t\t\t\t\tBasics.SCRIPTBLOCK, //\n\t\t\t\t\t\t\t\tBasics.LINE_COMMENT, //\n\t\t\t\t\t\t\t\tBasics.PARAGRAPH //\n\t\t\t\t\t\t));\n\t\t\t}\n\t\t}\n\t}"
      }, {
        "id" : "Elements.Agent",
        "type" : "class_declaration",
        "code" : "public static class Agent {\n\t\tpublic static final PrefixBlock THINKING_META = new PrefixBlock(EditorInterface.THINKING_META, false);\n\t\tpublic static final LineSection THINKING = new LineSection(EditorInterface.THINKING, false,\n\t\t\t\tof(Basics.PARAGRAPH), of(THINKING_META));\n\t\tpublic static final LineSection TEXT = new LineSection(EditorInterface.TEXT, false, of(Basics.PARAGRAPH),\n\t\t\t\tof(NONE)); // replaced\n\t\tpublic static final PrefixBlock TOOLUSE = new PrefixBlock(EditorInterface.TOOLUSE, false);\n\t\tpublic static final PrefixBlock TOOLRESULT = new PrefixBlock(EditorInterface.TOOLRESULT, false);\n\t\tpublic static final PrefixBlock REASONING_TOKEN = new PrefixBlock(ProtocolParser.REASONING_TOKEN, false);\n\t\tpublic static final PrefixBlock TOKEN_STATS = new PrefixBlock(ProtocolParser.TOKEN_STATS, false);\n\t\tpublic static final PrefixBlock SYSTEM_INIT = new PrefixBlock(ProtocolParser.SYSTEM_INIT, false);\n\t\tpublic static final PrefixBlock RESULT = new PrefixBlock(AIAnswer.RESULT, true);\n\t\tpublic static final AbstractNode[] ALL = of(THINKING_META, THINKING, TEXT, TOOLUSE, TOOLRESULT, REASONING_TOKEN,\n\t\t\t\tTOKEN_STATS, SYSTEM_INIT, RESULT);\n\t}"
      }, {
        "id" : "Elements.Tools",
        "type" : "class_declaration",
        "code" : "public static class Tools {\n\t\tpublic static final PrefixBlock ANSWER = new PrefixBlock(AnswerCommand.CMD_ANSWER, true);\n\t\tpublic static final LineSection CONTROL_REQUEST = new LineSection(EditorInterface.CONTROL_REQUEST, false, of(//\n\t\t\t\tANSWER, //\n\t\t\t\tBasics.SCRIPTBLOCK), //\n\t\t\t\tNONE// replaced later\n\t\t);\n\t\tpublic static final AbstractNode[] ALL = of(CONTROL_REQUEST);\n\t}"
      }, {
        "id" : "Elements.Page",
        "type" : "class_declaration",
        "code" : "public static class Page {\n\t\tprivate static final AbstractNode[] PAGE_ELEMENTS = concat( //\n\t\t\t\tHeadings.HEADINGS, //\n\t\t\t\tof(//\n\t\t\t\t\t\tBasics.PARAGRAPH //\n\t\t\t\t));\n\t\tpublic static final PageSection PAGE = new PageSection(PAGE_ELEMENTS);\n\t\tstatic {\n\t\t\tfor (int i = 0; i < Headings.HEADINGS.length; i++)\n\t\t\t\tHeadings.HEADINGS[i].terminals = concat( //\n\t\t\t\t\t\tof(PAGE), //\n\t\t\t\t\t\tTools.ALL, //\n\t\t\t\t\t\tAgent.ALL);\n\t\t}\n\t}"
      }, {
        "id" : "Elements.Chat",
        "type" : "class_declaration",
        "code" : "public static class Chat {\n\t\tprivate static final AbstractNode[] USER_ELEMENTS = concat( //\n\t\t\t\tHeadings.HEADINGS, //\n\t\t\t\tof( //\n\t\t\t\t\t\tBasics.SCRIPTBLOCK, //\n\t\t\t\t\t\tBasics.LINE_COMMENT, //\n\t\t\t\t\t\tPage.PAGE, //\n\t\t\t\t\t\tBasics.PARAGRAPH //\n\t\t\t\t));\n\n\t\tpublic static final LineSection USER = new LineSection(EditorInterface.USER, true, USER_ELEMENTS, //\n\t\t\t\tNONE); // later replaced\n\n\t\tprivate static final AbstractNode[] AGENT_ONLY = of( //\n\t\t\t\tTools.CONTROL_REQUEST, //\n\t\t\t\tAgent.THINKING_META, //\n\t\t\t\tAgent.THINKING, //\n\t\t\t\tAgent.TEXT, //\n\t\t\t\tAgent.TOOLUSE, //\n\t\t\t\tAgent.TOOLRESULT, //\n\t\t\t\tAgent.REASONING_TOKEN, //\n\t\t\t\tAgent.TOKEN_STATS, //\n\t\t\t\tAgent.SYSTEM_INIT, //\n\t\t\t\tAgent.RESULT //\n\t\t);\n\t\tprivate static final AbstractNode[] AGENT_ELEMENTS = concat( //\n\t\t\t\tAGENT_ONLY, //\n\t\t\t\tUSER_ELEMENTS //\n\t\t);\n\t\tpublic static final LineSection AGENT = new LineSection(EditorInterface.AGENT, false, AGENT_ELEMENTS, of(USER));\n\t\tpublic static final AbstractNode[] ALL = of(USER, AGENT);\n\n\t\tstatic {\n\t\t\tUSER.terminalNodes = of(AGENT);\n\t\t\tAgent.TEXT.terminalNodes = AGENT_ONLY;\n\t\t\tTools.CONTROL_REQUEST.terminalNodes = concat(//\n\t\t\t\t\tof(USER, AGENT, //\n\t\t\t\t\t\t\tTools.CONTROL_REQUEST //\n\t\t\t\t\t), //\n\t\t\t\t\tHeadings.HEADINGS, //\n\t\t\t\t\tof(Agent.THINKING, //\n\t\t\t\t\t\t\tAgent.TEXT, //\n\t\t\t\t\t\t\tAgent.TOOLUSE, //\n\t\t\t\t\t\t\tAgent.TOOLRESULT, //\n\t\t\t\t\t\t\tAgent.REASONING_TOKEN, //\n\t\t\t\t\t\t\tAgent.TOKEN_STATS, //\n\t\t\t\t\t\t\tAgent.SYSTEM_INIT, //\n\t\t\t\t\t\t\tAgent.RESULT, //\n\t\t\t\t\t\t\tBasics.PARAGRAPH //\n\t\t\t\t\t));\n\t\t}\n\t}"
      }, {
        "id" : "Elements.Roots",
        "type" : "class_declaration",
        "code" : "public static class Roots {\n\t\tprivate static final AbstractNode[] ROOT_ELEMENTS = concat( //\n\t\t\t\tof(Page.PAGE), //\n\t\t\t\tHeadings.HEADINGS, //\n\t\t\t\tChat.ALL, //\n\t\t\t\tAgent.ALL, //\n\t\t\t\tTools.ALL, //\n\t\t\t\tBasics.ALL //\n\t\t);\n\n\t\tpublic static final Root ROOT = new Root(ROOT_ELEMENTS);\n\n\t\tstatic {\n\t\t\t// all execpt itself\n\t\t\tBasics.PARAGRAPH.terminals = Stream.of(ROOT_ELEMENTS).filter(e -> e != Basics.PARAGRAPH)\n\t\t\t\t\t.toArray(AbstractNode[]::new);\n\t\t}\n\n\t}"
      }, {
        "id" : "Elements",
        "type" : "class_declaration",
        "signature" : "public class Elements {",
        "children" : [ {
          "id" : "Elements.Y9EXjM|HiUGIG",
          "type" : "statements",
          "code" : "public static final AbstractNode[] NONE = new AbstractNode[0];"
        }, {
          "id" : "Elements.Basics",
          "type" : "class_declaration",
          "code" : "public static class Basics {\n\t\tpublic static final ScriptBlock SCRIPTBLOCK = new ScriptBlock();\n\t\tpublic static final LineCommentBlock LINE_COMMENT = new LineCommentBlock(AbstractRule.LINE_COMMENT);\n\t\tpublic static final Paragraph PARAGRAPH = new Paragraph(NONE); // replaced later\n\t\tpublic static final AbstractNode[] ALL = of(SCRIPTBLOCK, LINE_COMMENT, PARAGRAPH);\n\t}"
        }, {
          "id" : "Elements.Headings",
          "type" : "class_declaration",
          "code" : "public static class Headings {\n\t\tpublic static final HeadingSection[] HEADINGS = new HeadingSection[HeadingSection.MAX_ORDER];\n\t\tstatic {\n\t\t\tfor (int i = 0; i < HEADINGS.length; i++)\n\t\t\t\tHEADINGS[i] = new HeadingSection(HeadingSection.MAX_ORDER - i, NONE, HEADINGS); // None will be replaced\n\n\t\t\tfor (int i = 0; i < HEADINGS.length; i++) {\n\t\t\t\tAbstractNode[] childNodes = new AbstractNode[i];\n\t\t\t\tfor (int j = 0; j < i; j++)\n\t\t\t\t\tchildNodes[j] = HEADINGS[j];\n\n\t\t\t\tHEADINGS[i].childNodes = concat( //\n\t\t\t\t\t\tchildNodes, //\n\t\t\t\t\t\tof( //\n\t\t\t\t\t\t\t\tBasics.SCRIPTBLOCK, //\n\t\t\t\t\t\t\t\tBasics.LINE_COMMENT, //\n\t\t\t\t\t\t\t\tBasics.PARAGRAPH //\n\t\t\t\t\t\t));\n\t\t\t}\n\t\t}\n\t}"
        }, {
          "id" : "Elements.Agent",
          "type" : "class_declaration",
          "code" : "public static class Agent {\n\t\tpublic static final PrefixBlock THINKING_META = new PrefixBlock(EditorInterface.THINKING_META, false);\n\t\tpublic static final LineSection THINKING = new LineSection(EditorInterface.THINKING, false,\n\t\t\t\tof(Basics.PARAGRAPH), of(THINKING_META));\n\t\tpublic static final LineSection TEXT = new LineSection(EditorInterface.TEXT, false, of(Basics.PARAGRAPH),\n\t\t\t\tof(NONE)); // replaced\n\t\tpublic static final PrefixBlock TOOLUSE = new PrefixBlock(EditorInterface.TOOLUSE, false);\n\t\tpublic static final PrefixBlock TOOLRESULT = new PrefixBlock(EditorInterface.TOOLRESULT, false);\n\t\tpublic static final PrefixBlock REASONING_TOKEN = new PrefixBlock(ProtocolParser.REASONING_TOKEN, false);\n\t\tpublic static final PrefixBlock TOKEN_STATS = new PrefixBlock(ProtocolParser.TOKEN_STATS, false);\n\t\tpublic static final PrefixBlock SYSTEM_INIT = new PrefixBlock(ProtocolParser.SYSTEM_INIT, false);\n\t\tpublic static final PrefixBlock RESULT = new PrefixBlock(AIAnswer.RESULT, true);\n\t\tpublic static final AbstractNode[] ALL = of(THINKING_META, THINKING, TEXT, TOOLUSE, TOOLRESULT, REASONING_TOKEN,\n\t\t\t\tTOKEN_STATS, SYSTEM_INIT, RESULT);\n\t}"
        }, {
          "id" : "Elements.Tools",
          "type" : "class_declaration",
          "code" : "public static class Tools {\n\t\tpublic static final PrefixBlock ANSWER = new PrefixBlock(AnswerCommand.CMD_ANSWER, true);\n\t\tpublic static final LineSection CONTROL_REQUEST = new LineSection(EditorInterface.CONTROL_REQUEST, false, of(//\n\t\t\t\tANSWER, //\n\t\t\t\tBasics.SCRIPTBLOCK), //\n\t\t\t\tNONE// replaced later\n\t\t);\n\t\tpublic static final AbstractNode[] ALL = of(CONTROL_REQUEST);\n\t}"
        }, {
          "id" : "Elements.Page",
          "type" : "class_declaration",
          "code" : "public static class Page {\n\t\tprivate static final AbstractNode[] PAGE_ELEMENTS = concat( //\n\t\t\t\tHeadings.HEADINGS, //\n\t\t\t\tof(//\n\t\t\t\t\t\tBasics.PARAGRAPH //\n\t\t\t\t));\n\t\tpublic static final PageSection PAGE = new PageSection(PAGE_ELEMENTS);\n\t\tstatic {\n\t\t\tfor (int i = 0; i < Headings.HEADINGS.length; i++)\n\t\t\t\tHeadings.HEADINGS[i].terminals = concat( //\n\t\t\t\t\t\tof(PAGE), //\n\t\t\t\t\t\tTools.ALL, //\n\t\t\t\t\t\tAgent.ALL);\n\t\t}\n\t}"
        }, {
          "id" : "Elements.Chat",
          "type" : "class_declaration",
          "code" : "public static class Chat {\n\t\tprivate static final AbstractNode[] USER_ELEMENTS = concat( //\n\t\t\t\tHeadings.HEADINGS, //\n\t\t\t\tof( //\n\t\t\t\t\t\tBasics.SCRIPTBLOCK, //\n\t\t\t\t\t\tBasics.LINE_COMMENT, //\n\t\t\t\t\t\tPage.PAGE, //\n\t\t\t\t\t\tBasics.PARAGRAPH //\n\t\t\t\t));\n\n\t\tpublic static final LineSection USER = new LineSection(EditorInterface.USER, true, USER_ELEMENTS, //\n\t\t\t\tNONE); // later replaced\n\n\t\tprivate static final AbstractNode[] AGENT_ONLY = of( //\n\t\t\t\tTools.CONTROL_REQUEST, //\n\t\t\t\tAgent.THINKING_META, //\n\t\t\t\tAgent.THINKING, //\n\t\t\t\tAgent.TEXT, //\n\t\t\t\tAgent.TOOLUSE, //\n\t\t\t\tAgent.TOOLRESULT, //\n\t\t\t\tAgent.REASONING_TOKEN, //\n\t\t\t\tAgent.TOKEN_STATS, //\n\t\t\t\tAgent.SYSTEM_INIT, //\n\t\t\t\tAgent.RESULT //\n\t\t);\n\t\tprivate static final AbstractNode[] AGENT_ELEMENTS = concat( //\n\t\t\t\tAGENT_ONLY, //\n\t\t\t\tUSER_ELEMENTS //\n\t\t);\n\t\tpublic static final LineSection AGENT = new LineSection(EditorInterface.AGENT, false, AGENT_ELEMENTS, of(USER));\n\t\tpublic static final AbstractNode[] ALL = of(USER, AGENT);\n\n\t\tstatic {\n\t\t\tUSER.terminalNodes = of(AGENT);\n\t\t\tAgent.TEXT.terminalNodes = AGENT_ONLY;\n\t\t\tTools.CONTROL_REQUEST.terminalNodes = concat(//\n\t\t\t\t\tof(USER, AGENT, //\n\t\t\t\t\t\t\tTools.CONTROL_REQUEST //\n\t\t\t\t\t), //\n\t\t\t\t\tHeadings.HEADINGS, //\n\t\t\t\t\tof(Agent.THINKING, //\n\t\t\t\t\t\t\tAgent.TEXT, //\n\t\t\t\t\t\t\tAgent.TOOLUSE, //\n\t\t\t\t\t\t\tAgent.TOOLRESULT, //\n\t\t\t\t\t\t\tAgent.REASONING_TOKEN, //\n\t\t\t\t\t\t\tAgent.TOKEN_STATS, //\n\t\t\t\t\t\t\tAgent.SYSTEM_INIT, //\n\t\t\t\t\t\t\tAgent.RESULT, //\n\t\t\t\t\t\t\tBasics.PARAGRAPH //\n\t\t\t\t\t));\n\t\t}\n\t}"
        }, {
          "id" : "Elements.Roots",
          "type" : "class_declaration",
          "code" : "public static class Roots {\n\t\tprivate static final AbstractNode[] ROOT_ELEMENTS = concat( //\n\t\t\t\tof(Page.PAGE), //\n\t\t\t\tHeadings.HEADINGS, //\n\t\t\t\tChat.ALL, //\n\t\t\t\tAgent.ALL, //\n\t\t\t\tTools.ALL, //\n\t\t\t\tBasics.ALL //\n\t\t);\n\n\t\tpublic static final Root ROOT = new Root(ROOT_ELEMENTS);\n\n\t\tstatic {\n\t\t\t// all execpt itself\n\t\t\tBasics.PARAGRAPH.terminals = Stream.of(ROOT_ELEMENTS).filter(e -> e != Basics.PARAGRAPH)\n\t\t\t\t\t.toArray(AbstractNode[]::new);\n\t\t}\n\n\t}"
        }, {
          "id" : "Elements.xGT0VH|2VA8hK",
          "type" : "statements",
          "code" : "public static final Root ROOT = Roots.ROOT;"
        }, {
          "id" : "Elements.of",
          "type" : "method_declaration",
          "code" : "private static AbstractNode[] of(AbstractNode... nodes) {\n\t\treturn nodes;\n\t}"
        }, {
          "id" : "Elements.concat",
          "type" : "method_declaration",
          "code" : "private static AbstractNode[] concat(AbstractNode[]... s) {\n\t\tStream<AbstractNode> ss = Stream.of(s[0]);\n\t\tfor (int i = 1; i < s.length; i++)\n\t\t\tss = Stream.concat(ss, Stream.of(s[i]));\n\t\treturn ss.toArray(AbstractNode[]::new);\n\t}"
        } ]
      } ]
    } ]
  }
}
```
Result Stats: id=7aad80de-ccfc-4dad-b41e-5e78df5eee5a, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_01_rPyaCOZibp3pUTQSAxco5727",
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java",
      "nodes" : [ {
        "id" : "fau272|XRuDTB",
        "signature" : "package xy.ai.workbench.commands;"
      }, {
        "id" : "AnswerCommand",
        "signature" : "public class AnswerCommand extends Command {",
        "children" : [ {
          "id" : "AnswerCommand.LN3qky|soKy4k",
          "signature" : "public static final String CMD_ANSWER = \"/answer\";"
        }, {
          "id" : "AnswerCommand.AnswerCommand",
          "signature" : "public AnswerCommand() {"
        }, {
          "id" : "AnswerCommand.AnswerCommand_1",
          "signature" : "public AnswerCommand(String id, String action, String reason) {"
        }, {
          "id" : "AnswerCommand.prefix",
          "signature" : "public String prefix() {"
        }, {
          "id" : "AnswerCommand.matches",
          "signature" : "public boolean matches(String line) {"
        }, {
          "id" : "AnswerCommand.parse",
          "signature" : "public Command parse(String line) {"
        }, {
          "id" : "AnswerCommand.processorAction",
          "signature" : "public ProcessorAction processorAction() {"
        }, {
          "id" : "AnswerCommand.Action",
          "signature" : "public enum Action {"
        }, {
          "id" : "AnswerCommand.action",
          "signature" : "public Action action() {"
        } ]
      } ],
      "errors" : [ "ids [] resolved to no node; redirected to ast_list, returning the full outline instead." ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallCommand.java",
      "nodes" : [ {
        "id" : "fau272|XRuDTB",
        "signature" : "package xy.ai.workbench.commands;"
      }, {
        "id" : "CallCommand",
        "signature" : "public class CallCommand extends Command {",
        "children" : [ {
          "id" : "CallCommand.SX50AH|QuqZES",
          "signature" : "public static final String CMD_CALL = \"/call\";"
        }, {
          "id" : "CallCommand.CallCommand",
          "signature" : "public CallCommand() {"
        }, {
          "id" : "CallCommand.prefix",
          "signature" : "public String prefix() {"
        }, {
          "id" : "CallCommand.matches",
          "signature" : "public boolean matches(String line) {"
        }, {
          "id" : "CallCommand.parse",
          "signature" : "public Command parse(String line) {"
        }, {
          "id" : "CallCommand.processorAction",
          "signature" : "public ProcessorAction processorAction() {"
        } ]
      } ],
      "errors" : [ "ids [] resolved to no node; redirected to ast_list, returning the full outline instead." ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallEditCommand.java",
      "nodes" : [ {
        "id" : "KQ7DY1|Ws2Bgb",
        "signature" : "package xy.ai.workbench.commands;"
      }, {
        "id" : "5xOt41|JUnBPM",
        "signature" : "/**"
      }, {
        "id" : "CallEditCommand",
        "signature" : "public class CallEditCommand extends Command {",
        "children" : [ {
          "id" : "CallEditCommand.rU7BIw|BKuAVj",
          "signature" : "private static final Pattern YAML_BLOCK = Pattern.compile(\"^```yaml\\\\R(.*?)\\\\R?…"
        }, {
          "id" : "CallEditCommand.CallEditCommand",
          "signature" : "public CallEditCommand() {"
        }, {
          "id" : "CallEditCommand.CallEditCommand_1",
          "signature" : "private CallEditCommand(String yaml) {"
        }, {
          "id" : "CallEditCommand.prefix",
          "signature" : "public String prefix() {"
        }, {
          "id" : "CallEditCommand.matches",
          "signature" : "public boolean matches(String line) {"
        }, {
          "id" : "CallEditCommand.parse",
          "signature" : "public Command parse(String line) {"
        }, {
          "id" : "CallEditCommand.processorAction",
          "signature" : "public ProcessorAction processorAction() {"
        }, {
          "id" : "CallEditCommand.yaml",
          "signature" : "public String yaml() {"
        } ]
      } ],
      "errors" : [ "ids [] resolved to no node; redirected to ast_list, returning the full outline instead." ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/Command.java",
      "nodes" : [ {
        "id" : "fau272|XRuDTB",
        "signature" : "package xy.ai.workbench.commands;"
      }, {
        "id" : "Command",
        "signature" : "public abstract class Command {",
        "children" : [ {
          "id" : "Command.ProcessorAction",
          "signature" : "public enum ProcessorAction {"
        }, {
          "id" : "Command.iKqF62|lJcluI",
          "signature" : "protected final String[] parameters;"
        }, {
          "id" : "Command.Command",
          "signature" : "protected Command(String... parameters) {"
        }, {
          "id" : "Command.prefix",
          "signature" : "public abstract String prefix();"
        }, {
          "id" : "Command.matches",
          "signature" : "public boolean matches(String line) {"
        }, {
          "id" : "Command.parse",
          "signature" : "public abstract Command parse(String line);"
        }, {
          "id" : "Command.processorAction",
          "signature" : "public abstract ProcessorAction processorAction();"
        }, {
          "id" : "Command.parameter",
          "signature" : "public String parameter(int i) {"
        } ]
      } ],
      "errors" : [ "ids [] resolved to no node; redirected to ast_list, returning the full outline instead." ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ControlRequestCommand.java",
      "nodes" : [ {
        "id" : "FAE3JL|44PDgA",
        "signature" : "package xy.ai.workbench.commands;"
      }, {
        "id" : "ControlRequestCommand",
        "signature" : "public class ControlRequestCommand extends Command {",
        "children" : [ {
          "id" : "ControlRequestCommand.ControlRequestCommand",
          "signature" : "public ControlRequestCommand() {"
        }, {
          "id" : "ControlRequestCommand.prefix",
          "signature" : "public String prefix() {"
        }, {
          "id" : "ControlRequestCommand.parse",
          "signature" : "public Command parse(String line) {"
        }, {
          "id" : "ControlRequestCommand.processorAction",
          "signature" : "public ProcessorAction processorAction() {"
        } ]
      } ],
      "errors" : [ "ids [] resolved to no node; redirected to ast_list, returning the full outline instead." ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ExitCommand.java",
      "nodes" : [ {
        "id" : "fau272|XRuDTB",
        "signature" : "package xy.ai.workbench.commands;"
      }, {
        "id" : "ExitCommand",
        "signature" : "public class ExitCommand extends Command {",
        "children" : [ {
          "id" : "ExitCommand.Qvx8ey|kUwoIB",
          "signature" : "public static final String CMD_EXIT = \"/exit\";"
        }, {
          "id" : "ExitCommand.ExitCommand",
          "signature" : "public ExitCommand() {"
        }, {
          "id" : "ExitCommand.prefix",
          "signature" : "public String prefix() {"
        }, {
          "id" : "ExitCommand.matches",
          "signature" : "public boolean matches(String line) {"
        }, {
          "id" : "ExitCommand.parse",
          "signature" : "public Command parse(String line) {"
        }, {
          "id" : "ExitCommand.processorAction",
          "signature" : "public ProcessorAction processorAction() {"
        } ]
      } ],
      "errors" : [ "ids [] resolved to no node; redirected to ast_list, returning the full outline instead." ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ResumeCommand.java",
      "nodes" : [ {
        "id" : "fau272|XRuDTB",
        "signature" : "package xy.ai.workbench.commands;"
      }, {
        "id" : "ResumeCommand",
        "signature" : "public class ResumeCommand extends Command {",
        "children" : [ {
          "id" : "ResumeCommand.M0PdNg|ksR6XK",
          "signature" : "public static final String CMD_RESUME = \"/resume\";"
        }, {
          "id" : "ResumeCommand.ResumeCommand",
          "signature" : "public ResumeCommand() {"
        }, {
          "id" : "ResumeCommand.ResumeCommand_1",
          "signature" : "public ResumeCommand(String sessionId) {"
        }, {
          "id" : "ResumeCommand.prefix",
          "signature" : "public String prefix() {"
        }, {
          "id" : "ResumeCommand.matches",
          "signature" : "public boolean matches(String line) {"
        }, {
          "id" : "ResumeCommand.parse",
          "signature" : "public Command parse(String line) {"
        }, {
          "id" : "ResumeCommand.processorAction",
          "signature" : "public ProcessorAction processorAction() {"
        } ]
      } ],
      "errors" : [ "ids [] resolved to no node; redirected to ast_list, returning the full outline instead." ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java",
      "nodes" : [ {
        "id" : "fau272|XRuDTB",
        "signature" : "package xy.ai.workbench.commands;"
      }, {
        "id" : "ToolCommand",
        "signature" : "public class ToolCommand extends Command {",
        "children" : [ {
          "id" : "ToolCommand.18LEdl|nBKSDD",
          "signature" : "public static final String CMD_TOOL = \"/tool\";"
        }, {
          "id" : "ToolCommand.ToolCommand",
          "signature" : "public ToolCommand() {"
        }, {
          "id" : "ToolCommand.ToolCommand_1",
          "signature" : "public ToolCommand(String toolId) {"
        }, {
          "id" : "ToolCommand.prefix",
          "signature" : "public String prefix() {"
        }, {
          "id" : "ToolCommand.matches",
          "signature" : "public boolean matches(String line) {"
        }, {
          "id" : "ToolCommand.parse",
          "signature" : "public Command parse(String line) {"
        }, {
          "id" : "ToolCommand.processorAction",
          "signature" : "public ProcessorAction processorAction() {"
        } ]
      } ],
      "errors" : [ "ids [] resolved to no node; redirected to ast_list, returning the full outline instead." ]
    } ]
  }
}
```
Result Stats: id=804602419, total: 16342, in: 10481, out: 741, reason: 37, read: 5120, write: 0
Thinking:
I got outlines but not code for the command files. The ast_read with empty ids returned outlines. I need to read actual method bodies. Let me read specific nodes for command files.
Thinking Meta: {"type":"reasoning","id":"1141e032-4de4-446f-a5e1-b9fd628f10b9","status":"completed","content":[{"type":"reasoning_text","text":"{{reasoning-text-0}}"}],"summary":[],"encrypted_content":"5afc7876-df61-417a-93b2-889db3e00cd2-0"}
Tool:
```yaml
id: call_00_mWN3K8sWLqHpSBQDKrci7047
tool: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java
    ids:
    - AnswerCommand.LN3qky|soKy4k
    - AnswerCommand.AnswerCommand
    - AnswerCommand.prefix
    - AnswerCommand.matches
    - AnswerCommand.parse
    - AnswerCommand.processorAction
    - AnswerCommand.Action
    - AnswerCommand.action
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallCommand.java
    ids:
    - CallCommand.SX50AH|QuqZES
    - CallCommand.CallCommand
    - CallCommand.prefix
    - CallCommand.matches
    - CallCommand.parse
    - CallCommand.processorAction
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallEditCommand.java
    ids:
    - CallEditCommand.rU7BIw|BKuAVj
    - CallEditCommand.CallEditCommand
    - CallEditCommand.prefix
    - CallEditCommand.matches
    - CallEditCommand.parse
    - CallEditCommand.processorAction
    - CallEditCommand.yaml
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/Command.java
    ids:
    - Command.ProcessorAction
    - Command.Command
    - Command.prefix
    - Command.matches
    - Command.parse
    - Command.processorAction
    - Command.parameter
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ControlRequestCommand.java
    ids:
    - ControlRequestCommand.ControlRequestCommand
    - ControlRequestCommand.prefix
    - ControlRequestCommand.parse
    - ControlRequestCommand.processorAction
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ExitCommand.java
    ids:
    - ExitCommand.Qvx8ey|kUwoIB
    - ExitCommand.ExitCommand
    - ExitCommand.prefix
    - ExitCommand.matches
    - ExitCommand.parse
    - ExitCommand.processorAction
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ResumeCommand.java
    ids:
    - ResumeCommand.M0PdNg|ksR6XK
    - ResumeCommand.ResumeCommand
    - ResumeCommand.prefix
    - ResumeCommand.matches
    - ResumeCommand.parse
    - ResumeCommand.processorAction
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java
    ids:
    - ToolCommand.18LEdl|nBKSDD
    - ToolCommand.ToolCommand
    - ToolCommand.prefix
    - ToolCommand.matches
    - ToolCommand.parse
    - ToolCommand.processorAction
  reason: Read command method bodies for precise slash-command documentation.
```
/call call_00_mWN3K8sWLqHpSBQDKrci7047
Result Stats: id=17f382c8-5777-4c8b-80d5-a21218773002, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_00_mWN3K8sWLqHpSBQDKrci7047",
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/AnswerCommand.java",
      "nodes" : [ {
        "id" : "AnswerCommand.LN3qky|soKy4k",
        "type" : "statements",
        "code" : "public static final String CMD_ANSWER = \"/answer\";"
      }, {
        "id" : "AnswerCommand.AnswerCommand",
        "type" : "constructor_declaration",
        "code" : "public AnswerCommand() {\n\t\tthis(\"\", \"\", \"\");\n\t}"
      }, {
        "id" : "AnswerCommand.prefix",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic String prefix() {\n\t\treturn AnswerCommand.CMD_ANSWER;\n\t}"
      }, {
        "id" : "AnswerCommand.matches",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic boolean matches(String line) {\n\t\t// \"s\" (DOTALL) allows a block-selected multi-line reason/hint after allow|deny.\n\t\treturn line != null && line.strip().matches(\"(?is)\" + prefix() + \"\\\\s+\\\\S+\\\\s+(allow|deny)(\\\\s+.*)?\");\n\t}"
      }, {
        "id" : "AnswerCommand.parse",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic Command parse(String line) {\n\t\tString[] parts = line.strip().split(\"\\\\s+\", 4);\n\t\tString id = parts[1];\n\t\tString action = parts[2].toLowerCase();\n\t\tString reason = parts.length > 3 ? parts[3].strip() : \"\";\n\t\treturn new AnswerCommand(id, action, reason);\n\t}"
      }, {
        "id" : "AnswerCommand.processorAction",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.TRANSFORM;\n\t}"
      }, {
        "id" : "AnswerCommand.Action",
        "type" : "enum_declaration",
        "code" : "public enum Action {\n\t\tAllow, Deny\n\t}"
      }, {
        "id" : "AnswerCommand.action",
        "type" : "method_declaration",
        "code" : "public Action action() {\n\t\treturn \"allow\".equalsIgnoreCase(parameter(1)) ? Action.Allow : Action.Deny;\n\t}"
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallCommand.java",
      "nodes" : [ {
        "id" : "CallCommand.SX50AH|QuqZES",
        "type" : "statements",
        "code" : "public static final String CMD_CALL = \"/call\";"
      }, {
        "id" : "CallCommand.CallCommand",
        "type" : "constructor_declaration",
        "code" : "public CallCommand() {\n\t\tsuper();\n\t}"
      }, {
        "id" : "CallCommand.prefix",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic String prefix() {\n\t\treturn CallCommand.CMD_CALL;\n\t}"
      }, {
        "id" : "CallCommand.matches",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && line.strip().startsWith(prefix());\n\t}"
      }, {
        "id" : "CallCommand.parse",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic Command parse(String line) {\n\t\treturn new CallCommand();\n\t}"
      }, {
        "id" : "CallCommand.processorAction",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}"
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/CallEditCommand.java",
      "nodes" : [ {
        "id" : "CallEditCommand.rU7BIw|BKuAVj",
        "type" : "statements",
        "code" : "private static final Pattern YAML_BLOCK = Pattern.compile(\"^```yaml\\\\R(.*?)\\\\R?```$\", Pattern.DOTALL);"
      }, {
        "id" : "CallEditCommand.CallEditCommand",
        "type" : "constructor_declaration",
        "code" : "public CallEditCommand() {\n\t\tsuper();\n\t}"
      }, {
        "id" : "CallEditCommand.prefix",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic String prefix() {\n\t\treturn \"```yaml\";\n\t}"
      }, {
        "id" : "CallEditCommand.matches",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && YAML_BLOCK.matcher(line.strip()).matches();\n\t}"
      }, {
        "id" : "CallEditCommand.parse",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic Command parse(String line) {\n\t\tMatcher m = YAML_BLOCK.matcher(line.strip());\n\t\tm.matches();\n\t\treturn new CallEditCommand(m.group(1).strip());\n\t}"
      }, {
        "id" : "CallEditCommand.processorAction",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}"
      }, {
        "id" : "CallEditCommand.yaml",
        "type" : "method_declaration",
        "code" : "public String yaml() {\n\t\treturn parameter(0);\n\t}"
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/Command.java",
      "nodes" : [ {
        "id" : "Command.ProcessorAction",
        "type" : "enum_declaration",
        "code" : "public enum ProcessorAction {\n\t\tREMOVE, TRANSFORM, IGNORE_BLOCK, NONE\n\t}"
      }, {
        "id" : "Command.Command",
        "type" : "constructor_declaration",
        "code" : "protected Command(String... parameters) {\n\t\tthis.parameters = parameters;\n\t}"
      }, {
        "id" : "Command.prefix",
        "type" : "method_declaration",
        "code" : "public abstract String prefix();"
      }, {
        "id" : "Command.matches",
        "type" : "method_declaration",
        "code" : "public boolean matches(String line) {\n\t\treturn line != null && line.strip().startsWith(prefix());\n\t}"
      }, {
        "id" : "Command.parse",
        "type" : "method_declaration",
        "code" : "public abstract Command parse(String line);"
      }, {
        "id" : "Command.processorAction",
        "type" : "method_declaration",
        "code" : "public abstract ProcessorAction processorAction();"
      }, {
        "id" : "Command.parameter",
        "type" : "method_declaration",
        "code" : "public String parameter(int i) {\n\t\treturn parameters.length > i ? parameters[i] : null;\n\n\t}"
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ControlRequestCommand.java",
      "nodes" : [ {
        "id" : "ControlRequestCommand.ControlRequestCommand",
        "type" : "constructor_declaration",
        "code" : "public ControlRequestCommand() {\n\t\tsuper();\n\t}"
      }, {
        "id" : "ControlRequestCommand.prefix",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic String prefix() {\n\t\treturn EditorInterface.CONTROL_REQUEST;\n\t}"
      }, {
        "id" : "ControlRequestCommand.parse",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic Command parse(String line) {\n\t\treturn new ControlRequestCommand();\n\t}"
      }, {
        "id" : "ControlRequestCommand.processorAction",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.IGNORE_BLOCK;\n\t}"
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ExitCommand.java",
      "nodes" : [ {
        "id" : "ExitCommand.Qvx8ey|kUwoIB",
        "type" : "statements",
        "code" : "public static final String CMD_EXIT = \"/exit\";"
      }, {
        "id" : "ExitCommand.ExitCommand",
        "type" : "constructor_declaration",
        "code" : "public ExitCommand() {\n\t\tsuper();\n\t}"
      }, {
        "id" : "ExitCommand.prefix",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic String prefix() {\n\t\treturn ExitCommand.CMD_EXIT;\n\t}"
      }, {
        "id" : "ExitCommand.matches",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && prefix().equalsIgnoreCase(line.strip());\n\t}"
      }, {
        "id" : "ExitCommand.parse",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic Command parse(String line) {\n\t\treturn new ExitCommand();\n\t}"
      }, {
        "id" : "ExitCommand.processorAction",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}"
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ResumeCommand.java",
      "nodes" : [ {
        "id" : "ResumeCommand.M0PdNg|ksR6XK",
        "type" : "statements",
        "code" : "public static final String CMD_RESUME = \"/resume\";"
      }, {
        "id" : "ResumeCommand.ResumeCommand",
        "type" : "constructor_declaration",
        "code" : "public ResumeCommand() {\n\t\tthis(\"\");\n\t}"
      }, {
        "id" : "ResumeCommand.prefix",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic String prefix() {\n\t\treturn ResumeCommand.CMD_RESUME;\n\t}"
      }, {
        "id" : "ResumeCommand.matches",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && line.strip().matches(\"(?i)\" + prefix() + \"\\\\s+\\\\S+\");\n\t}"
      }, {
        "id" : "ResumeCommand.parse",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic Command parse(String line) {\n\t\treturn new ResumeCommand(line.strip().split(\"\\\\s+\", 2)[1].strip());\n\t}"
      }, {
        "id" : "ResumeCommand.processorAction",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}"
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/commands/ToolCommand.java",
      "nodes" : [ {
        "id" : "ToolCommand.18LEdl|nBKSDD",
        "type" : "statements",
        "code" : "public static final String CMD_TOOL = \"/tool\";"
      }, {
        "id" : "ToolCommand.ToolCommand",
        "type" : "constructor_declaration",
        "code" : "public ToolCommand() {\n\t\tthis(\"\");\n\t}"
      }, {
        "id" : "ToolCommand.prefix",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic String prefix() {\n\t\treturn ToolCommand.CMD_TOOL;\n\t}"
      }, {
        "id" : "ToolCommand.matches",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic boolean matches(String line) {\n\t\treturn line != null && line.strip().matches(\"(?i)\" + prefix() + \"\\\\s+\\\\S+\");\n\t}"
      }, {
        "id" : "ToolCommand.parse",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic Command parse(String line) {\n\t\treturn new ToolCommand(line.strip().split(\"\\\\s+\", 2)[1].strip());\n\t}"
      }, {
        "id" : "ToolCommand.processorAction",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic ProcessorAction processorAction() {\n\t\treturn ProcessorAction.REMOVE;\n\t}"
      } ]
    } ]
  }
}
```
Result Stats: id=1843188555, total: 19209, in: 2585, out: 368, reason: 131, read: 16256, write: 0
Thinking:
I now have most info. But I need to understand "Labels: Label:" -> segments with metadata. The user mentions "Label:" concept. The EditorInterface has USER, AGENT, TEXT, THINKING, THINKING_META, TOOLUSE, TOOLRESULT, CONTROL_REQUEST. Also "Label:" generic? Let me search for "Label" usage across source. The user says "Labels: 'Label:' -> Segmente mit Metadaten". Perhaps there is a generic "Label:" prefix that segments with metadata. Let me grep for "Label:" and MarkerRessourceScanner / getPromptTag to understand metadata.
Thinking Meta: {"type":"reasoning","id":"02983032-262c-4c56-87aa-3e682d7801df","status":"completed","content":[{"type":"reasoning_text","text":"{{reasoning-text-0}}"}],"summary":[],"encrypted_content":"4623b5c6-4499-440d-b4f9-4ee9bc18e830-0"}
Tool:
```yaml
id: call_00_OcGS4Gv6KU2Gx0Jdn7un5121
tool: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: Label:|getPromptTag|PROMPT_TAG|PromptTag|label
    include:
    - '*.java'
    exclude:
    - '*.min.js'
  reason: Locate generic label segment syntax and metadata handling.
```
/call call_00_OcGS4Gv6KU2Gx0Jdn7un5121
Tool:
```yaml
id: call_01_kZD2O94WZyujyzGLUBkw9449
tool: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: MarkerRessourceScanner|getPromptTag|PromptTag|LABEL|Label
    include:
    - '*.java'
    exclude:
    - '*.min.js'
  reason: Find prompt tag/marker scanner implementation for label metadata documentation.
```
/call call_01_kZD2O94WZyujyzGLUBkw9449
Result Stats: id=cb4cc576-e511-4a66-8b0d-c3d6356c1128, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_00_OcGS4Gv6KU2Gx0Jdn7un5121",
  "result" : {
    "results" : [ {
      "directory" : [ "/home/user/xyan/xy.ai.workbench/src" ],
      "matches" : [ {
        "path" : "xy/ai/workbench/EditorInterface.java",
        "matches" : [ {
          "lineno" : 145,
          "match" : "return MarkerRessourceScanner.getPromptTag(pattern.name(), req.getID());"
        } ]
      }, {
        "path" : "xy/ai/workbench/view/session/ModelParametersSection.java",
        "matches" : [ {
          "lineno" : 194,
          "match" : "private void toggleControl(Label label, Control ctrl, boolean enabled) {"
        }, {
          "lineno" : 195,
          "match" : "label.setEnabled(enabled);"
        }, {
          "lineno" : 196,
          "match" : "label.setVisible(enabled);"
        }, {
          "lineno" : 199,
          "match" : "((GridData) label.getLayoutData()).exclude = !enabled;"
        } ]
      }, {
        "path" : "xy/ai/workbench/view/AIBatchView.java",
        "matches" : [ {
          "lineno" : 189,
          "match" : "private TableColumn createColumn(Table parent, String label, int weight) {"
        }, {
          "lineno" : 193,
          "match" : "res.setText(label);"
        } ]
      }, {
        "path" : "xy/ai/workbench/view/ClaudeCodeSessionView.java",
        "matches" : [ {
          "lineno" : 392,
          "match" : "private TableViewerColumn createColumn(String label, int weight) {"
        }, {
          "lineno" : 394,
          "match" : "col.setText(label);"
        } ]
      }, {
        "path" : "xy/ai/workbench/marker/MarkerRessourceScanner.java",
        "matches" : [ {
          "lineno" : 493,
          "match" : "public static String getPromptTag(String meta, String id) {"
        } ]
      }, {
        "path" : "xy/ai/workbench/connector/google/GeminiConnector.java",
        "matches" : [ {
          "lineno" : 74,
          "match" : "Map<String, String> labels = new HashMap<>();"
        }, {
          "lineno" : 75,
          "match" : "labels.put(GeminiRequest.CUSTOM_ID, id + \"\");"
        }, {
          "lineno" : 84,
          "match" : "// .labels(labels) // not supported"
        } ]
      } ]
    } ]
  }
}
```
Result Stats: id=57182a57-82c3-41c0-a3b8-e8324cffeb0e, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_01_kZD2O94WZyujyzGLUBkw9449",
  "result" : {
    "results" : [ {
      "directory" : [ "/home/user/xyan/xy.ai.workbench/src" ],
      "matches" : [ {
        "path" : "xy/ai/workbench/IncludeAdapter.java",
        "matches" : [ {
          "lineno" : 150,
          "match" : "// LOG.info(\"Searchresult changed: \" + result.getLabel());"
        } ]
      }, {
        "path" : "xy/ai/workbench/EditorInterface.java",
        "matches" : [ {
          "lineno" : 33,
          "match" : "import xy.ai.workbench.marker.MarkerRessourceScanner;"
        }, {
          "lineno" : 145,
          "match" : "return MarkerRessourceScanner.getPromptTag(pattern.name(), req.getID());"
        } ]
      }, {
        "path" : "xy/ai/workbench/view/session/OutputModeSection.java",
        "matches" : [ {
          "lineno" : 23,
          "match" : "toolkit.createLabel(bottom, \"Output:\");"
        } ]
      }, {
        "path" : "xy/ai/workbench/view/session/ModelParametersSection.java",
        "matches" : [ {
          "lineno" : 16,
          "match" : "import org.eclipse.swt.widgets.Label;"
        }, {
          "lineno" : 47,
          "match" : "Label maxTokenLabel = toolkit.createLabel(top, \"Max Token:\");"
        }, {
          "lineno" : 48,
          "match" : "maxTokenLabel.setLayoutData(new GridData());"
        }, {
          "lineno" : 56,
          "match" : "Label tempLabel = toolkit.createLabel(top, \"Temp:\");"
        }, {
          "lineno" : 57,
          "match" : "tempLabel.setLayoutData(new GridData());"
        }, {
          "lineno" : 65,
          "match" : "Label topPLabel = toolkit.createLabel(top, \"TopP:\");"
        }, {
          "lineno" : 66,
          "match" : "topPLabel.setLayoutData(new GridData());"
        }, {
          "lineno" : 73,
          "match" : "createReasoningSection(top, body, cfg, tempLabel, temp, topPLabel, topP, maxTokenLabel, maxToken);"
        }, {
          "lineno" : 78,
          "match" : "toolkit.createLabel(top, \"Key:\");"
        }, {
          "lineno" : 89,
          "match" : "toolkit.createLabel(top, \"Model:\");"
        }, {
          "lineno" : 109,
          "match" : "Label profileLabel = toolkit.createLabel(top, \"Profile:\");"
        } ]
      } ],
      "warning" : "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    } ]
  }
}
```
Label meint alle prefixartigen Konstrukte. Beispielweise auch die Result Stats in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/AIAnswer.java`.
Result Stats: id=1997958882, total: 20509, in: 934, out: 375, reason: 131, read: 19200, write: 0
Thinking:
The user clarified: "Label" means all prefix-like constructs, e.g. also the Result Stats in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/AIAnswer.java`. I need to read AIAnswer.java and related ProtocolParser for REASONING_TOKEN, TOKEN_STATS, SYSTEM_INIT, RESULT, and MarkerRessourceScanner.getPromptTag for the "Label:" metadata format. Let me search and read.

Also need to understand the prompt tag format produced by MarkerRessourceScanner.getPromptTag. Let me read that file around line 493 and AIAnswer.java.
Thinking Meta: {"type":"reasoning","id":"4da39a1d-1b5a-441d-8e70-8c11cc7b26eb","status":"completed","content":[{"type":"reasoning_text","text":"{{reasoning-text-0}}"}],"summary":[],"encrypted_content":"38bcbe3b-9ff2-45fb-98e3-d2344acc4ea6-0"}
Tool:
```yaml
id: call_00_Fqu3uwFZ88bLvxjqnwdm3723
tool: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/AIAnswer.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
  reason: Outline AIAnswer and marker scanner to document result stats labels and prompt tag format.
```
/call call_00_Fqu3uwFZ88bLvxjqnwdm3723
Tool:
```yaml
id: call_01_N2mJ351VpPkTLRLmDc7V8347
tool: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: REASONING_TOKEN|TOKEN_STATS|SYSTEM_INIT|ProtocolParser|RESULT
    include:
    - '*.java'
    exclude:
    - '*.min.js'
  reason: Locate all label/prefix constants and their string values.
```
/call call_01_N2mJ351VpPkTLRLmDc7V8347
Result Stats: id=719ea9eb-b481-4120-bdc9-ee29c046d955, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_00_Fqu3uwFZ88bLvxjqnwdm3723",
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/AIAnswer.java",
      "nodes" : [ {
        "id" : "P0NBhV|7GvmCt",
        "signature" : "package xy.ai.workbench.models;"
      }, {
        "id" : "AIAnswer",
        "signature" : "public class AIAnswer {",
        "children" : [ {
          "id" : "AIAnswer.j6EUkY|xbZ5uh",
          "signature" : "public static final String RESULT = \"Result Stats: \";"
        }, {
          "id" : "AIAnswer.AIAnswer",
          "signature" : "public AIAnswer(String id) {"
        }, {
          "id" : "AIAnswer.toString",
          "signature" : "public String toString() {"
        }, {
          "id" : "AIAnswer.print",
          "signature" : "public String print() {"
        }, {
          "id" : "AIAnswer.fromString",
          "signature" : "public static AIAnswer fromString(String line) {"
        } ]
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
      "nodes" : [ {
        "id" : "PRiB7o|xEqgmJ",
        "signature" : "package xy.ai.workbench.marker;"
      }, {
        "id" : "msPtFF|g0Uy5k",
        "signature" : "import org.eclipse.core.resources.IMarker;"
      }, {
        "id" : "Lry2Uy|BOP77h",
        "signature" : "import org.eclipse.core.runtime.jobs.Job;"
      }, {
        "id" : "BhWDpq|8PQHOY",
        "signature" : "import org.eclipse.ui.IPartListener2;"
      }, {
        "id" : "uzcHjn|N8UB8W",
        "signature" : "import xy.ai.workbench.models.AIAnswer;"
      }, {
        "id" : "MarkerRessourceScanner",
        "signature" : "public class MarkerRessourceScanner implements IResourceChangeListener, IResour…",
        "children" : [ {
          "id" : "MarkerRessourceScanner.R6Wy2I|YzDKeQ",
          "signature" : "private static final String AIREQ_PREFIX = \"xy.ai.req\";"
        }, {
          "id" : "MarkerRessourceScanner.MarkerRessourceScanner",
          "signature" : "public MarkerRessourceScanner(BundleContext context) {"
        }, {
          "id" : "MarkerRessourceScanner.dispose",
          "signature" : "public void dispose(BundleContext context) {"
        }, {
          "id" : "MarkerRessourceScanner.visit",
          "signature" : "public boolean visit(IResourceDelta delta) throws CoreException {"
        }, {
          "id" : "MarkerRessourceScanner.rescannFile",
          "signature" : "private void rescannFile(IFile file) {"
        }, {
          "id" : "MarkerRessourceScanner.resourceChanged",
          "signature" : "public void resourceChanged(IResourceChangeEvent event) {"
        }, {
          "id" : "MarkerRessourceScanner.BBGnlK|IcbjdV",
          "signature" : "/**"
        }, {
          "id" : "MarkerRessourceScanner.findAndReplaceMarkers",
          "signature" : "public boolean findAndReplaceMarkers(AIAnswer ans, ITextEditor hint) {"
        }, {
          "id" : "MarkerRessourceScanner.replaceMarker",
          "signature" : "private boolean replaceMarker(AIAnswer ans, IMarker marker) {"
        }, {
          "id" : "MarkerRessourceScanner.m0F31x|GiPjgj",
          "signature" : "/**"
        }, {
          "id" : "MarkerRessourceScanner.resolveTagRange",
          "signature" : "private int[] resolveTagRange(IDocument doc, String requestId, int off, int len…"
        }, {
          "id" : "MarkerRessourceScanner.Y9bMXf|pb0Hqj",
          "signature" : "/**"
        }, {
          "id" : "MarkerRessourceScanner.findTagInDocument",
          "signature" : "private int[] findTagInDocument(IDocument doc, String requestId) {"
        }, {
          "id" : "MarkerRessourceScanner.h5oOYA|PhFO9t",
          "signature" : "/**"
        }, {
          "id" : "MarkerRessourceScanner.tryReplaceInEditor",
          "signature" : "private boolean tryReplaceInEditor(AIAnswer ans, ITextEditor editor) {"
        }, {
          "id" : "MarkerRessourceScanner.replaceInDocument",
          "signature" : "private boolean replaceInDocument(AIAnswer ans, ITextEditor editor, IDocument d…"
        }, {
          "id" : "MarkerRessourceScanner.findAndReplaceInOpenEditors",
          "signature" : "private boolean findAndReplaceInOpenEditors(AIAnswer ans) {"
        }, {
          "id" : "MarkerRessourceScanner.findOpenEditorFor",
          "signature" : "private ITextEditor findOpenEditorFor(IFile file) {"
        }, {
          "id" : "MarkerRessourceScanner.isAutoFollowModeEnabled",
          "signature" : "private boolean isAutoFollowModeEnabled() {"
        }, {
          "id" : "MarkerRessourceScanner.shouldAutoFollow",
          "signature" : "private boolean shouldAutoFollow(ITextEditor editor, IDocument doc) {"
        }, {
          "id" : "MarkerRessourceScanner.NPnkWw|vzfeEJ",
          "signature" : "/**"
        }, {
          "id" : "MarkerRessourceScanner.AutoFollowState",
          "signature" : "private static final class AutoFollowState {",
          "children" : [ {
            "id" : "MarkerRessourceScanner.AutoFollowState.hrG5hD|XY39OM",
            "signature" : "final ITextEditor editor;"
          }, {
            "id" : "MarkerRessourceScanner.AutoFollowState.AutoFollowState",
            "signature" : "AutoFollowState(ITextEditor editor, IDocumentListener listener) {"
          } ]
        }, {
          "id" : "MarkerRessourceScanner.QY71kA|oEhJzH",
          "signature" : "/**"
        }, {
          "id" : "MarkerRessourceScanner.qOS4ZL|NLQrud",
          "signature" : "/**"
        }, {
          "id" : "MarkerRessourceScanner.moveCursorToLastLineStart",
          "signature" : "private void moveCursorToLastLineStart(ITextEditor editor, IDocument doc) {"
        }, {
          "id" : "MarkerRessourceScanner.ensureAutoFollowListener",
          "signature" : "private void ensureAutoFollowListener(ITextEditor editor, IDocument doc) {"
        }, {
          "id" : "MarkerRessourceScanner.ensurePartCloseCleanupRegistered",
          "signature" : "private void ensurePartCloseCleanupRegistered() {"
        }, {
          "id" : "MarkerRessourceScanner.unwrapTextEditor",
          "signature" : "private ITextEditor unwrapTextEditor(IWorkbenchPart part) {"
        }, {
          "id" : "MarkerRessourceScanner.removeAutoFollowListenerFor",
          "signature" : "private void removeAutoFollowListenerFor(ITextEditor editor) {"
        }, {
          "id" : "MarkerRessourceScanner.wxHUUQ|l5SxJF",
          "signature" : "/**"
        }, {
          "id" : "MarkerRessourceScanner.tryMoveCursorToLastLineStart",
          "signature" : "private boolean tryMoveCursorToLastLineStart(ITextEditor editor, IDocument doc)…"
        }, {
          "id" : "MarkerRessourceScanner.getOpenTextEditors",
          "signature" : "private java.util.List<ITextEditor> getOpenTextEditors() {"
        }, {
          "id" : "MarkerRessourceScanner.getPromptTag",
          "signature" : "public static String getPromptTag(String meta, String id) {"
        } ]
      } ]
    } ]
  }
}
```
Result Stats: id=233a93e3-f7a8-451b-8fb4-a94c539401db, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_01_N2mJ351VpPkTLRLmDc7V8347",
  "result" : {
    "results" : [ {
      "directory" : [ "/home/user/xyan/xy.ai.workbench/src" ],
      "matches" : [ {
        "path" : "xy/ai/workbench/EditorInterface.java",
        "matches" : [ {
          "lineno" : 45,
          "match" : "public static final String TOOLRESULT = \"ToolResult:\";"
        } ]
      }, {
        "path" : "xy/ai/workbench/models/AIAnswer.java",
        "matches" : [ {
          "lineno" : 9,
          "match" : "public static final String RESULT = \"Result Stats: \";"
        }, {
          "lineno" : 34,
          "match" : "return String.format(\"%sid=%s, %s\", RESULT, sessionId, stats.print());"
        } ]
      }, {
        "path" : "xy/ai/workbench/editor/mdast/nodes/Elements.java",
        "matches" : [ {
          "lineno" : 7,
          "match" : "import xy.ai.workbench.connector.claudecode.ProtocolParser;"
        }, {
          "lineno" : 53,
          "match" : "public static final PrefixBlock TOOLRESULT = new PrefixBlock(EditorInterface.TOOLRESULT, false);"
        }, {
          "lineno" : 54,
          "match" : "public static final PrefixBlock REASONING_TOKEN = new PrefixBlock(ProtocolParser.REASONING_TOKEN, false);"
        }, {
          "lineno" : 55,
          "match" : "public static final PrefixBlock TOKEN_STATS = new PrefixBlock(ProtocolParser.TOKEN_STATS, false);"
        }, {
          "lineno" : 56,
          "match" : "public static final PrefixBlock SYSTEM_INIT = new PrefixBlock(ProtocolParser.SYSTEM_INIT, false);"
        }, {
          "lineno" : 57,
          "match" : "public static final PrefixBlock RESULT = new PrefixBlock(AIAnswer.RESULT, true);"
        }, {
          "lineno" : 58,
          "match" : "public static final AbstractNode[] ALL = of(THINKING_META, THINKING, TEXT, TOOLUSE, TOOLRESULT, REASONING_TOKEN,"
        }, {
          "lineno" : 59,
          "match" : "TOKEN_STATS, SYSTEM_INIT, RESULT);"
        }, {
          "lineno" : 107,
          "match" : "Agent.TOOLRESULT, //"
        }, {
          "lineno" : 108,
          "match" : "Agent.REASONING_TOKEN, //"
        }, {
          "lineno" : 109,
          "match" : "Agent.TOKEN_STATS, //"
        }, {
          "lineno" : 110,
          "match" : "Agent.SYSTEM_INIT, //"
        } ]
      } ],
      "warning" : "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    } ]
  }
}
```
Result Stats: id=1145470229, total: 22865, in: 1973, out: 412, reason: 61, read: 20480, write: 0
Thinking:
Need to read AIAnswer.java code and ProtocolParser constants, and MarkerRessourceScanner.getPromptTag. Also maybe the mdast node types (PrefixBlock, LineSection, PageSection, ScriptBlock, LineCommentBlock, Paragraph, HeadingSection) to document structure and semantics. Let me read.
Thinking Meta: {"type":"reasoning","id":"cf7f103d-444f-462e-a791-73f4eae504be","status":"completed","content":[{"type":"reasoning_text","text":"{{reasoning-text-0}}"}],"summary":[],"encrypted_content":"c0ac69a2-fd42-4e6d-a28a-524a0d6fb0e0-0"}
Tool:
```yaml
id: call_00_U9VXB6nOS35k9L4vFAVA0134
tool: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/AIAnswer.java
    ids:
    - AIAnswer.j6EUkY|xbZ5uh
    - AIAnswer.toString
    - AIAnswer.print
    - AIAnswer.fromString
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java
    ids:
    - MarkerRessourceScanner.getPromptTag
    - MarkerRessourceScanner.findAndReplaceMarkers
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ProtocolParser.java
    ids: []
  reason: "Read result stats label, prompt tag format, and protocol parser constants."
```
/call call_00_U9VXB6nOS35k9L4vFAVA0134
Tool:
```yaml
id: call_01_MRwXV87BOS3Vm3TwEdrN8343
tool: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode
    pattern: REASONING_TOKEN|TOKEN_STATS|SYSTEM_INIT
    include:
    - '*.java'
    exclude:
    - '*.min.js'
  reason: Find exact label strings for protocol parser prefixes.
```
/call call_01_MRwXV87BOS3Vm3TwEdrN8343
Result Stats: id=d846e59e-d312-4b4f-a8ab-c1fea279af8d, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_00_U9VXB6nOS35k9L4vFAVA0134",
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/AIAnswer.java",
      "nodes" : [ {
        "id" : "AIAnswer.j6EUkY|xbZ5uh",
        "type" : "statements",
        "code" : "public static final String RESULT = \"Result Stats: \";\n\n\tprivate static final Pattern ID_PATTERN = Pattern.compile(\"id=([^,]*), \");\n\n\tpublic final String id;\n\tpublic String sessionId;\n\tpublic final TokenStats stats = new TokenStats();\n\tpublic String answer = \"\";\n\tpublic String instructions = \"\";\n\t/**\n\t * Prompt that produced this answer; null once no longer available\n\t */\n\tpublic Prompt prompt;"
      }, {
        "id" : "AIAnswer.toString",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic String toString() {\n\t\treturn String.format(\"AIAnswer [stats=%s]\", stats);\n\t}"
      }, {
        "id" : "AIAnswer.print",
        "type" : "method_declaration",
        "code" : "public String print() {\n\t\treturn String.format(\"%sid=%s, %s\", RESULT, sessionId, stats.print());\n\t}"
      }, {
        "id" : "AIAnswer.fromString",
        "type" : "method_declaration",
        "code" : "public static AIAnswer fromString(String line) {\n\t\tif (line == null)\n\t\t\treturn null;\n\t\tMatcher idMatcher = ID_PATTERN.matcher(line);\n\t\tif (!idMatcher.find())\n\t\t\treturn null;\n\t\tTokenStats stats = TokenStats.fromString(line.substring(idMatcher.end()));\n\t\tif (stats == null)\n\t\t\treturn null;\n\t\tAIAnswer ans = new AIAnswer(idMatcher.group(1));\n\t\tans.stats.add(stats);\n\t\treturn ans;\n\t}"
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/marker/MarkerRessourceScanner.java",
      "nodes" : [ {
        "id" : "MarkerRessourceScanner.getPromptTag",
        "type" : "method_declaration",
        "code" : "public static String getPromptTag(String meta, String id) {\n\t\treturn String.format(\"[%s:%s:%s]\", AIREQ_PREFIX, meta, id);\n\t}"
      }, {
        "id" : "MarkerRessourceScanner.findAndReplaceMarkers",
        "type" : "method_declaration",
        "code" : "public boolean findAndReplaceMarkers(AIAnswer ans, ITextEditor hint) {\n\t\tif (hint != null && tryReplaceInEditor(ans, hint))\n\t\t\treturn true;\n\n\t\tIWorkspaceRoot root = ResourcesPlugin.getWorkspace().getRoot();\n\t\tboolean res = false;\n\t\ttry {\n\t\t\tIMarker[] markers = root.findMarkers(MARKER_ID, true, IResource.DEPTH_INFINITE);\n\t\t\tfor (IMarker marker : markers)\n\t\t\t\tif (ans.id.equals(marker.getAttribute(MARKER_REQ_ID_ATTR)))\n\t\t\t\t\tres |= replaceMarker(ans, marker);\n\t\t} catch (CoreException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t}\n\n\t\t// Fallback: the marker's stored position is outdated\n\t\tif (!res)\n\t\t\tres = findAndReplaceInOpenEditors(ans);\n\n\t\treturn res;\n\t}"
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ProtocolParser.java",
      "nodes" : [ {
        "id" : "W4WJQg|SOe5em",
        "signature" : "package xy.ai.workbench.connector.claudecode;"
      }, {
        "id" : "IAjvhQ|pjZy9G",
        "signature" : "import xy.ai.workbench.models.TokenStats;"
      }, {
        "id" : "ProtocolParser",
        "signature" : "public class ProtocolParser {",
        "children" : [ {
          "id" : "ProtocolParser.4Meoof|IpQvMt",
          "signature" : "public static final String SYSTEM_INIT = \"SystemInit: \";"
        }, {
          "id" : "ProtocolParser.parseLine",
          "signature" : "public void parseLine(CCResponse resp, CCSession session, SubMonitor sub, Strin…"
        }, {
          "id" : "ProtocolParser.updateLastParsedMessage",
          "signature" : "private void updateLastParsedMessage(CCResponse resp, CCSession session) {"
        }, {
          "id" : "ProtocolParser.parseResult",
          "signature" : "private void parseResult(CCResponse resp, JsonNode node) {"
        }, {
          "id" : "ProtocolParser.extractUsage",
          "signature" : "private void extractUsage(CCResponse resp, JsonNode node) {"
        }, {
          "id" : "ProtocolParser.appendEvents",
          "signature" : "public static void appendEvents(Map<String, String> events, StringBuilder resul…"
        }, {
          "id" : "ProtocolParser.parseToolUse",
          "signature" : "private void parseToolUse(CCResponse resp, JsonNode node) {"
        }, {
          "id" : "ProtocolParser.parseAssistantEvents",
          "signature" : "private void parseAssistantEvents(JsonNode node, CCResponse resp, boolean recor…"
        }, {
          "id" : "ProtocolParser.collectMessageDeltaEvent",
          "signature" : "private void collectMessageDeltaEvent(CCResponse resp, JsonNode node) {"
        }, {
          "id" : "ProtocolParser.parseRateLimitEvent",
          "signature" : "private void parseRateLimitEvent(JsonNode node) {"
        }, {
          "id" : "ProtocolParser.parseSystemInitEvent",
          "signature" : "private void parseSystemInitEvent(CCResponse resp, JsonNode node) {"
        }, {
          "id" : "ProtocolParser.joinErrors",
          "signature" : "private static String joinErrors(JsonNode errors) {"
        }, {
          "id" : "ProtocolParser.commented",
          "signature" : "public static String commented(String input) {"
        } ]
      } ],
      "errors" : [ "ids [] resolved to no node; redirected to ast_list, returning the full outline instead." ]
    } ]
  }
}
```
Result Stats: id=1c91f094-f423-4774-ae8f-d351b522cc6c, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_01_MRwXV87BOS3Vm3TwEdrN8343",
  "result" : {
    "results" : [ {
      "directory" : [ "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode" ],
      "matches" : [ {
        "path" : "ProtocolParser.java",
        "matches" : [ {
          "lineno" : 21,
          "match" : "public static final String SYSTEM_INIT = \"SystemInit: \";"
        }, {
          "lineno" : 22,
          "match" : "public static final String REASONING_TOKEN = \"ReasoningToken: \";"
        }, {
          "lineno" : 23,
          "match" : "public static final String TOKEN_STATS = \"Token Usage: \";"
        }, {
          "lineno" : 200,
          "match" : "String value = REASONING_TOKEN + thinkingTokens;"
        }, {
          "lineno" : 277,
          "match" : "String metadata = String.format(\"%s id=%s, cwd=%s, model=%s\", SYSTEM_INIT, sessionId, cwd, model);"
        } ]
      } ]
    } ]
  }
}
```
Result Stats: id=1379323958, total: 24697, in: 1555, out: 358, reason: 64, read: 22784, write: 0
Thinking:
I have most of what I need. Let me also read ProtocolParser lines around REASONING_TOKEN, TOKEN_STATS, SYSTEM_INIT to understand metadata formats. Read the full constants and parseResult/extractUsage maybe. Also AIAnswer TokenStats.print to document Result Stats metadata. Let me read TokenStats.
Thinking Meta: {"type":"reasoning","id":"eb9d27bf-a072-400e-ab31-b027fa6c18a7","status":"completed","content":[{"type":"reasoning_text","text":"{{reasoning-text-0}}"}],"summary":[],"encrypted_content":"bb464e46-203d-41ac-a1da-b850229dbcd3-0"}
Tool:
```yaml
id: call_00_wjJb3ADTA8JOrPlauaGm1797
tool: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ProtocolParser.java
    ids:
    - ProtocolParser.4Meoof|IpQvMt
    - ProtocolParser.parseResult
    - ProtocolParser.extractUsage
    - ProtocolParser.parseSystemInitEvent
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/TokenStats.java
    ids: []
  reason: "Read exact metadata formats for result, token usage, system init and token stats labels."
```
/call call_00_wjJb3ADTA8JOrPlauaGm1797
Tool:
```yaml
id: call_01_qyMV9b8aiMSK01IDlmhR1242
tool: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models
    pattern: class TokenStats|String print|fromString|STATS
    include:
    - '*.java'
    exclude:
    - '*.min.js'
  reason: Locate TokenStats print format for Result Stats documentation.
```
/call call_01_qyMV9b8aiMSK01IDlmhR1242
Result Stats: id=da876175-ad2b-48de-80b1-3e5e88e1ca3b, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_00_wjJb3ADTA8JOrPlauaGm1797",
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/ProtocolParser.java",
      "nodes" : [ {
        "id" : "ProtocolParser.4Meoof|IpQvMt",
        "type" : "statements",
        "code" : "public static final String SYSTEM_INIT = \"SystemInit: \";\n\tpublic static final String REASONING_TOKEN = \"ReasoningToken: \";\n\tpublic static final String TOKEN_STATS = \"Token Usage: \";\n\tprivate static final String TEXT_CACHE_PREEFIX = \"text\\0\";\n\tprivate static final int TOOL_INPUT_MAX_LENGTH = 120;\n\n\tprivate final HookParser postProcessor = new HookParser();"
      }, {
        "id" : "ProtocolParser.parseResult",
        "type" : "method_declaration",
        "code" : "private void parseResult(CCResponse resp, JsonNode node) {\n\t\tboolean isError = node.path(\"is_error\").asBoolean(false) || \"error\".equals(node.path(\"subtype\").asText());\n\t\tStringBuilder res = new StringBuilder();\n\n\t\t// plainText yields the logical result value without JSON quoting/escaping\n\t\t// (and handles a structured result node), instead of a bare asText().\n\t\tString resultText = postProcessor.process(JsonUtil.plainText(node.path(\"result\")));\n\t\tresp.events.remove(TEXT_CACHE_PREEFIX + resultText);\n\t\textractUsage(resp, node);\n\n\t\tappendEvents(resp.events, res);\n\n\t\tif (!resultText.isEmpty())\n\t\t\tres.append(resultText.strip()).append(\"\\n\");\n\n\t\t// Some subtypes (e.g. \"error_during_execution\") carry no \"result\" field but\n\t\t// report the failure(s) in an \"errors\" array instead.\n\t\tString errorsText = joinErrors(node.path(\"errors\"));\n\t\tif (!errorsText.isEmpty())\n\t\t\tres.append(errorsText.strip()).append(\"\\n\");\n\n\t\tresp.resultText = res.toString();\n\t\tresp.isError = isError;\n\t}"
      }, {
        "id" : "ProtocolParser.extractUsage",
        "type" : "method_declaration",
        "code" : "private void extractUsage(CCResponse resp, JsonNode node) {\n\t\t// Extract token usage information\n\t\tJsonNode modelUsage = node.path(\"modelUsage\");\n\t\tif (modelUsage.isObject()) {\n\t\t\t@SuppressWarnings(\"deprecation\")\n\t\t\tIterator<Entry<String, JsonNode>> fields = modelUsage.fields();\n\t\t\tTokenStats stats = new TokenStats();\n\t\t\tfields.forEachRemaining(entry -> {\n\t\t\t\tJsonNode usage = entry.getValue();\n\t\t\t\tstats.inputToken += usage.path(\"inputTokens\").asLong(0);\n\t\t\t\tstats.outputToken += usage.path(\"outputTokens\").asLong(0);\n\t\t\t\tstats.cacheRead += usage.path(\"cacheReadInputTokens\").asLong(0);\n\t\t\t\tstats.cacheCreate += usage.path(\"cacheCreationInputTokens\").asLong(0);\n\t\t\t});\n\t\t\tresp.stats.add(stats);\n\t\t}\n\t}"
      }, {
        "id" : "ProtocolParser.parseSystemInitEvent",
        "type" : "method_declaration",
        "code" : "private void parseSystemInitEvent(CCResponse resp, JsonNode node) {\n\t\tMap<String, String> assistantEvents = resp.events;\n\t\ttry {\n\t\t\tString cwd = node.path(\"cwd\").asText(\"\");\n\t\t\tString sessionId = node.path(\"session_id\").asText(\"\");\n\t\t\tString model = node.path(\"model\").asText(\"\");\n\n\t\t\tStringBuilder pluginNames = new StringBuilder();\n\t\t\tJsonNode plugins = node.path(\"plugins\");\n\t\t\tif (plugins.isArray()) {\n\t\t\t\tboolean first = true;\n\t\t\t\tfor (JsonNode plugin : plugins) {\n\t\t\t\t\tString name = plugin.path(\"name\").asText(\"\");\n\t\t\t\t\tif (!name.isEmpty()) {\n\t\t\t\t\t\tif (!first)\n\t\t\t\t\t\t\tpluginNames.append(\", \");\n\t\t\t\t\t\tpluginNames.append(name);\n\t\t\t\t\t\tfirst = false;\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\n\t\t\tString metadata = String.format(\"%s id=%s, cwd=%s, model=%s\", SYSTEM_INIT, sessionId, cwd, model);\n\t\t\tassistantEvents.putIfAbsent(\"system_init\\0metadata\", metadata);\n\t\t} catch (Exception e) {\n\t\t\tLOG.error(\"Failed to parse system init event\", e);\n\t\t\tthrow e;\n\t\t}\n\t}"
      } ]
    }, {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/TokenStats.java",
      "nodes" : [ {
        "id" : "KQ7DY1|ztjLQR",
        "signature" : "package xy.ai.workbench.models;"
      }, {
        "id" : "TokenStats",
        "signature" : "public class TokenStats {",
        "children" : [ {
          "id" : "TokenStats.tV0ZpA|zqghTH",
          "signature" : "public long inputToken;"
        }, {
          "id" : "TokenStats.toString",
          "signature" : "public String toString() {"
        }, {
          "id" : "TokenStats.jxwqEb|BLZv9v",
          "signature" : "private static final Pattern PATTERN = Pattern"
        }, {
          "id" : "TokenStats.print",
          "signature" : "public String print() {"
        }, {
          "id" : "TokenStats.ty4m8Z|5xtu53",
          "signature" : "/**"
        }, {
          "id" : "TokenStats.fromString",
          "signature" : "public static TokenStats fromString(String s) {"
        }, {
          "id" : "TokenStats.add",
          "signature" : "public void add(TokenStats stats) {"
        } ]
      } ],
      "errors" : [ "ids [] resolved to no node; redirected to ast_list, returning the full outline instead." ]
    } ]
  }
}
```
Result Stats: id=a45a80df-ca95-47c8-9bcb-96aa448d9f44, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_01_qyMV9b8aiMSK01IDlmhR1242",
  "result" : {
    "results" : [ {
      "directory" : [ "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models" ],
      "matches" : [ {
        "path" : "TokenStats.java",
        "matches" : [ {
          "lineno" : 6,
          "match" : "public class TokenStats {"
        }, {
          "lineno" : 24,
          "match" : "public String print() {"
        }, {
          "lineno" : 33,
          "match" : "public static TokenStats fromString(String s) {"
        } ]
      }, {
        "path" : "AIAnswer.java",
        "matches" : [ {
          "lineno" : 33,
          "match" : "public String print() {"
        }, {
          "lineno" : 37,
          "match" : "public static AIAnswer fromString(String line) {"
        }, {
          "lineno" : 43,
          "match" : "TokenStats stats = TokenStats.fromString(line.substring(idMatcher.end()));"
        } ]
      } ]
    } ]
  }
}
```
Result Stats: id=316294884, total: 26391, in: 1694, out: 121, reason: 0, read: 24576, write: 0
Tool:
```yaml
id: call_00_ET_RXj0VhktPweHqQohsf3R0306
tool: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/TokenStats.java
    ids:
    - TokenStats.toString
    - TokenStats.print
    - TokenStats.fromString
    - TokenStats.add
  reason: Read TokenStats format for Result Stats metadata documentation.
```
/call call_00_ET_RXj0VhktPweHqQohsf3R0306
Result Stats: id=cbba6dc2-d89e-4c62-9d42-3d6359187fb7, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_00_ET_RXj0VhktPweHqQohsf3R0306",
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/models/TokenStats.java",
      "nodes" : [ {
        "id" : "TokenStats.toString",
        "type" : "method_declaration",
        "code" : "@Override\n\tpublic String toString() {\n\t\treturn String.format(\n\t\t\t\t\"TokenStats [inputToken=%s, outputToken=%s, reasoningToken=%s, totalInToken=%s, cacheRead=%s, cacheCreate=%s]\",\n\t\t\t\tinputToken, outputToken, reasoningToken, totalToken, cacheRead, cacheCreate);\n\t}"
      }, {
        "id" : "TokenStats.print",
        "type" : "method_declaration",
        "code" : "public String print() {\n\t\treturn String.format(\"total: %s, in: %s, out: %s, reason: %s, read: %s, write: %s\", totalToken, inputToken,\n\t\t\t\toutputToken, reasoningToken, cacheRead, cacheCreate);\n\t}"
      }, {
        "id" : "TokenStats.fromString",
        "type" : "method_declaration",
        "code" : "public static TokenStats fromString(String s) {\n\t\tif (s == null)\n\t\t\treturn null;\n\t\tMatcher m = PATTERN.matcher(s);\n\t\tif (!m.find())\n\t\t\treturn null;\n\t\tTokenStats stats = new TokenStats();\n\t\tstats.totalToken = Long.parseLong(m.group(1));\n\t\tstats.inputToken = Long.parseLong(m.group(2));\n\t\tstats.outputToken = Long.parseLong(m.group(3));\n\t\tstats.reasoningToken = Long.parseLong(m.group(4));\n\t\tstats.cacheRead = Long.parseLong(m.group(5));\n\t\tstats.cacheCreate = Long.parseLong(m.group(6));\n\t\treturn stats;\n\t}"
      }, {
        "id" : "TokenStats.add",
        "type" : "method_declaration",
        "code" : "public void add(TokenStats stats) {\n\t\tinputToken += stats.inputToken;\n\t\toutputToken += stats.outputToken;\n\t\treasoningToken += stats.reasoningToken;\n\t\ttotalToken += stats.totalToken;\n\t\tcacheRead += stats.cacheRead;\n\t\tcacheCreate += stats.cacheCreate;\n\t}"
      } ]
    } ]
  }
}
```
Result Stats: id=1631751753, total: 27186, in: 546, out: 272, reason: 152, read: 26368, write: 0
Thinking:
Now I have all necessary info. Let me also check if there is a "Label:" prefix specifically. The user first said Labels "Label:" -> segments with metadata, then clarified Label means all prefix-like constructs, e.g. Result Stats in AIAnswer. So I'll document all prefix/label constructs as "Labels" not just "Label:" literal.

I should also check for the prompt tag format: `[xy.ai.req:<meta>:<id>]`. The generateTag uses pattern.name() as meta. What is KeyPattern names? Could search, but not necessary. But maybe helpful to document that "Label" prompt markers appear as `[xy.ai.req:...:...]`. Let me search for KeyPattern enum.
Thinking Meta: {"type":"reasoning","id":"03427267-ca6a-4da6-9985-959427e9d626","status":"completed","content":[{"type":"reasoning_text","text":"{{reasoning-text-0}}"}],"summary":[],"encrypted_content":"51a82a89-99ba-4da6-92c1-796d0685c184-0"}
Tool:
```yaml
id: call_00_baUQJjhQxgMxmTBnNlxb4097
tool: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src
    pattern: enum KeyPattern|KeyPattern|getSupportedKeyPattern
    include:
    - '*.java'
    exclude:
    - '*.min.js'
  reason: Identify prompt-tag meta values used in generated labels.
```
/call call_00_baUQJjhQxgMxmTBnNlxb4097
Result Stats: id=387e4bdc-e71c-40fa-91f7-2d2f01fc421a, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_00_baUQJjhQxgMxmTBnNlxb4097",
  "result" : {
    "results" : [ {
      "directory" : [ "/home/user/xyan/xy.ai.workbench/src" ],
      "matches" : [ {
        "path" : "xy/ai/workbench/MementoConverter.java",
        "matches" : [ {
          "lineno" : 30,
          "match" : "snap.modelProvider = cfg.model.cap.getKeyPattern().name();"
        } ]
      }, {
        "path" : "xy/ai/workbench/EditorInterface.java",
        "matches" : [ {
          "lineno" : 31,
          "match" : "import xy.ai.workbench.Model.KeyPattern;"
        }, {
          "lineno" : 144,
          "match" : "KeyPattern pattern = connector.getConnector(req).getSupportedKeyPattern();"
        } ]
      }, {
        "path" : "xy/ai/workbench/model/DeepseekModelResolver.java",
        "matches" : [ {
          "lineno" : 18,
          "match" : "import xy.ai.workbench.Model.KeyPattern;"
        }, {
          "lineno" : 38,
          "match" : "Capabilities cap = ModelsDevCatalog.get().toCapabilities(KeyPattern.Deepseek, node);"
        }, {
          "lineno" : 51,
          "match" : "return Arrays.asList(Model.defaultsFor(KeyPattern.Deepseek));"
        } ]
      }, {
        "path" : "xy/ai/workbench/model/AnthropicModelResolver.java",
        "matches" : [ {
          "lineno" : 18,
          "match" : "import xy.ai.workbench.Model.KeyPattern;"
        }, {
          "lineno" : 40,
          "match" : "return Arrays.asList(Model.defaultsFor(KeyPattern.Claude));"
        }, {
          "lineno" : 45,
          "match" : "return Arrays.asList(Model.defaultsFor(KeyPattern.Claude));"
        }, {
          "lineno" : 50,
          "match" : "Capabilities cap = new Capabilities().key(KeyPattern.Claude);"
        } ]
      }, {
        "path" : "xy/ai/workbench/model/ModelResolver.java",
        "matches" : [ {
          "lineno" : 9,
          "match" : "* provider (identified by {@link Model.KeyPattern}) and resolves the models"
        } ]
      }, {
        "path" : "xy/ai/workbench/model/OpenAIModelResolver.java",
        "matches" : [ {
          "lineno" : 18,
          "match" : "import xy.ai.workbench.Model.KeyPattern;"
        }, {
          "lineno" : 41,
          "match" : "return Arrays.asList(Model.defaultsFor(KeyPattern.OpenAI));"
        }, {
          "lineno" : 46,
          "match" : "return Arrays.asList(Model.defaultsFor(KeyPattern.OpenAI));"
        }, {
          "lineno" : 51,
          "match" : "for (Model dflt : Model.defaultsFor(KeyPattern.OpenAI))"
        } ]
      } ],
      "warning" : "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
    } ]
  }
}
```
Result Stats: id=1691248707, total: 27818, in: 564, out: 118, reason: 0, read: 27136, write: 0
Tool:
```yaml
id: call_00_ET_DRjxgu3jjnwNQ2uiyK2S1421
tool: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench
    pattern: enum KeyPattern
    include:
    - '*.java'
    exclude:
    - '*.min.js'
  reason: Find KeyPattern enum values for meta in prompt tags.
```
/call call_00_ET_DRjxgu3jjnwNQ2uiyK2S1421
Result Stats: id=b20e8f37-70f6-4c12-a25f-f012d70d7f5d, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_00_ET_DRjxgu3jjnwNQ2uiyK2S1421",
  "result" : {
    "results" : [ {
      "directory" : [ "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench" ],
      "matches" : [ {
        "path" : "Model.java",
        "matches" : [ {
          "lineno" : 39,
          "match" : "public static enum KeyPattern {"
        } ]
      } ]
    } ]
  }
}
```
Result Stats: id=1562383953, total: 27980, in: 109, out: 95, reason: 0, read: 27776, write: 0
Tool:
```yaml
id: call_00_ET_rZ2nC3XDKysqMAmnOMjk7944
tool: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Model.java
    ids: []
  reason: Read KeyPattern enum values.
```
/call call_00_ET_rZ2nC3XDKysqMAmnOMjk7944
Result Stats: id=97fe9cfa-bbae-4c56-b1ac-7a20a6a1581d, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_00_ET_rZ2nC3XDKysqMAmnOMjk7944",
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Model.java",
      "nodes" : [ {
        "id" : "KQ7DY1|hLhpmj",
        "signature" : "package xy.ai.workbench;"
      }, {
        "id" : "Model",
        "signature" : "public class Model {",
        "children" : [ {
          "id" : "Model.POnqvV|QWbGOe",
          "signature" : "public final String apiName;"
        }, {
          "id" : "Model.Model",
          "signature" : "public Model(String apiName, String displayName, Capabilities cap) {"
        }, {
          "id" : "Model.Model_1",
          "signature" : "public Model(String apiName, Capabilities cap) {"
        }, {
          "id" : "Model.toString",
          "signature" : "public String toString() {"
        }, {
          "id" : "Model.333ZPM|6QgBuP",
          "signature" : "/*"
        }, {
          "id" : "Model.matches",
          "signature" : "public boolean matches(KeyPattern provider, String apiName) {"
        }, {
          "id" : "Model.KeyPattern",
          "signature" : "public static enum KeyPattern {"
        }, {
          "id" : "Model.V9NlVG|662wzi",
          "signature" : "// Default model catalog: used by the DefaultModelResolver, i.e. as long as no"
        }, {
          "id" : "Model.Cslw8j|w6aUVQ",
          "signature" : "public static final Model CC_HAIKU = new Model(\"haiku\", \"CC Haiku\", new Capabil…"
        }, {
          "id" : "Model.Cslw8j|gIVZ6i",
          "signature" : "public static final Model CC_SONNET = new Model(\"sonnet\", \"CC Sonnet\", new Capa…"
        }, {
          "id" : "Model.Cslw8j|SpJpvW",
          "signature" : "public static final Model CC_OPUS = new Model(\"opus\", \"CC Opus\", new Capabiliti…"
        }, {
          "id" : "Model.Cslw8j|S3bEua",
          "signature" : "public static final Model CC_MCPC_HAIKU = new Model(\"haiku\", \"CC Haiku (mcpc)\",…"
        }, {
          "id" : "Model.Cslw8j|c1M6oM",
          "signature" : "public static final Model CC_MCPC_SONNET = new Model(\"sonnet\", \"CC Sonnet (mcpc…"
        }, {
          "id" : "Model.Cslw8j|dwEMq8",
          "signature" : "public static final Model CC_MCPC_OPUS = new Model(\"opus\", \"CC Opus (mcpc)\", ne…"
        }, {
          "id" : "Model.O2u8qG|WcA7WQ",
          "signature" : "public static final Model MCP_TOOLS = new Model(\"mcp-tools\", \"MCP Tools\", new C…"
        }, {
          "id" : "Model.defaultsFor",
          "signature" : "public static Model[] defaultsFor(KeyPattern provider) {"
        } ]
      } ],
      "errors" : [ "ids [] resolved to no node; redirected to ast_list, returning the full outline instead." ]
    } ]
  }
}
```
Result Stats: id=1004690949, total: 28717, in: 714, out: 99, reason: 0, read: 27904, write: 0
Tool:
```yaml
id: call_00_ET_mOzDxcQXnAtSG2O8sF4r3956
tool: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Model.java
    ids:
    - Model.KeyPattern
  reason: Read KeyPattern enum values.
```
/call call_00_ET_mOzDxcQXnAtSG2O8sF4r3956
Result Stats: id=8de27f5e-4731-43de-9b6e-29728818236e, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
ToolResult:
```json
{
  "id" : "call_00_ET_mOzDxcQXnAtSG2O8sF4r3956",
  "result" : {
    "results" : [ {
      "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Model.java",
      "nodes" : [ {
        "id" : "Model.KeyPattern",
        "type" : "enum_declaration",
        "code" : "public static enum KeyPattern {\n\t\tOpenAI(\"^sk-proj-.*$\"), Gemini(\"^[a-zA-Z0-9]{39}$\"), Claude(\"^sk-ant-api.*$\"), Deepseek(\"^sk-[a-z0-9]{32}$\"),\n\t\tNone(\"^none$\"), ClaudeCode(\"^(work|personal)$\"), Misc(\"^.*$\");\n\n\t\tpublic final Pattern pattern;\n\n\t\tprivate KeyPattern(String pattern) {\n\t\t\tthis.pattern = Pattern.compile(pattern);\n\t\t}\n\n\t\tpublic boolean matches(String key) {\n\t\t\treturn pattern.matcher(key).matches();\n\t\t}\n\n\t}"
      } ]
    } ]
  }
}
```
Result Stats: id=1115551531, total: 33009, in: 269, out: 4068, reason: 3017, read: 28672, write: 0
Thinking:
Now I have all. I should produce a compact English listing/documentation for end users, covering:

1. Labels: prefix-like constructs ("Label:") that segment content with metadata.
   - The literal labels in EditorInterface:
     - `User:` — user section, begins a user turn; in editor interface inserted before prompt tag in Chat mode.
     - `Agent:` — agent response section.
     - `Text:` — agent text section.
     - `Thinking:` — agent thinking/reasoning section (consumed as reasoning).
     - `Thinking Meta:` — metadata within thinking (PrefixBlock, no newline? "false" means?). Need interpret PrefixBlock/LineSection args. The second boolean maybe "ends section"? Let's be careful. In Elements, `new PrefixBlock(EditorInterface.THINKING_META, false)`; `new LineSection(EditorInterface.THINKING, false, of(Basics.PARAGRAPH), of(THINKING_META))`. I can document semantics from SessionProcessor.run: it checks stripped.equals(USER), AGENT, THINKING, TEXT, TOOLUSE, TOOLRESULT; AIAnswer.RESULT prefix. The reasoning token, token usage, system init prefixes are parsed via ProtocolParser.
   - `Tool:` — tool call.
   - `ToolResult:` — tool result.
   - `Control Request:` — control request section, ignore block.
   - `Result Stats: ` — final result stats label with metadata `id=..., total:..., in:..., out:..., reason:..., read:..., write:...`.
   - `SystemInit: ` — metadata `id=..., cwd=..., model=...`.
   - `ReasoningToken: ` — reasoning token count.
   - `Token Usage: ` — token usage.
   - Prompt tag label: `[xy.ai.req:<meta>:<id>]` — generated marker, where meta is KeyPattern name (OpenAI, Gemini, Claude, Deepseek, None, ClaudeCode, Misc).

Let me verify exact strings. In EditorInterface: 
- USER = "User:"
- AGENT = "Agent:"
- TEXT = "Text:"
- THINKING = "Thinking:"
- THINKING_META = "Thinking Meta:"
- TOOLUSE = "Tool:"
- TOOLRESULT = "ToolResult:"
- CONTROL_REQUEST = "Control Request:"

In AIAnswer: RESULT = "Result Stats: " (with trailing space).

ProtocolParser:
- SYSTEM_INIT = "SystemInit: " (trailing space)
- REASONING_TOKEN = "ReasoningToken: " 
- TOKEN_STATS = "Token Usage: "

Note trailing spaces on some. Document format.

2. Slash commands:
   - `/answer <id> allow|deny [reason]` — TRANSFORM: replaces line with reason/hint? Actually processorAction TRANSFORM, SessionProcessor does: `String rest = command.parameter(2); if rest != null && !rest.isBlank() buffer.append(rest);`. So the answer command transforms into the reason text (parameter index 2 = reason). It's used in control requests to approve/deny with optional reason. The command itself `/answer` is in Elements.Tools as PrefixBlock with CMD_ANSWER and `true` maybe "start of section". But for the docs, the slash command listing from CommandRegistry:
     - AnswerCommand: `/answer <id> allow|deny [reason]` — matches multi-line reason/hint; action TRANSFORM; when processed in session, it's replaced by the optional reason text.
     - ControlRequestCommand: `Control Request:` prefix — action IGNORE_BLOCK (skips following fenced block). Not a slash command per se but a command recognized by processor. The user asked for slash commands but also "Kommandoimplementierung" in commands folder. I can include Control Request as a control prefix recognized by the same registry.
     - ResumeCommand: `/resume <session-id>` — action REMOVE (line removed when sending).
     - ExitCommand: `/exit` — action REMOVE (line removed).
     - ToolCommand: `/tool <tool-id>` — action REMOVE.
     - CallCommand: `/call ...` (startsWith /call) — action REMOVE.
     - CallEditCommand: fenced YAML block ```` ```yaml ... ``` ```` — action REMOVE. It's not slash but a command recognized by the registry.

Need to interpret what REMOVE means: SessionProcessor's run when command detected with REMOVE: flush; i++; continue. So the command line is discarded from the prompt content. TRANSFORM: replaces with rest (parameter(2)), which for answer is reason; IGNORE_BLOCK: flush and skip the fenced block after it.

Let me double check matches:
- AnswerCommand matches: line.strip().matches("(?is)/answer\s+\S+\s+(allow|deny)(\s+.*)?") — so `/answer <id> allow|deny [reason/hint]`. parse splits into 4 parts: id=parts[1], action=parts[2], reason=parts[3]. ProcessorAction TRANSFORM. In SessionProcessor TRANSFORM uses parameter(2) which is the reason. So the whole `/answer ...` line is replaced by reason text (if any). Note: parse parameters: `new AnswerCommand(id, action, reason)` -> super parameters = [id, action, reason]; parameter(2)=reason. Good.
- ControlRequestCommand: prefix "Control Request:", base matches startsWith prefix. action IGNORE_BLOCK. So a line starting with `Control Request:` causes following fenced block (fenceRange(lines, i+1)) to be skipped. Actually fenceRange finds a fenced block. So control request line is removed and the following code fence block is ignored (not sent to model).
- ResumeCommand: matches /resume\s+\S+; parse takes second token as sessionId; action REMOVE. The sessionId used elsewhere? ProcessorAction only REMOVE, so in SessionProcessor it's just removed. But maybe the editor/dialog uses it? Not in SessionProcessor. We can document only what SessionProcessor does: line removed. However the command's session id may be used by other components. But user wants end-user docs from given sources; we should be careful not to overclaim. We can say "recognized and removed before sending; the session id is captured but not processed by SessionProcessor" — maybe too dev. Better: "The line is recognized and removed from the prompt during session processing." Keep user-facing.
- ExitCommand: exact line `/exit` (case-insensitive), REMOVE.
- ToolCommand: `/tool <tool-id>`, REMOVE.
- CallCommand: line starting with `/call`, REMOVE. Matches any line starting /call.
- CallEditCommand: a complete fenced YAML block: ```` ```yaml ... ``` ````; matches entire string; REMOVE. It's not a slash command; document as "YAML fenced block (command-like)".

Also Elements.Tools has `ANSWER = new PrefixBlock(AnswerCommand.CMD_ANSWER, true)` and `CONTROL_REQUEST = LineSection(EditorInterface.CONTROL_REQUEST, false, of(ANSWER, SCRIPTBLOCK), NONE)`. That means in mdast structure, `Control Request:` section contains `/answer` and script blocks. Good.

3. Include instructions:
   - Syntax: `[include](path)` or `[include <kind>](arg)`, multiple per line allowed.
   - Pattern: `INCLUDE_LINE` matches line consisting only of one or more include directives (with optional leading whitespace). `INCLUDE_TAG` = `[include(?:\\s+(\\w+))?]\\(([^)]+)\\)`.
   - Kind defaults to path include: `[include](<file-path>)` — reads file, absolute normalized path, recursion with cycle detection (IncludeCycleException), processes nested includes in same way.
   - `[include contextprompt](<dir>)` — adapter context prompt; adds user message.
   - `[include files](<arg>)` — adapter files; adds tool results.
   - `[include file](<path>)` — adapter file; adds tool result.
   - `[include search](files|matches)` — adapter search; `files` returns file entries, `matches` returns rendered YAML list with file/line/text.
   - Includes are processed by SessionProcessor when the whole line matches. Multiple includes on a line processed left to right.
   - Includes can be nested; includePath reads file content and runs new Run with chain to detect cycles.

I should also document "Prefixzuordnung" (prefix assignment) from Elements.java and SessionProcessor.java: mapping of labels to roles/segments.

Let me structure final answer in English, compact, user-facing. The user said "Die Dokumentation richtet sich an Verwender nicht Entwickler" — for users, not developers. So write in English, concise, non-technical but precise. Use tables.

I should produce a comprehensive but compact list. Let me draft.

Section 1: Labels (prefixes that segment content)
Explain: A label is a line that starts with a fixed prefix followed by a colon (or space-colon), and marks the beginning of a segment. Some labels carry metadata after them.

List:
- `User:` — user message/turn segment. Content after it is treated as user input. Terminates prior agent section.
- `Agent:` — agent answer segment.
- `Thinking:` — agent reasoning; content consumed as reasoning.
- `Thinking Meta:` — metadata block inside thinking.
- `Text:` — agent text answer content.
- `Tool:` — tool call.
- `ToolResult:` — tool result.
- `Control Request:` — control block; the following fenced block is ignored/not sent.
- `Result Stats: ` — result metadata; format: `Result Stats: id=<session id>, total: <t>, in: <i>, out: <o>, reason: <r>, read: <cr>, write: <cw>`
- `SystemInit: ` — session metadata; format `SystemInit: id=<session id>, cwd=<dir>, model=<model>`
- `ReasoningToken: ` — count of reasoning tokens.
- `Token Usage: ` — token usage segment.
- Prompt tag: `[xy.ai.req:<provider>:<request id>]` — generated placeholder; provider is one of OpenAI, Gemini, Claude, Deepseek, None, ClaudeCode, Misc.

I need be accurate: THINKING_META and Token Usage/ReasoningToken/SystemInit have trailing spaces. Include exact strings with quotes. Let me include a table with "Exact prefix" and "Meaning".

Also for "Result Stats" the AIAnswer.print format is `Result Stats: id=%s, %s` where stats.print = `total: %s, in: %s, out: %s, reason: %s, read: %s, write: %s`. So full: `Result Stats: id=<sessionId>, total: <totalToken>, in: <inputToken>, out: <outputToken>, reason: <reasoningToken>, read: <cacheRead>, write: <cacheCreate>`.

Note AIAnswer.fromString expects `id=([^,]*), ` then TokenStats pattern. Fine.

Section 2: Slash commands and command-like blocks
List with syntax, recognition, effect:
- `/answer <id> allow|deny [reason]` — Approve/deny a control request. When the prompt is processed, the whole line is replaced by the optional reason text (the line itself is not sent verbatim).
- `/resume <session-id>` — recognized, removed from prompt.
- `/exit` — recognized, removed.
- `/tool <tool-id>` — recognized, removed.
- `/call ...` — any line starting with `/call`; removed.
- `Control Request:` — not a slash command but a label recognized; following fenced code block is skipped.
- ```` ```yaml ... ``` ```` — YAML command block; removed.

Wait the CallCommand matches any line starting with /call, so `/call` with or without args removed. Document "Any line beginning with /call is removed." 

Need to clarify AnswerCommand parse: `parts[1]` is id, `parts[2]` action, `parts[3]` reason. If missing id? Matches requires `/answer\s+\S+\s+(allow|deny)`, so id present, action present. Reason optional.

Also note AnswerCommand allows multi-line reason? Matches uses DOTALL so line can contain newlines? But SessionProcessor splits input into lines first, so a multi-line block selected? The comment says block-selected multi-line reason/hint. Hmm but detect gets a single line. In practice, editor may pass multi-line string? But not crucial. I'll note "optional reason text".

Section 3: Include directives
Syntax examples. Provide table.

Also mention: include directives are only recognized when a line contains only include directives (possibly multiple). That is important: they must be on their own line.

Let me draft final answer in English, compact. I'll use headings and tables. No need for tool calls further.

I should ensure I don't add a closing summary (per instructions: "Generiere keine abschließende Zusammenfassung"). So no "In summary" at end. Just end with the content.

Let me write.

Potential nuance: "Label:" segments with metadata — "Thinking Meta:" PrefixBlock, "Result Stats:" etc. Also `Text:`, `Thinking:`, etc are "labels" though they may not carry inline metadata.

Also maybe "AIAnswer.RESULT" is used as `Result Stats: ` with trailing space; exact. Good.

Let me also mention the role mapping in SessionProcessor: when `User:` encountered, role=Role.User; `Agent:` role=Role.Agent. Others consumed as respective sections. This is prefix mapping. I can include in Labels section.

Let me make it concise yet complete.

Format: English, tables where helpful.

Now write final.
Thinking Meta: {"type":"reasoning","id":"5f7fc613-1dde-4880-b897-3dc7481c085a","status":"completed","content":[{"type":"reasoning_text","text":"{{reasoning-text-0}}"}],"summary":[],"encrypted_content":"d9cf89d4-d97b-4851-9166-c147aae515a6-0"}
