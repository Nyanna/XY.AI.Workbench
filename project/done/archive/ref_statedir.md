State Location Referenz: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelResolverRegistry.java`

Folgende Implementierungen sollen von Temp File Cache auf State Location umgestellt werden. (7 Tage)
Dev Catalog:
`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelsDevCatalog.java`

Folgende Implementierungen sollen überhaupt einen File Cache des HTTP Results erhalten der 7 Tage lang gültig ist (last mod time).
Provider Resolver:
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/OpenAIModelResolver.java`
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/GeminiModelResolver.java`
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/DeepseekModelResolver.java`
- `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/AnthropicModelResolver.java`
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
SystemInit:  id=97145f6c-677e-4254-b160-b9f618153c93, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 7745-1
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelResolverRegistry.java
    ids:
    - root
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelsDevCatalog.java
    ids:
    - root
  reason: Referenzimplementierung State Location und aktuelle ModelsDevCatalog Implementierung analysieren
```
/answer 7745-1 allow
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: 7745-2
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelResolverRegistry.java
      nodes:
      - id: Y0mULj|t53TUx
        signature: package xy.ai.workbench.model;
      - id: vBDKSw|L385BW
        signature: import xy.ai.workbench.Model.KeyPattern;
      - id: Wmwd0H|DLBZF4
        signature: /**
      - id: ModelResolverRegistry
        signature: "public class ModelResolverRegistry {"
        children:
        - id: ModelResolverRegistry.IrpkcJ|wovak5
          signature: "private static final Map<KeyPattern, ModelResolver> RESOLVERS = new EnumMap<>(K…"
        - id: ModelResolverRegistry.TcMZwo|oktbFV
          signature: "static {"
        - id: ModelResolverRegistry.resolve
          signature: "public static List<Model> resolve(KeyPattern provider, String apiKey) {"
        - id: ModelResolverRegistry.tGk0Hi|ogHRiK
          signature: /** Wires the registry as a connect observer of the given (not yet connected) M…
        - id: ModelResolverRegistry.attach
          signature: "public static void attach(MCPClient client) {"
        - id: ModelResolverRegistry.ensureToolsLoaded
          signature: "private static synchronized void ensureToolsLoaded() {"
        - id: ModelResolverRegistry.onConnect
          signature: "private static void onConnect(MCPClient client) {"
        - id: ModelResolverRegistry.applyTools
          signature: "private static void applyTools(List<Model> models) {"
        - id: ModelResolverRegistry.sortByToolOrder
          signature: "private static String[] sortByToolOrder(Collection<String> names) {"
        - id: ModelResolverRegistry.loadTools
          signature: "private static String[] loadTools() {"
        - id: ModelResolverRegistry.saveTools
          signature: "private static void saveTools(String[] tools) {"
        - id: ModelResolverRegistry.toolsFile
          signature: "private static File toolsFile() {"
      errors:
      - "ids ['root'] resolved to no node; redirected to ast_list, returning the full outline instead."
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelsDevCatalog.java
      nodes:
      - id: W8sm05|x2X9ey
        signature: package xy.ai.workbench.model;
      - id: 1rqlm7|eXDolB
        signature: import com.fasterxml.jackson.databind.ObjectMapper;
      - id: luyP3M|Fp4giu
        signature: /**
      - id: ModelsDevCatalog
        signature: "public final class ModelsDevCatalog {"
        children:
        - id: ModelsDevCatalog.eK0QW3|nw6Q7k
          signature: private static final String URL = "https://models.dev/api.json";
        - id: ModelsDevCatalog.get
          signature: "public static synchronized ModelsDevCatalog get() {"
        - id: ModelsDevCatalog.ModelsDevCatalog
          signature: "private ModelsDevCatalog() {"
        - id: ModelsDevCatalog.load
          signature: "private JsonNode load() {"
        - id: ModelsDevCatalog.readFreshCache
          signature: "private String readFreshCache() {"
        - id: ModelsDevCatalog.fetch
          signature: "private String fetch() throws IOException, InterruptedException {"
        - id: ModelsDevCatalog.fetchH
          signature: "private String fetchH() throws IOException {"
        - id: ModelsDevCatalog.mpEdJf|Fx4VhB
          signature: /**
        - id: ModelsDevCatalog.findModel
          signature: "public JsonNode findModel(String providerId, String modelId) {"
        - id: ModelsDevCatalog.6xRXGi|eHXSE0
          signature: /**
        - id: ModelsDevCatalog.toCapabilities
          signature: "public Capabilities toCapabilities(xy.ai.workbench.Model.KeyPattern provider, J…"
      errors:
      - "ids ['root'] resolved to no node; redirected to ast_list, returning the full outline instead."
```
/answer 7745-2 allow
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 0, in: 0, out: 0, reason: 22, read: 0, write: 0
ReasoningToken: 22
Control Request:
```yaml
id: 7745-3
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelResolverRegistry.java
    ids:
    - ModelResolverRegistry.loadTools
    - ModelResolverRegistry.saveTools
    - ModelResolverRegistry.toolsFile
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelsDevCatalog.java
    ids:
    - ModelsDevCatalog.eK0QW3|nw6Q7k
    - ModelsDevCatalog.load
    - ModelsDevCatalog.readFreshCache
    - ModelsDevCatalog.fetch
    - ModelsDevCatalog.fetchH
    - 1rqlm7|eXDolB
  reason: Vollständigen Code der relevanten Methoden lesen um State Location Pattern und aktuellen Temp-Cache zu verstehen
```
/answer 7745-3 allow
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: 7745-4
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelResolverRegistry.java
      nodes:
      - id: ModelResolverRegistry.loadTools
        type: method_declaration
        code: "private static String[] loadTools() {\n\t\ttry {\n\t\t\tFile file = toolsFile();\n\t\t\tif (!file.exists())\n\t\t\t\treturn null;\n\t\t\tString content = Files.readString(file.toPath());\n\t\t\treturn content.isBlank() ? new String[0] : content.split(\"\\n\");\n\t\t} catch (IOException e) {\n\t\t\tLOG.error(\"Could not load persisted tool list\", e);\n\t\t\treturn null;\n\t\t}\n\t}"
      - id: ModelResolverRegistry.saveTools
        type: method_declaration
        code: "private static void saveTools(String[] tools) {\n\t\ttry {\n\t\t\tFiles.writeString(toolsFile().toPath(), String.join(\"\\n\", tools));\n\t\t} catch (IOException e) {\n\t\t\tLOG.error(\"Could not persist tool list\", e);\n\t\t}\n\t}"
      - id: ModelResolverRegistry.toolsFile
        type: method_declaration
        code: "private static File toolsFile() {\n\t\treturn Activator.getDefault().getStateLocation().append(TOOLS_FILE).toFile();\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelsDevCatalog.java
      nodes:
      - id: ModelsDevCatalog.eK0QW3|nw6Q7k
        type: statements
        code: "private static final String URL = \"https://models.dev/api.json\";\n\tprivate static final Duration MAX_AGE = Duration.ofDays(7);\n\tprivate static final Path CACHE_FILE = Path.of(System.getProperty(\"java.io.tmpdir\"),\n\t\t\t\"xy-ai-workbench-models-dev-cache.json\");\n\n\tprivate static ModelsDevCatalog instance;\n\n\tprivate final JsonNode root;"
      - id: ModelsDevCatalog.load
        type: method_declaration
        code: "private JsonNode load() {\n\t\tObjectMapper om = new ObjectMapper();\n\t\ttry {\n\t\t\tString json = readFreshCache();\n\t\t\tif (json == null) {\n\t\t\t\tjson = fetchH();\n\t\t\t\ttry {\n\t\t\t\t\tFiles.writeString(CACHE_FILE, json, StandardCharsets.UTF_8);\n\t\t\t\t} catch (IOException e) {\n\t\t\t\t\tLOG.error(\"Unable to cache models.dev catalog\", e);\n\t\t\t\t}\n\t\t\t}\n\t\t\treturn om.readTree(json);\n\t\t} catch (Exception e) {\n\t\t\tLOG.error(\"Unable to load models.dev catalog\", e);\n\t\t\treturn om.createObjectNode();\n\t\t}\n\t}"
      - id: ModelsDevCatalog.readFreshCache
        type: method_declaration
        code: "private String readFreshCache() {\n\t\ttry {\n\t\t\tif (!Files.isRegularFile(CACHE_FILE))\n\t\t\t\treturn null;\n\t\t\tInstant modified = Files.getLastModifiedTime(CACHE_FILE).toInstant();\n\t\t\tif (Duration.between(modified, Instant.now()).compareTo(MAX_AGE) > 0)\n\t\t\t\treturn null;\n\t\t\treturn Files.readString(CACHE_FILE, StandardCharsets.UTF_8);\n\t\t} catch (IOException e) {\n\t\t\tLOG.error(\"Unable to read cached models.dev catalog\", e);\n\t\t\treturn null;\n\t\t}\n\t}"
      - id: ModelsDevCatalog.fetch
        type: method_declaration
        code: "@SuppressWarnings(\"unused\")\n\tprivate String fetch() throws IOException, InterruptedException {\n\t\tHttpClient client = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(3)).build();\n\t\tHttpRequest req = HttpRequest.newBuilder(URI.create(URL)).GET().timeout(Duration.ofSeconds(3)).build();\n\t\tHttpResponse<String> resp = client.send(req, HttpResponse.BodyHandlers.ofString());\n\t\tif (resp.statusCode() / 100 != 2)\n\t\t\tthrow new IOException(\"models.dev returned HTTP \" + resp.statusCode());\n\t\treturn resp.body();\n\t}"
      - id: ModelsDevCatalog.fetchH
        type: method_declaration
        code: "private String fetchH() throws IOException {\n\t\tURLConnection connection = URI.create(URL).toURL().openConnection();\n\n\t\tHttpURLConnection http = (HttpURLConnection) connection;\n\t\thttp.setConnectTimeout(3_000);\n\t\thttp.setReadTimeout(3_000);\n\t\thttp.setRequestMethod(\"GET\");\n\n\t\ttry {\n\t\t\tint status = http.getResponseCode();\n\t\t\tif (status / 100 != 2)\n\t\t\t\tthrow new IOException(\"models.dev returned HTTP \" + status);\n\n\t\t\ttry (InputStream in = http.getInputStream()) {\n\t\t\t\treturn new String(in.readAllBytes(), StandardCharsets.UTF_8);\n\t\t\t}\n\t\t} finally {\n\t\t\thttp.disconnect();\n\t\t}\n\t}"
      - id: 1rqlm7|eXDolB
        type: imports
        code: |-
          import com.fasterxml.jackson.databind.ObjectMapper;

          import xy.ai.workbench.Capabilities;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.Reasoning;
