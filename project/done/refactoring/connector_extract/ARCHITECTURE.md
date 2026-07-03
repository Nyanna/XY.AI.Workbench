# Claude Code Connector - Refactored Architecture

## Klassenstruktur nach Refaktorierung

```
┌─────────────────────────────────────────────────────────────────┐
│                    IAIConnector (Interface)                      │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           │ implements
                           ▼
┌──────────────────────────────────────────────────────────────────┐
│              ClaudeCodeConnector (Core Orchestrator)             │
│  ─────────────────────────────────────────────────────────────  │
│  Responsibilities:                                               │
│  • Process lifecycle management (start/stop)                    │
│  • Request/Response orchestration                               │
│  • Input preprocessing                                          │
│  • Stream reading and event dispatching                         │
│  ─────────────────────────────────────────────────────────────  │
│  + createRequest(String, String, List, boolean, IProgressMon)  │
│  + executeRequest(IModelRequest, IProgressMonitor)             │
│  + convertResponse(IModelResponse, IProgressMonitor)           │
│  - ensureProcess(ClaudeCodeRequest)                            │
│  - terminateProcess()                                           │
│  - readUntilResult(ClaudeCodeRequest)                          │
│  - preprocessInput(String, List, boolean[])                    │
└──────────────────────────────────────────────────────────────────┘
           │                                      │
           │ uses                                 │ uses
           ▼                                      ▼
    ┌──────────────────────┐            ┌──────────────────────┐
    │ ClaudeCodeRequest    │            │ ClaudeCodeResponse   │
    │ Builder              │            │                      │
    │ ────────────────────│            │ Properties:          │
    │ • JSON construction  │            │ • id: String         │
    │ • CLI command build  │            │ • resultText: String │
    │ • Arg preparation    │            │ • isError: boolean   │
    │ ────────────────────│            │ • toolUseId: String  │
    │ + buildPromptJson()  │            │ • tokenData          │
    │ + buildApproveJson() │            │ • isExited: boolean  │
    │ + buildDenyJson()    │            └──────────────────────┘
    │ + buildCommand()     │
    └──────────────────────┘
           △                                      △
           │ injects                              │ uses
           │                                      │
    ┌──────────────────────────────────────────────────────────────┐
    │         ClaudeCodeRequestBuilder (Build Helper)              │
    │  ─────────────────────────────────────────────────────────  │
    │  Responsibilities:                                           │
    │  • JSON structure construction                               │
    │  • CLI command composition                                   │
    │  • Request message building                                  │
    │  ─────────────────────────────────────────────────────────  │
    │  - mapper: ObjectMapper                                      │
    │  - cfg: ConfigManager                                        │
    │  ─────────────────────────────────────────────────────────  │
    │  + buildPromptJson(String): String                           │
    │  + buildApproveJson(String): String                          │
    │  + buildDenyJson(String): String                             │
    │  + buildCommand(String): List<String>                        │
    └──────────────────────────────────────────────────────────────┘
                                 △
                                 │
                                 │ injects
                                 │
┌──────────────────────────────────────────────────────────────────┐
│        ClaudeCodeJsonParser (Parse & Event Handler)              │
│  ─────────────────────────────────────────────────────────────  │
│  Responsibilities:                                               │
│  • Event type parsing                                            │
│  • Token counting and aggregation                                │
│  • Rate limit processing                                         │
│  • Content extraction from responses                             │
│  ─────────────────────────────────────────────────────────────  │
│  - mapper: ObjectMapper                                          │
│  - resultPostProcessor: ResultPostProcessor                      │
│  - recordText: boolean                                           │
│  ─────────────────────────────────────────────────────────────  │
│  + parseResult(...): ClaudeCodeResponse                          │
│  + parseToolUse(...): ClaudeCodeResponse                         │
│  + collectAssistantEvents(...): void                             │
│  + collectMessageDeltaEvent(...): long                           │
│  + processRateLimitEvent(...): void                              │
│  - commented(String): String                                     │
└──────────────────────────────────────────────────────────────────┘
           │
           │ delegates to
           ▼
    ┌──────────────────────┐
    │ ResultPostProcessor  │
    │ (External Utility)   │
    │ ────────────────────│
    │ Processes result     │
    │ text transformations │
    └──────────────────────┘
```

## Data Flow

