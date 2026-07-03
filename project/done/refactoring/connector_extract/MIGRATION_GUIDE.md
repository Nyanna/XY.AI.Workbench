# Migration Guide - ClaudeCodeConnector Refactoring

## Für Entwickler, die den Connector verwenden

### ✅ Keine Änderungen erforderlich!
Die Public API von `ClaudeCodeConnector` ist **vollständig kompatibel** mit der Originalversion.

```java
// Bestehendes Code funktioniert unverändert:
IAIConnector connector = new ClaudeCodeConnector(configManager);

// Alle Standard-Operationen arbeiten wie zuvor:
IModelRequest request = connector.createRequest(input, prompt, tools, false, monitor);
IModelResponse response = connector.executeRequest(request, monitor);
AIAnswer answer = connector.convertResponse(response, monitor);
```

---

## Für Entwickler, die erweitern wollen

### 📋 Szenario 1: Neuen Event-Type hinzufügen

**Problem:** Claude Code API gibt neue Event-Typen aus.

**Lösung:** Erweiter `ClaudeCodeJsonParser`

```java
// Alte Methode (in readUntilResult):
if ("my_event".equals(type)) {
    return parseMyEvent(node);
}

// Neue Methode (in ClaudeCodeJsonParser):
public void collectMyEvents(JsonNode node, LinkedHashMap<String, String> events) {
    JsonNode data = node.path("my_data");
    // ... extract and add to events map
}

// Im Connector:
if ("my_event".equals(type)) {
    jsonParser.collectMyEvents(node, assistantEvents);
}
```

---

### 📋 Szenario 2: CLI-Befehl anpassen

**Problem:** Neue Claude-Code-Flags oder Konfiguration erforderlich.

**Lösung:** Erweitere `ClaudeCodeRequestBuilder.buildCommand()`

```java
public List<String> buildCommand(String profile) {
    List<String> cmd = new ArrayList<>();
    cmd.add(SCRIPT);
    // ... bestehende Args
    
    // Neuer Arg:
    cmd.add("--my-new-flag");
    cmd.add("value");
    
    return cmd;
}
```

---

### 📋 Szenario 3: JSON-Struktur ändern

**Problem:** API erwartet neues Format für Messages.

**Lösung:** Modifizier `ClaudeCodeRequestBuilder` Methoden

```java
public String buildPromptJson(String text) {
    try {
        ObjectNode root = mapper.createObjectNode();
        root.put("type", "user");
        ObjectNode message = root.putObject("message");
        message.put("role", "user");
        
        // Neuer Block:
        ObjectNode metadata = message.putObject("metadata");
        metadata.put("version", "2.0");
        
        ArrayNode content = message.putArray("content");
        ObjectNode textNode = content.addObject();
        textNode.put("type", "text");
        textNode.put("text", text);
        return mapper.writeValueAsString(root);
    } catch (Exception e) {
        throw new IllegalStateException("Failed to build prompt JSON", e);
    }
}
```

---

### 📋 Szenario 4: Bessere Fehlerbehandlung für Events

**Problem:** Parsing-Fehler bei bestimmten Event-Typen.

**Lösung:** Füge Validierung in `ClaudeCodeJsonParser` hinzu

```java
public ClaudeCodeResponse parseResult(String id, JsonNode node,
        LinkedHashMap<String, String> assistantEvents, long totalReasoningTokens) {
    
    // Validiere vor Verarbeitung:
    if (!node.hasNonNull("result")) {
        LOG.error("Result node missing 'result' field");
        throw new IllegalArgumentException("Invalid result structure");
    }
    
    // ... rest der Methode
}
```

---

## Klassen Reference

### ClaudeCodeRequestBuilder

```java
ClaudeCodeRequestBuilder builder = new ClaudeCodeRequestBuilder(mapper, cfg);

// JSON Messages
String json = builder.buildPromptJson("Hallo, Claude!");
String approve = builder.buildApproveJson("tool-id-123");
String deny = builder.buildDenyJson("tool-id-456");

// CLI Command
List<String> cmd = builder.buildCommand("default");
// => ["path/to/claude-session.sh", "agent-name", "--profile", "default", ...]
```