```
/answer 7745-4 allow
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: 7745-5
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelResolverRegistry.java
    ids:
    - vBDKSw|L385BW
    - ModelResolverRegistry.IrpkcJ|wovak5
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelsDevCatalog.java
    ids:
    - W8sm05|x2X9ey
  reason: Vollständige Import-Liste und Konstantenblock der Registry sehen für Activator-Import und TOOLS_FILE-Konstante
```
/answer 7745-5 allow
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: 7745-6
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelResolverRegistry.java
      nodes:
      - id: vBDKSw|L385BW
        type: imports
        code: |-
          import xy.ai.workbench.Model.KeyPattern;
          import xy.ai.workbench.Tools;
          import xy.ai.workbench.connector.mcp.MCPClient;
      - id: ModelResolverRegistry.IrpkcJ|wovak5
        type: statements
        code: "private static final Map<KeyPattern, ModelResolver> RESOLVERS = new EnumMap<>(KeyPattern.class);\n\tprivate static final Map<String, List<Model>> CACHE = new ConcurrentHashMap<>();\n\tprivate static final String TOOLS_FILE = \"tools.txt\";\n\n\t/** Cached tool list, lazily loaded from the state location on first resolve(). */\n\tprivate static String[] tools;"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelsDevCatalog.java
      nodes:
      - id: W8sm05|x2X9ey
        type: imports
        code: |-
          package xy.ai.workbench.model;

          import java.io.IOException;
          import java.io.InputStream;
          import java.net.HttpURLConnection;
          import java.net.URI;
          import java.net.URLConnection;
          import java.net.http.HttpClient;
          import java.net.http.HttpRequest;
          import java.net.http.HttpResponse;
          import java.nio.charset.StandardCharsets;
          import java.nio.file.Files;
          import java.nio.file.Path;
          import java.time.Duration;
          import java.time.Instant;

          import com.fasterxml.jackson.databind.JsonNode;
