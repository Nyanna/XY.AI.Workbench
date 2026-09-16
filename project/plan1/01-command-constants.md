# Step 01 – Globale Kommando-Konstanten (Phase 0)

**Übersicht:** `/home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md`
**Ziel:** Eine einzige, geteilte Quelle für Kommando-Präfixe etablieren, ohne Verhalten zu ändern.
**Abhängigkeiten:** keine. Grundlage für Steps 03/04/06/08.

## Ist-Zustand
- `EditorInterface.java`, Feldblock **Node `EditorInterface.nZrzm0`** enthält bereits die Marker:
```java
public static final String USER = "User:";
public static final String AGENT = "Agent:";
public static final String TEXT = "Text:";
public static final String THINKING = "Thinking:";
public static final String TOOLUSE = "Tool:";
public static final String TOOLRESULT = "ToolResult:";
```
- `connector/claudecode/CCControlClient.java` (Feldblock ~Z.32/33):
```java
public static final String ANSWER = "/answer";
public static final String CONTROL_REQUEST = "Control Request:";
```
- Verstreute String-Literale: `"/exit"`, `"/resume"` in `CCConnector.preprocessInput`;
  `"/exit"`, `"/tool"` in `MCPConnector.preprocess`.

## Aktionen
1. In `EditorInterface.java` (Node `EditorInterface.nZrzm0`), unterhalb der Marker, einen
   Kommando-Konstantenblock ergänzen (symmetrisch zu den Markern):
```java
/** Command prefixes recognized by connector.CommandHandler / SessionProcessor. */
public static final String CMD_ANSWER = "/answer";
public static final String CMD_RESUME = "/resume";
public static final String CMD_EXIT   = "/exit";
public static final String CMD_TOOL   = "/tool";
public static final String CMD_CALL   = "/call";
public static final String CONTROL_REQUEST = "Control Request:";
```
2. `CCControlClient.ANSWER` und `CCControlClient.CONTROL_REQUEST` auf die zentralen Konstanten
   umbiegen (Kompatibilität behalten – Felder bleiben bestehen, referenzieren aber `EditorInterface`):
```java
public static final String ANSWER = EditorInterface.CMD_ANSWER;
public static final String CONTROL_REQUEST = EditorInterface.CONTROL_REQUEST;
```
   Import für `EditorInterface` in `CCControlClient` sicherstellen.

## Hinweise
- Noch **keine** Umstellung von `CCConnector`/`MCPConnector` hier (erfolgt in Step 03).
- `checkControlEndpoint` in `CCControlClient` nutzt `ANSWER`/`CONTROL_REQUEST` bereits – bleibt funktional.

## Validierung
`ast_validate` für `EditorInterface.java`, `connector/claudecode/CCControlClient.java`.

SystemInit:  id=9870c88d-1e7b-40d8-92db-888817998470, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 7fa5-25
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench
    pattern: public static final String (USER|ANSWER|CONTROL_REQUEST) *=
    include:
    - '*.java'
  reason: Locate EditorInterface constants and CCControlClient fields
```
/answer 7fa5-25 allow
Control Request:
```yaml
id: 7fa5-26
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench
      pattern: public static final String (USER|ANSWER|CONTROL_REQUEST) *=
      matches:
      - path: src/xy/ai/workbench/EditorInterface.java
        lineno: 38
        match: "\tpublic static final String USER = \"User:\";"
      - path: src/xy/ai/workbench/connector/claudecode/CCControlClient.java
        lineno: 32
        match: "\tpublic static final String ANSWER = \"/answer\";"
      - path: src/xy/ai/workbench/connector/claudecode/CCControlClient.java
        lineno: 33
        match: "\tpublic static final String CONTROL_REQUEST = \"Control Request:\";"
```
/answer 7fa5-26 allow
Control Request:
```yaml
id: 7fa5-27
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench
  id: EditorInterface.nZrzm0
  reason: Read the field block node containing marker constants in EditorInterface.java