### ClaudeCodeJsonParser

```java
ClaudeCodeJsonParser parser = new ClaudeCodeJsonParser(mapper, postProcessor);
parser.setRecordText(true); // Enable/disable text recording

// Parse Events
ClaudeCodeResponse result = parser.parseResult(id, node, events, tokens);
ClaudeCodeResponse tool = parser.parseToolUse(id, node);

// Collect Events
parser.collectAssistantEvents(node, eventMap);
long tokens = parser.collectMessageDeltaEvent(node, eventMap);

// Process Events
parser.processRateLimitEvent(node);
```

---

## Architektur-Tipps

### ✅ Good: Dependency Injection
```java
// In Unit Tests:
ObjectMapper mockMapper = mock(ObjectMapper.class);
ConfigManager mockCfg = mock(ConfigManager.class);
ClaudeCodeRequestBuilder builder = new ClaudeCodeRequestBuilder(mockMapper, mockCfg);
```

### ✅ Good: Event Processing Pipeline
```java
// Klare Trennung von Bedenken:
// 1. Build Request
IModelRequest req = connector.createRequest(...);

// 2. Execute (Process Management + Stream Reading)
IModelResponse resp = connector.executeRequest(req, ...);

// 3. Convert (Token Aggregation + Formatting)
AIAnswer answer = connector.convertResponse(resp, ...);
```

### ⚠️ Avoid: Direct Instantiation in Tests
```java
// NICHT SO:
JsonNode node = mapper.readTree("{\"type\":\"result\"}");
parser.parseResult(id, node, ...); // Unmockable

// BESSER:
JsonNode mockNode = mock(JsonNode.class);
when(mockNode.path("type")).thenReturn(mock(JsonNode.class));
parser.parseResult(id, mockNode, ...); // Mockable
```

### ⚠️ Avoid: Modifying Shared State
```java
// NICHT SO:
LinkedHashMap<String, String> events = new LinkedHashMap<>();
parser.collectAssistantEvents(node1, events);
parser.collectAssistantEvents(node2, events); // Seiteneffekte!

// BESSER:
LinkedHashMap<String, String> result = new LinkedHashMap<>();
for (JsonNode eventNode : eventStream) {
    parser.collectAssistantEvents(eventNode, result);
}
```

---

## Testing-Checkliste

- [ ] Unit Tests für `buildPromptJson()` (verschiedene Inputs)
- [ ] Unit Tests für `buildCommand()` (Konfiguration mit/ohne Profil)
- [ ] Unit Tests für `parseResult()` (Error/Success-Fälle)
- [ ] Unit Tests für `parseToolUse()` (verschiedene Input-Strukturen)
- [ ] Integration Tests für Gesamtkontrollflusss
- [ ] Performance Tests für große Event-Streams

---

## Breaking Changes: KEINE

| Komponente | Status | Anpassung erforderlich |
|------------|--------|------------------------|
| Public API | ✅ Unverändert | Nein |
| Konstruktor | ✅ Kompatibel | Nein |
| Method Signatures | ✅ Kompatibel | Nein |
| Return Types | ✅ Kompatibel | Nein |

---

## Performance-Implikationen

- ✅ Keine Verschlechterung (gleiche Algorithmen)
- ✅ Bessere CPU-Cache-Nutzung (spezialisierte Klassen)
- ✅ Keine zusätzlichen Allocationen
- ✅ Event-Processing bleibt O(n)

---

## Support

Für Fragen zum neuen Design:
1. Siehe `ARCHITECTURE.md` für Überblick
2. Siehe inline Javadoc in den Utility-Klassen
3. Siehe Test-Cases für Verwendungsbeispiele

---
**Letzte Aktualisierung:** 2026-07-03  
**Kompatibilität:** 100% (Backward Compatible)