```
/answer 7745-6 allow
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 0, in: 0, out: 0, reason: 46, read: 0, write: 0
ReasoningToken: 46
Control Request:
```yaml
id: 7745-7
toolName: grep
arguments:
  items:
  - directory:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model
    pattern: import xy.ai.workbench.Activator
    include:
    - '*.java'
  reason: "Import-Pfad für Activator ermitteln, um ihn in ModelsDevCatalog und Resolver-Klassen zu verwenden"
```
/answer 7745-7 allow
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: 7745-8
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model
      matches:
      - path: ModelResolverRegistry.java
        matches:
        - lineno: 17
          match: import xy.ai.workbench.Activator;
```
/answer 7745-8 allow
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 0, in: 0, out: 0, reason: 78, read: 0, write: 0
ReasoningToken: 78
Text:
 Jetzt die vier Provider-Resolver anschauen.
Control Request:
```yaml
id: 7745-9
toolName: ast_list
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/OpenAIModelResolver.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/GeminiModelResolver.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/DeepseekModelResolver.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/AnthropicModelResolver.java
  reason: Struktur der vier Provider-Resolver-Klassen ermitteln um HTTP-Fetch-Methoden für Caching zu identifizieren
```
/answer 7745-9 allow
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: 7745-a
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/OpenAIModelResolver.java
      nodes:
      - id: zjt5mC|OOeoQ2
        signature: package xy.ai.workbench.model;
      - id: OpenAIModelResolver
        signature: "public class OpenAIModelResolver implements ModelResolver {"
        children:
        - id: OpenAIModelResolver.resolve
          signature: "public List<Model> resolve(String apiKey) {"
        - id: OpenAIModelResolver.capabilitiesFor
          signature: "private Capabilities capabilitiesFor(String id) {"
        - id: OpenAIModelResolver.fetch
          signature: "private String fetch(String apiKey) throws Exception {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/GeminiModelResolver.java
      nodes:
      - id: zjt5mC|OOeoQ2
        signature: package xy.ai.workbench.model;
      - id: cdWHVr|jjUMOA
        signature: import xy.ai.workbench.Reasoning;
      - id: GeminiModelResolver
        signature: "public class GeminiModelResolver implements ModelResolver {"
        children:
        - id: GeminiModelResolver.resolve
          signature: "public List<Model> resolve(String apiKey) {"
        - id: GeminiModelResolver.capabilitiesFor
          signature: "private Capabilities capabilitiesFor(JsonNode entry) {"
        - id: GeminiModelResolver.fetch
          signature: "private String fetch(String apiKey) throws Exception {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/DeepseekModelResolver.java
      nodes:
      - id: zjt5mC|OOeoQ2
        signature: package xy.ai.workbench.model;
      - id: cdWHVr|jjUMOA
        signature: import xy.ai.workbench.Reasoning;
      - id: CaCZYQ|shFigb
        signature: /**
      - id: DeepseekModelResolver
        signature: "public class DeepseekModelResolver implements ModelResolver {"
        children:
        - id: DeepseekModelResolver.resolve
          signature: "public List<Model> resolve(String apiKey) {"
        - id: DeepseekModelResolver.fetch
          signature: "private String fetch(String apiKey) throws Exception {"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/AnthropicModelResolver.java
      nodes:
      - id: zjt5mC|OOeoQ2
        signature: package xy.ai.workbench.model;
      - id: cdWHVr|jjUMOA
        signature: import xy.ai.workbench.Reasoning;
      - id: Gn1fcV|StYD98
        signature: /** Resolves mainstream models currently available for an Anthropic API key. */
      - id: AnthropicModelResolver
        signature: "public class AnthropicModelResolver implements ModelResolver {"
        children:
        - id: AnthropicModelResolver.resolve
          signature: "public List<Model> resolve(String apiKey) {"
        - id: AnthropicModelResolver.capabilitiesFor
          signature: "private Capabilities capabilitiesFor(JsonNode entry) {"
        - id: AnthropicModelResolver.fetch
          signature: "private String fetch(String apiKey) throws Exception {"