```
/answer 7fa5-27 allow
Control Request:
```yaml
id: 7fa5-28
result:
  content:
  - type: text
    text: Not a regular file.
  isError: true
```
/answer 7fa5-28 allow
ReasoningToken: 32
Control Request:
```yaml
id: 7fa5-29
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
  id: EditorInterface.nZrzm0
  reason: Read the field block node containing marker constants in EditorInterface.java
```
/answer 7fa5-29 allow
Control Request:
```yaml
id: 7fa5-2a
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
      nodes:
      - id: EditorInterface.nZrzm0
        type: statements
        code: "public static final String USER = \"User:\";\n\tpublic static final String AGENT = \"Agent:\";\n\n\t/** Session block markers recognized by {@code connector.harness.SessionProcessor}. */\n\tpublic static final String TEXT = \"Text:\";\n\tpublic static final String THINKING = \"Thinking:\";\n\tpublic static final String TOOLUSE = \"Tool:\";\n\tpublic static final String TOOLRESULT = \"ToolResult:\";\n\n\tprivate final ActiveEditorListener editorListener;\n\tprivate final AdaptingConnector connector;\n\tprivate final ConfigManager cfg;"
```
/answer 7fa5-2a allow
Control Request:
```yaml
id: 7fa5-2b
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
  lineno: 32
  reason: Locate ANSWER/CONTROL_REQUEST field node and import section for edit
```
/answer 7fa5-2b allow
Control Request:
```yaml
id: 7fa5-2c
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      nodes:
      - id: CCControlClient.hibHfZ
        type: statements
        code: "public static final String ANSWER = \"/answer\";\n\tpublic static final String CONTROL_REQUEST = \"Control Request:\";\n\tprivate static final String CONTROL_URL = \"http://localhost:9093/control/tool\";\n\tprivate static final Duration TIMEOUT = Duration.ofSeconds(5);\n\n\tprivate final ObjectMapper mapper = JsonUtil.mapper();\n\tprivate YamlRenderer yaml = new YamlRenderer();\n\tprivate final HttpClient http = HttpClient.newBuilder().connectTimeout(TIMEOUT).build();"
```
/answer 7fa5-2c allow
Control Request:
```yaml
id: 7fa5-2d
toolName: ast_find
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
  node_type: Import
  reason: Check existing imports for CCControlClient
```
/answer 7fa5-2d allow
Control Request:
```yaml
id: 7fa5-2e
result:
  structuredContent:
    files:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      nodes: []
```
/answer 7fa5-2e allow
ReasoningToken: 29
Control Request:
```yaml
id: 7fa5-2f
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
  reason: Find import statements node id/structure
```
/answer 7fa5-2f allow
Control Request:
```yaml
id: 7fa5-30
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      nodes:
      - id: WHD6Rc
        type: imports
      - id: msdu0B
        type: imports
      - id: AQWMTb
        type: statements
      - id: CCControlClient
        type: class_declaration
        signature: "public class CCControlClient {"
        children:
        - id: CCControlClient.hibHfZ
          type: statements
        - id: CCControlClient.checkControlEndpoint
          type: method_declaration
          signature: "public void checkControlEndpoint(CCResponse resp) {"
        - id: CCControlClient.toYaml
          type: method_declaration
          signature: "public String toYaml(JsonNode node) {"
        - id: CCControlClient.fromYaml
          type: method_declaration
          signature: "public JsonNode fromYaml(String text) throws JsonProcessingException {"
        - id: CCControlClient.isMCPCAvailable
          type: method_declaration
          signature: "public boolean isMCPCAvailable() {"
        - id: CCControlClient.poll
          type: method_declaration
          signature: "private ArrayNode poll() {"
        - id: CCControlClient.approve
          type: method_declaration
          signature: "public ArrayNode approve(String id) {"
        - id: CCControlClient.approve_1
          type: method_declaration
          signature: "public ArrayNode approve(String id, String hint) {"
        - id: CCControlClient.deny
          type: method_declaration
          signature: "public ArrayNode deny(String id, String reason) {"
        - id: CCControlClient.submitEdit
          type: method_declaration
          signature: "public boolean submitEdit(String rawText) {"
        - id: CCControlClient.extractYamlBlock
          type: method_declaration
          signature: "private String extractYamlBlock(String text) {"
        - id: CCControlClient.approvalNode
          type: method_declaration
          signature: "private ObjectNode approvalNode(String id, JsonNode arguments, JsonNode result,…"
        - id: CCControlClient.submit
          type: method_declaration
          signature: "private ArrayNode submit(ObjectNode approval) {"
        - id: CCControlClient.post
          type: method_declaration
          signature: "private ArrayNode post(ObjectNode body) {"
