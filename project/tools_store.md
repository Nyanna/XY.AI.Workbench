Die Tool Discovery soll verbessert werde. Aktuelle werden die Tools eines Modells im `AISessionManager` lazy bei Auswahl eines Modells gesetzt. Das soll entfernt werden.
Die `ModelResolverRegistry` soll in Zukunft beim Modell Resolving die Tools aller Modelle setzen. Der MCP-Client ist nicht initial verfügbar, daher soll die Registry selbst eine Version der Liste halten.
Die Toolliste soll bei der ersten Modellauflösung geladen werden und nur nach einem Update gespeichert werden auf Basis der StateLocation.
Der MCPClient soll einen Connect Observer erhalten den die ModelResolverRegistry verwendet. Bei der ersten Verbindung des MCPClient soll der Observer benachrichtigt werden. Darauf holt die Registry die aktuelle Toolliste, aktualisiert alle Modell und speichert die Liste, jedoch nur, wenn es Änderungen zur internen Liste gibt.

- Session Manager: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java`
- Activator: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java`
- Registry: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelResolverRegistry.java`
- MCPClient: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPClient.java`

Beispielcode:

```java
import java.io.File;
import java.io.IOException;
import java.nio.file.Files;

import org.eclipse.core.runtime.IPath;
import org.eclipse.core.runtime.Plugin;
import org.osgi.framework.BundleContext;

public class Activator extends Plugin {

    public static final String PLUGIN_ID = "com.example.myplugin";

    private static Activator plugin;

    @Override
    public void start(BundleContext context) throws Exception {
        super.start(context);
        plugin = this;
    }

    @Override
    public void stop(BundleContext context) throws Exception {
        plugin = null;
        super.stop(context);
    }

    public static Activator getDefault() {
        return plugin;
    }

    public void saveData(String content) throws IOException {
        IPath stateLocation = getStateLocation();

        File file = stateLocation
                .append("data.txt")
                .toFile();

        Files.writeString(file.toPath(), content);
    }

    public String loadData() throws IOException {
        File file = getStateLocation()
                .append("data.txt")
                .toFile();

        if (!file.exists()) {
            return null;
        }

        return Files.readString(file.toPath());
    }
}
```
Result Stats: id=1051923d-c52d-4f28-a4d5-95754fa98340, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
SystemInit:  id=1051923d-c52d-4f28-a4d5-95754fa98340, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: d348-1
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelResolverRegistry.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPClient.java
  reason: "Aktuelle Struktur der Dateien verstehen, um Tool Discovery umzubauen"
```
/answer d348-1 allow
Result Stats: id=1051923d-c52d-4f28-a4d5-95754fa98340, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: d348-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
      nodes:
      - id: DSKSeM|LspTQI
        signature: package xy.ai.workbench;
      - id: AISessionManager
        signature: "public class AISessionManager {"
        children:
        - id: AISessionManager.KRUHNH|Z9WnNd
          signature: public static final String CONTEXT_PROMPT_TXT = "context.prompt.txt";
        - id: AISessionManager.AISessionManager
          signature: "public AISessionManager(ConfigManager cfg, AdaptingConnector connector, AIBatch…"
        - id: AISessionManager.discoverTools
          signature: "private void discoverTools(Model model) {"
        - id: AISessionManager.sortByToolOrder
          signature: "private String[] sortByToolOrder(Collection<String> names) {"
        - id: AISessionManager.getPromptHandler
          signature: "public PromptHandler getPromptHandler() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/Activator.java
      nodes:
      - id: DSKSeM|wOqaoM
        signature: package xy.ai.workbench;
      - id: 7ZHimu|1FfhNT
        signature: import xy.ai.workbench.marker.MarkerRessourceScanner;
      - id: ndBDf2|sMGxXy
        signature: /**
      - id: Activator
        signature: "public class Activator extends AbstractUIPlugin {"
        children:
        - id: Activator.bcZCIU|H6kbFx
          signature: // The plug-in ID
        - id: Activator.kob2Df|i2FR3t
          signature: public AIBatchManager batch = new AIBatchManager(connector);
        - id: Activator.start
          signature: "public void start(BundleContext context) throws Exception {"
        - id: Activator.stop
          signature: "public void stop(BundleContext context) throws Exception {"
        - id: Activator.Cch09r|QOclzD
          signature: /**
        - id: Activator.getDefault
          signature: "public static Activator getDefault() {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelResolverRegistry.java
      nodes:
      - id: zjt5mC|T6pTDg
        signature: package xy.ai.workbench.model;
      - id: Wmwd0H|DLBZF4
        signature: /**
      - id: ModelResolverRegistry
        signature: "public class ModelResolverRegistry {"
        children:
        - id: ModelResolverRegistry.zDOPAW|pA9zkw
          signature: "private static final Map<KeyPattern, ModelResolver> RESOLVERS = new EnumMap<>(K…"
        - id: ModelResolverRegistry.TcMZwo|oktbFV
          signature: "static {"
        - id: ModelResolverRegistry.resolve
          signature: "public static List<Model> resolve(KeyPattern provider, String apiKey) {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/mcp/MCPClient.java
      nodes:
      - id: EdXLLM|DLA8pY
        signature: package xy.ai.workbench.connector.mcp;
      - id: rBrJSJ|Knoc5D
        signature: import com.fasterxml.jackson.databind.node.ArrayNode;
      - id: xa1GeE|d9koxH
        signature: /**
      - id: MCPClient
        signature: "public class MCPClient {"
        children:
        - id: MCPClient.62SeLa|J8XhWN
          signature: private static final String SERVER_URL = "http://localhost:9093/mcp";
        - id: MCPClient.c5fgij|UujMEH
          signature: private final String sessionId = UUID.randomUUID().toString();
        - id: MCPClient.findTool
          signature: "public JsonNode findTool(String name) {"
        - id: MCPClient.listTools
          signature: "public synchronized ArrayNode listTools() {"
        - id: MCPClient.dyZrFr|yGC9tV
          signature: /**
        - id: MCPClient.renderToolCall
          signature: "public String renderToolCall(String name, JsonNode arguments) {"
        - id: MCPClient.callTool
          signature: "public synchronized JsonNode callTool(String name, JsonNode arguments, JsonNode…"
        - id: MCPClient.unwrapPayload
          signature: "private JsonNode unwrapPayload(JsonNode result) {"
        - id: MCPClient.withId
          signature: "private JsonNode withId(JsonNode payload, JsonNode id) {"
        - id: MCPClient.extractErrorText
          signature: "private String extractErrorText(JsonNode result) {"
        - id: MCPClient.close
          signature: "public synchronized void close() {"
        - id: MCPClient.ensureConnected
          signature: "private void ensureConnected() {"
        - id: MCPClient.send
          signature: "private JsonNode send(ObjectNode rpc, boolean expectResult) {"
        - id: MCPClient.parseMessage
          signature: private JsonNode parseMessage(HttpResponse<String> response) throws JsonProcess…
        - id: MCPClient.pickResponse
          signature: "private JsonNode pickResponse(String json, JsonNode current) {"
        - id: MCPClient.request
          signature: "private ObjectNode request(String method, JsonNode params) {"
        - id: MCPClient.notification
          signature: "private ObjectNode notification(String method) {"
```
/answer d348-2 allow
[xy.ai.req:ClaudeCode:7b8014bc-7362-47b4-9482-f4d12dc72bd1]