```
/answer 7745-a allow
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: 7745-b
toolName: ast_read
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/OpenAIModelResolver.java
    ids:
    - zjt5mC|OOeoQ2
    - OpenAIModelResolver.resolve
    - OpenAIModelResolver.fetch
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/GeminiModelResolver.java
    ids:
    - zjt5mC|OOeoQ2
    - GeminiModelResolver.resolve
    - GeminiModelResolver.fetch
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/DeepseekModelResolver.java
    ids:
    - zjt5mC|OOeoQ2
    - DeepseekModelResolver.resolve
    - DeepseekModelResolver.fetch
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/AnthropicModelResolver.java
    ids:
    - zjt5mC|OOeoQ2
    - AnthropicModelResolver.resolve
    - AnthropicModelResolver.fetch
  reason: Vollständigen Code der resolve/fetch Methoden und Imports der vier Provider-Resolver lesen für Cache-Integration
```
/answer 7745-b allow
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: 7745-c
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/OpenAIModelResolver.java
      nodes:
      - id: zjt5mC|OOeoQ2
        type: imports
        code: |-
          package xy.ai.workbench.model;

          import java.net.URI;
          import java.net.http.HttpClient;
          import java.net.http.HttpRequest;
          import java.net.http.HttpResponse;
          import java.time.Duration;
          import java.util.ArrayList;
          import java.util.Arrays;
          import java.util.List;

          import com.fasterxml.jackson.databind.JsonNode;
          import com.fasterxml.jackson.databind.ObjectMapper;

          import xy.ai.workbench.Capabilities;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.Model;
          import xy.ai.workbench.Model.KeyPattern;
      - id: OpenAIModelResolver.resolve
        type: method_declaration
        code: "@Override\n\tpublic List<Model> resolve(String apiKey) {\n\t\ttry {\n\t\t\tString body = fetch(apiKey);\n\t\t\tObjectMapper om = new ObjectMapper();\n\t\t\tJsonNode data = om.readTree(body).path(\"data\");\n\t\t\tList<Model> result = new ArrayList<>();\n\t\t\tfor (JsonNode entry : data) {\n\t\t\t\tString id = entry.path(\"id\").asText(null);\n\t\t\t\tJsonNode shutdown = entry.path(\"shutdown_date\");\n\t\t\t\tboolean hasShutdownDate = !shutdown.isMissingNode() && !shutdown.isNull();\n\t\t\t\tif (id == null || hasShutdownDate)\n\t\t\t\t\tcontinue;\n\t\t\t\tif (!ModelFilter.isMainstream(id))\n\t\t\t\t\tcontinue;\n\t\t\t\tresult.add(new Model(id, id, capabilitiesFor(id)));\n\t\t\t}\n\t\t\tif (result.isEmpty()) {\n\t\t\t\tLOG.error(\"OpenAI model list was empty, falling back to defaults\");\n\t\t\t\treturn Arrays.asList(Model.defaultsFor(KeyPattern.OpenAI));\n\t\t\t}\n\t\t\treturn result;\n\t\t} catch (Exception e) {\n\t\t\tLOG.error(\"Unable to resolve OpenAI models, falling back to defaults\", e);\n\t\t\treturn Arrays.asList(Model.defaultsFor(KeyPattern.OpenAI));\n\t\t}\n\t}"
      - id: OpenAIModelResolver.fetch
        type: method_declaration
        code: "private String fetch(String apiKey) throws Exception {\n\t\tHttpClient client = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(10)).build();\n\t\tHttpRequest req = HttpRequest.newBuilder(URI.create(\"https://api.openai.com/v1/models\"))\n\t\t\t\t.header(\"Authorization\", \"Bearer \" + apiKey).timeout(Duration.ofSeconds(20)).GET().build();\n\t\tHttpResponse<String> resp = client.send(req, HttpResponse.BodyHandlers.ofString());\n\t\tif (resp.statusCode() / 100 != 2)\n\t\t\tthrow new java.io.IOException(\"OpenAI /v1/models returned HTTP \" + resp.statusCode());\n\t\treturn resp.body();\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/GeminiModelResolver.java
      nodes:
      - id: zjt5mC|OOeoQ2
        type: imports
        code: |-
          package xy.ai.workbench.model;

          import java.net.URI;
          import java.net.http.HttpClient;
          import java.net.http.HttpRequest;
          import java.net.http.HttpResponse;
          import java.time.Duration;
          import java.util.ArrayList;
          import java.util.Arrays;
          import java.util.List;

          import com.fasterxml.jackson.databind.JsonNode;
          import com.fasterxml.jackson.databind.ObjectMapper;

          import xy.ai.workbench.Capabilities;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.Model;
          import xy.ai.workbench.Model.KeyPattern;
      - id: GeminiModelResolver.resolve
        type: method_declaration
        code: "@Override\n\tpublic List<Model> resolve(String apiKey) {\n\t\ttry {\n\t\t\tString body = fetch(apiKey);\n\t\t\tObjectMapper om = new ObjectMapper();\n\t\t\tJsonNode models = om.readTree(body).path(\"models\");\n\t\t\tList<Model> result = new ArrayList<>();\n\t\t\tfor (JsonNode entry : models) {\n\t\t\t\tboolean supportsGenerate = false;\n\t\t\t\tfor (JsonNode method : entry.path(\"supportedGenerationMethods\"))\n\t\t\t\t\tif (\"generateContent\".equals(method.asText()))\n\t\t\t\t\t\tsupportsGenerate = true;\n\t\t\t\tif (!supportsGenerate)\n\t\t\t\t\tcontinue;\n\t\t\t\tString id = entry.path(\"baseModelId\").asText(null);\n\t\t\t\tif (id == null || id.isBlank())\n\t\t\t\t\tid = entry.path(\"name\").asText(\"\").replace(\"models/\", \"\");\n\t\t\t\tif (id.isBlank() || !ModelFilter.isMainstream(id))\n\t\t\t\t\tcontinue;\n\t\t\t\tString displayName = entry.path(\"displayName\").asText(id);\n\t\t\t\tresult.add(new Model(id, displayName, capabilitiesFor(entry)));\n\t\t\t}\n\t\t\tif (result.isEmpty()) {\n\t\t\t\tLOG.error(\"Gemini model list was empty, falling back to defaults\");\n\t\t\t\treturn Arrays.asList(Model.defaultsFor(KeyPattern.Gemini));\n\t\t\t}\n\t\t\treturn result;\n\t\t} catch (Exception e) {\n\t\t\tLOG.error(\"Unable to resolve Gemini models, falling back to defaults\", e);\n\t\t\treturn Arrays.asList(Model.defaultsFor(KeyPattern.Gemini));\n\t\t}\n\t}"
      - id: GeminiModelResolver.fetch
        type: method_declaration
        code: "private String fetch(String apiKey) throws Exception {\n\t\tHttpClient client = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(10)).build();\n\t\tHttpRequest req = HttpRequest\n\t\t\t\t.newBuilder(URI.create(\"https://generativelanguage.googleapis.com/v1beta/models?pageSize=1000\"))\n\t\t\t\t.header(\"x-goog-api-key\", apiKey).timeout(Duration.ofSeconds(20)).GET().build();\n\t\tHttpResponse<String> resp = client.send(req, HttpResponse.BodyHandlers.ofString());\n\t\tif (resp.statusCode() / 100 != 2)\n\t\t\tthrow new java.io.IOException(\"Gemini /v1beta/models returned HTTP \" + resp.statusCode());\n\t\treturn resp.body();\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/DeepseekModelResolver.java
      nodes:
      - id: zjt5mC|OOeoQ2
        type: imports
        code: |-
          package xy.ai.workbench.model;

          import java.net.URI;
          import java.net.http.HttpClient;
          import java.net.http.HttpRequest;
          import java.net.http.HttpResponse;
          import java.time.Duration;
          import java.util.ArrayList;
          import java.util.Arrays;
          import java.util.List;

          import com.fasterxml.jackson.databind.JsonNode;
          import com.fasterxml.jackson.databind.ObjectMapper;

          import xy.ai.workbench.Capabilities;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.Model;
          import xy.ai.workbench.Model.KeyPattern;
      - id: DeepseekModelResolver.resolve
        type: method_declaration
        code: "@Override\n\tpublic List<Model> resolve(String apiKey) {\n\t\ttry {\n\t\t\tString body = fetch(apiKey);\n\t\t\tObjectMapper om = new ObjectMapper();\n\t\t\tJsonNode data = om.readTree(body).path(\"data\");\n\t\t\tList<Model> result = new ArrayList<>();\n\t\t\tfor (JsonNode entry : data) {\n\t\t\t\tString id = entry.path(\"id\").asText(null);\n\t\t\t\tif (id == null || !ModelFilter.isMainstream(id))\n\t\t\t\t\tcontinue;\n\t\t\t\tJsonNode node = ModelsDevCatalog.get().findModel(\"deepseek\", id);\n\t\t\t\tCapabilities cap = ModelsDevCatalog.get().toCapabilities(KeyPattern.Deepseek, node);\n\t\t\t\tcap.reasonings(Reasoning.Disabled, Reasoning.low, Reasoning.high, Reasoning.max);\n\t\t\t\tcap.supportBatch(false);\n\t\t\t\tcap.topPMin(0.95d);\n\t\t\t\tcap.supportTemperature(true);\n\t\t\t\tcap.supportReasoningTopP(true);\n\t\t\t\tresult.add(new Model(id, id, cap));\n\t\t\t}\n\t\t\tif (result.isEmpty())\n\t\t\t\tLOG.error(\"Deepseek model list was empty\");\n\t\t\treturn result;\n\t\t} catch (Exception e) {\n\t\t\tLOG.error(\"Unable to resolve Deepseek models\", e);\n\t\t\treturn Arrays.asList(Model.defaultsFor(KeyPattern.Deepseek));\n\t\t}\n\t}"
      - id: DeepseekModelResolver.fetch
        type: method_declaration
        code: "private String fetch(String apiKey) throws Exception {\n\t\tHttpClient client = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(10)).build();\n\t\tHttpRequest req = HttpRequest.newBuilder(URI.create(\"https://api.deepseek.com/models\"))\n\t\t\t\t.header(\"Accept\", \"application/json\").header(\"Authorization\", \"Bearer \" + apiKey)\n\t\t\t\t.timeout(Duration.ofSeconds(20)).GET().build();\n\t\tHttpResponse<String> resp = client.send(req, HttpResponse.BodyHandlers.ofString());\n\t\tif (resp.statusCode() / 100 != 2)\n\t\t\tthrow new java.io.IOException(\"Deepseek /models returned HTTP \" + resp.statusCode());\n\t\treturn resp.body();\n\t}"
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/AnthropicModelResolver.java
      nodes:
      - id: zjt5mC|OOeoQ2
        type: imports
        code: |-
          package xy.ai.workbench.model;

          import java.net.URI;
          import java.net.http.HttpClient;
          import java.net.http.HttpRequest;
          import java.net.http.HttpResponse;
          import java.time.Duration;
          import java.util.ArrayList;
          import java.util.Arrays;
          import java.util.List;

          import com.fasterxml.jackson.databind.JsonNode;
          import com.fasterxml.jackson.databind.ObjectMapper;

          import xy.ai.workbench.Capabilities;
          import xy.ai.workbench.LOG;
          import xy.ai.workbench.Model;
          import xy.ai.workbench.Model.KeyPattern;
      - id: AnthropicModelResolver.resolve
        type: method_declaration
        code: "@Override\n\tpublic List<Model> resolve(String apiKey) {\n\t\ttry {\n\t\t\tString body = fetch(apiKey);\n\t\t\tObjectMapper om = new ObjectMapper();\n\t\t\tJsonNode data = om.readTree(body).path(\"data\");\n\t\t\tList<Model> result = new ArrayList<>();\n\t\t\tfor (JsonNode entry : data) {\n\t\t\t\tString id = entry.path(\"id\").asText(null);\n\t\t\t\tif (id == null || !ModelFilter.isMainstream(id))\n\t\t\t\t\tcontinue;\n\t\t\t\tString displayName = entry.path(\"display_name\").asText(id);\n\t\t\t\tresult.add(new Model(id, displayName, capabilitiesFor(entry)));\n\t\t\t}\n\t\t\tif (result.isEmpty()) {\n\t\t\t\tLOG.error(\"Anthropic model list was empty, falling back to defaults\");\n\t\t\t\treturn Arrays.asList(Model.defaultsFor(KeyPattern.Claude));\n\t\t\t}\n\t\t\treturn result;\n\t\t} catch (Exception e) {\n\t\t\tLOG.error(\"Unable to resolve Anthropic models, falling back to defaults\", e);\n\t\t\treturn Arrays.asList(Model.defaultsFor(KeyPattern.Claude));\n\t\t}\n\t}"
      - id: AnthropicModelResolver.fetch
        type: method_declaration
        code: "private String fetch(String apiKey) throws Exception {\n\t\tHttpClient client = HttpClient.newBuilder().connectTimeout(Duration.ofSeconds(10)).build();\n\t\tHttpRequest req = HttpRequest.newBuilder(URI.create(\"https://api.anthropic.com/v1/models?limit=200\"))\n\t\t\t\t.header(\"x-api-key\", apiKey).header(\"anthropic-version\", \"2023-06-01\").timeout(Duration.ofSeconds(20))\n\t\t\t\t.GET().build();\n\t\tHttpResponse<String> resp = client.send(req, HttpResponse.BodyHandlers.ofString());\n\t\tif (resp.statusCode() / 100 != 2)\n\t\t\tthrow new java.io.IOException(\"Anthropic /v1/models returned HTTP \" + resp.statusCode());\n\t\treturn resp.body();\n\t}"