```
/answer 7fa5-30 allow
Control Request:
```yaml
id: 7fa5-31
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    ids:
    - WHD6Rc
    - msdu0B
  reason: See existing import lists to add EditorInterface import correctly
```
/answer 7fa5-31 allow
Control Request:
```yaml
id: 7fa5-32
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
      nodes:
      - id: WHD6Rc
        type: imports
        code: |-
          package xy.ai.workbench.connector.claudecode;

          import java.io.IOException;
          import java.net.URI;
          import java.net.http.HttpClient;
          import java.net.http.HttpRequest;
          import java.net.http.HttpRequest.BodyPublishers;
          import java.net.http.HttpResponse;
          import java.net.http.HttpResponse.BodyHandlers;
          import java.time.Duration;

          import com.fasterxml.jackson.core.JsonProcessingException;
          import com.fasterxml.jackson.databind.JsonNode;
          import com.fasterxml.jackson.databind.ObjectMapper;
      - id: msdu0B
        type: imports
        code: |-
          import com.fasterxml.jackson.databind.node.ArrayNode;
          import com.fasterxml.jackson.databind.node.ObjectNode;

          import xy.ai.workbench.LOG;
```
/answer 7fa5-32 allow
Text:
 EditorInterface.java is in package `xy.ai.workbench`, same as `xy.ai.workbench.LOG` import already present pattern. Now apply edits.
Control Request:
```yaml
id: 7fa5-33
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
    id: EditorInterface.nZrzm0
    old_text: public static final String TOOLRESULT = "ToolResult:";
    new_text: "public static final String TOOLRESULT = \"ToolResult:\";\n\n\t/** Command prefixes recognized by connector.CommandHandler / SessionProcessor. */\n\tpublic static final String CMD_ANSWER = \"/answer\";\n\tpublic static final String CMD_RESUME = \"/resume\";\n\tpublic static final String CMD_EXIT   = \"/exit\";\n\tpublic static final String CMD_TOOL   = \"/tool\";\n\tpublic static final String CMD_CALL   = \"/call\";\n\tpublic static final String CONTROL_REQUEST = \"Control Request:\";"
  reason: Add global command constants block per Step 01 spec
```
/answer 7fa5-33 allow
Control Request:
```yaml
id: 7fa5-34
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    id: CCControlClient.hibHfZ
    old_text: "public static final String ANSWER = \"/answer\";\n\tpublic static final String CONTROL_REQUEST = \"Control Request:\";"
    new_text: "public static final String ANSWER = EditorInterface.CMD_ANSWER;\n\tpublic static final String CONTROL_REQUEST = EditorInterface.CONTROL_REQUEST;"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
    id: msdu0B
    old_text: import xy.ai.workbench.LOG;
    new_text: |-
      import xy.ai.workbench.EditorInterface;
      import xy.ai.workbench.LOG;
  reason: Point ANSWER/CONTROL_REQUEST to central EditorInterface constants and add import
```
/answer 7fa5-34 allow
Control Request:
```yaml
id: 7fa5-35
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claudecode/CCControlClient.java
  reason: Validate both files compile after edits
```
/answer 7fa5-35 allow
Result Stats:  id=9870c88d-1e7b-40d8-92db-888817998470, total: 0, in: 147, out: 2337, reason: 0, read: 61228, write: 11694
Beide Dateien erfolgreich validiert. Step 01 abgeschlossen.