### Request Path
```
Input String
    │
    ├─→ preprocessInput() ──→ Extract /allow, /deny, /exit commands
    │                         ▼
    │                    requestBuilder.buildApproveJson()
    │                    requestBuilder.buildDenyJson()
    │
    ├─→ requestBuilder.buildPromptJson() ──→ JSON message structure
    │
    ├─→ requestBuilder.buildCommand() ──→ CLI command
    │
    └─→ ClaudeCodeRequest (data object)
        • preMessages (approve/deny commands)
        • promptJson (user message)
        • workDir (process directory)
        • outputJsonFile (mirror path)
        • exitAfterResult (flag)
```

### Response Path
```
JSON Event Stream (stdin)
    │
    ├─→ mapper.readTree(line) ──→ JsonNode
    │
    ├─→ Check event type
    │
    ├─→ if "result":
    │   └─→ jsonParser.parseResult()
    │       • Token aggregation
    │       • Error handling
    │       • Prepend thinking/text events
    │       → ClaudeCodeResponse
    │
    ├─→ if "tool_use":
    │   └─→ jsonParser.parseToolUse()
    │       → ClaudeCodeResponse (with toolUseId)
    │
    ├─→ if "assistant":
    │   └─→ jsonParser.collectAssistantEvents()
    │       • Extract thinking/text blocks
    │       • Dedup on (type, content)
    │       → Add to LinkedHashMap
    │
    ├─→ if "message_delta":
    │   └─→ jsonParser.collectMessageDeltaEvent()
    │       • Extract thinking_tokens
    │       → Accumulate in counter
    │
    └─→ if "rate_limit_event":
        └─→ jsonParser.processRateLimitEvent()
            • Format for logging
            → LOG.info()
```

### Conversion Path
```
ClaudeCodeResponse
    │
    └─→ convertResponse()
        ├─→ Extract inputTokens (+ cache)
        ├─→ Extract outputTokens
        ├─→ Extract reasoningTokens
        ├─→ Calculate totalTokens
        └─→ AIAnswer
            • Fully structured response
            • Ready for UI/storage
```

## Dependencies

### External (Jackson)
```
com.fasterxml.jackson.databind
├── JsonNode          [ClaudeCodeJsonParser]
├── ObjectMapper      [ClaudeCodeRequestBuilder, ClaudeCodeJsonParser]
└── node.*            [ClaudeCodeRequestBuilder]
```

### Internal (Workbench)
```
xy.ai.workbench
├── ConfigManager     [ClaudeCodeRequestBuilder]
├── LOG               [ClaudeCodeJsonParser]
├── ResultPostProcessor [ClaudeCodeJsonParser]
└── models.*          [ClaudeCodeConnector]
```

### Eclipse
```
org.eclipse.*
├── IProject
├── IProgressMonitor
└── UI components (Display, Workbench, Editor)
```

## Design Patterns

### 1. **Separation of Concerns**
- **Builder Pattern:** `ClaudeCodeRequestBuilder` handles construction
- **Parser Pattern:** `ClaudeCodeJsonParser` handles parsing
- **Orchestrator Pattern:** `ClaudeCodeConnector` coordinates components

### 2. **Single Responsibility**
- `ClaudeCodeConnector`: Process lifecycle and orchestration
- `ClaudeCodeRequestBuilder`: Request/message construction
- `ClaudeCodeJsonParser`: Response event parsing
- `ResultPostProcessor`: Result text transformation

### 3. **Dependency Injection**
- Components receive dependencies through constructor
- No static dependencies
- Testable in isolation

### 4. **Event Dispatching**
- Stream-based event processing (no buffering)
- Type-based handler dispatch
- Early exit on terminal events

## Metrics

| Aspect | Value |
|--------|-------|
| ClaudeCodeConnector Complexity | Low (12 methods) |
| ClaudeCodeRequestBuilder Complexity | Very Low (4 methods) |
| ClaudeCodeJsonParser Complexity | Medium (6 methods) |
| Cyclomatic Complexity | ~3-4 per method (good) |
| Test Isolation | High (3 independent test suites) |

## Future Extensions

1. **Config-Driven Builder**
   ```java
   class ClaudeCodeCommandConfig {
       List<String> defaultArgs();
       String scriptPath();
   }
   ```

2. **Event Handler Registry**
   ```java
   class EventDispatcher {
       Map<String, EventHandler> handlers;
       void dispatch(JsonNode event);
   }
   ```

3. **Response Validator**
   ```java
   class ClaudeCodeResponseValidator {
       void validate(ClaudeCodeResponse);
   }
   ```

4. **Token Counter**
   ```java
   class TokenCounterService {
       long totalTokens(ClaudeCodeResponse);
   }
   ```

---
**Version:** 1.0 (Post-Refactoring)  
**Last Updated:** 2026-07-03