```
/answer 7745-c allow
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Text:
 Ich erstelle zunächst eine gemeinsame Cache-Hilfsklasse und passe dann alle betroffenen Dateien an.
Control Request:
```yaml
id: 7745-d
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelHttpCache.java
    source: "package xy.ai.workbench.model;\n\nimport java.io.IOException;\nimport java.nio.charset.StandardCharsets;\nimport java.nio.file.Files;\nimport java.nio.file.Path;\nimport java.time.Duration;\nimport java.time.Instant;\n\nimport xy.ai.workbench.Activator;\nimport xy.ai.workbench.LOG;\n\n/** File cache for provider HTTP model-list responses, valid for 7 days based on the cache file's last-modified time. */\nfinal class ModelHttpCache {\n\n\tprivate static final Duration MAX_AGE = Duration.ofDays(7);\n\n\tprivate ModelHttpCache() {\n\t}\n\n\tinterface Fetcher {\n\t\tString fetch() throws Exception;\n\t}\n\n\t/** Returns the cached body for {@code fileName} if still fresh, otherwise fetches, caches and returns it. */\n\tstatic String get(String fileName, Fetcher fetcher) throws Exception {\n\t\tPath file = Activator.getDefault().getStateLocation().append(fileName).toFile().toPath();\n\t\tString cached = readFresh(file);\n\t\tif (cached != null)\n\t\t\treturn cached;\n\t\tString body = fetcher.fetch();\n\t\ttry {\n\t\t\tFiles.writeString(file, body, StandardCharsets.UTF_8);\n\t\t} catch (IOException e) {\n\t\t\tLOG.error(\"Unable to cache \" + fileName, e);\n\t\t}\n\t\treturn body;\n\t}\n\n\tprivate static String readFresh(Path file) {\n\t\ttry {\n\t\t\tif (!Files.isRegularFile(file))\n\t\t\t\treturn null;\n\t\t\tInstant modified = Files.getLastModifiedTime(file).toInstant();\n\t\t\tif (Duration.between(modified, Instant.now()).compareTo(MAX_AGE) > 0)\n\t\t\t\treturn null;\n\t\t\treturn Files.readString(file, StandardCharsets.UTF_8);\n\t\t} catch (IOException e) {\n\t\t\tLOG.error(\"Unable to read cache \" + file, e);\n\t\t\treturn null;\n\t\t}\n\t}\n}\n"
  reason: "Gemeinsame Cache-Utility-Klasse für die vier Provider-Resolver erstellen (State Location, 7 Tage Gültigkeit)"
```
/answer 7745-d allow
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: 7745-e
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelsDevCatalog.java
    old_text: "private static final String URL = \"https://models.dev/api.json\";\n\tprivate static final Duration MAX_AGE = Duration.ofDays(7);\n\tprivate static final Path CACHE_FILE = Path.of(System.getProperty(\"java.io.tmpdir\"),\n\t\t\t\"xy-ai-workbench-models-dev-cache.json\");"
    new_text: "private static final String URL = \"https://models.dev/api.json\";\n\tprivate static final Duration MAX_AGE = Duration.ofDays(7);\n\tprivate static final Path CACHE_FILE = Activator.getDefault().getStateLocation()\n\t\t\t.append(\"models-dev-cache.json\").toFile().toPath();"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelsDevCatalog.java
    old_text: import com.fasterxml.jackson.databind.JsonNode;
    new_text: |-
      import com.fasterxml.jackson.databind.JsonNode;

      import xy.ai.workbench.Activator;
  reason: Cache-Datei von tmpdir auf Eclipse State Location umstellen wie in ModelResolverRegistry
```
/answer 7745-e allow
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 0, in: 0, out: 0, reason: 954, read: 0, write: 0
ReasoningToken: 954
Text:
 Jetzt ModelsDevCatalog auf State Location umstellen und die vier Resolver mit dem neuen Cache verknüpfen.
Control Request:
```yaml
id: 7745-f
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/OpenAIModelResolver.java
    old_text: "try {\n\t\t\tString body = fetch(apiKey);\n\t\t\tObjectMapper om = new ObjectMapper();"
    new_text: "try {\n\t\t\tString body = ModelHttpCache.get(\"openai-models-cache.json\", () -> fetch(apiKey));\n\t\t\tObjectMapper om = new ObjectMapper();"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/GeminiModelResolver.java
    old_text: "try {\n\t\t\tString body = fetch(apiKey);\n\t\t\tObjectMapper om = new ObjectMapper();"
    new_text: "try {\n\t\t\tString body = ModelHttpCache.get(\"gemini-models-cache.json\", () -> fetch(apiKey));\n\t\t\tObjectMapper om = new ObjectMapper();"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/DeepseekModelResolver.java
    old_text: "try {\n\t\t\tString body = fetch(apiKey);\n\t\t\tObjectMapper om = new ObjectMapper();"
    new_text: "try {\n\t\t\tString body = ModelHttpCache.get(\"deepseek-models-cache.json\", () -> fetch(apiKey));\n\t\t\tObjectMapper om = new ObjectMapper();"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/AnthropicModelResolver.java
    old_text: "try {\n\t\t\tString body = fetch(apiKey);\n\t\t\tObjectMapper om = new ObjectMapper();"
    new_text: "try {\n\t\t\tString body = ModelHttpCache.get(\"anthropic-models-cache.json\", () -> fetch(apiKey));\n\t\t\tObjectMapper om = new ObjectMapper();"
  reason: HTTP-Fetch in den vier Provider-Resolvern über den neuen 7-Tage-State-Location-Cache leiten
```
/answer 7745-f allow
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 0, in: 0, out: 0, reason: 0, read: 0, write: 0
Control Request:
```yaml
id: 7745-10
toolName: ast_validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelHttpCache.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/ModelsDevCatalog.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/OpenAIModelResolver.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/GeminiModelResolver.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/DeepseekModelResolver.java
  - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/model/AnthropicModelResolver.java
  reason: Alle geänderten/neuen Dateien auf Kompilierbarkeit prüfen
```
/answer 7745-10 allow
Result Stats: id=97145f6c-677e-4254-b160-b9f618153c93, total: 31001, in: 153, out: 6411, reason: 0, read: 147439, write: 24437
Text:
 Nun die vier Provider-Resolver mit ModelHttpCache versehen.
Text:
 Jetzt validieren.
Alle Dateien kompilieren erfolgreich. Zusammenfassung der Änderungen:

- **`ModelHttpCache.java`** (neu): Generischer 7-Tage-Cache basierend auf der State Location, keyed by Dateiname.
- **`ModelsDevCatalog.java`**: `CACHE_FILE` von `java.io.tmpdir` auf `Activator.getDefault().getStateLocation()` umgestellt.
- **`OpenAIModelResolver`, `GeminiModelResolver`, `DeepseekModelResolver`, `AnthropicModelResolver`**: `fetch(apiKey)`-Aufrufe über `ModelHttpCache.get(...)` geleitet, wodurch HTTP-Ergebnisse nun 7 Tage in der State Location gecacht werden.
