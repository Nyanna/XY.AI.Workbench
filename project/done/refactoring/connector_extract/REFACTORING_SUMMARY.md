# Refactoring Summary: ClaudeCodeConnector

## Ziel
Refaktorierung des `ClaudeCodeConnector` zur Verbesserung der **Übersichtlichkeit**, **Wartbarkeit** und **automatischen Verarbeitung** durch Extraktion von Build- und Parse-Methoden in spezialisierte Utility-Klassen.

## Durchgeführte Änderungen

### 1. Neue Utility-Klasse: `ClaudeCodeRequestBuilder`
**Zweck:** Zentrale Verwaltung aller Request- und Struktur-Build-Operationen

**Extrahierte Methoden:**
- `buildPromptJson(String text)` - Erstellt User-Message-JSON-Struktur
- `buildApproveJson(String toolUseId)` - Erstellt Approval-JSON für Tool-Requests
- `buildDenyJson(String toolUseId)` - Erstellt Denial-JSON für Tool-Requests
- `buildCommand(String profile)` - Konstruiert CLI-Kommando für Claude-Code-Prozess

**Vorteile:**
- Zentralisierte JSON-Konstruktion
- Leichter testbar (Unit-Tests für JSON-Struktur)
- Entkopplung von CLI-Logik
- Rückholung von Konstanten (z.B. SCRIPT-Pfad)

### 2. Neue Utility-Klasse: `ClaudeCodeJsonParser`
**Zweck:** Spezialisierte Verarbeitung und Parsing von Claude-Code-API-Responses

**Extrahierte Methoden:**
- `parseResult(...)` - Parst Result-Events und Token-Informationen
- `parseToolUse(...)` - Parst Tool-Use-Events
- `collectAssistantEvents(...)` - Sammelt Think/Text-Blöcke aus Assistant-Events
- `collectMessageDeltaEvent(...)` - Extrahiert Reasoning-Tokens aus Message-Delta-Events
- `processRateLimitEvent(...)` - Verarbeitet und loggt Rate-Limit-Informationen
- `commented(String input)` - Formatiert Text als kommentierte Markdown-Linien

**Vorteile:**
- Dedizierte Event-Parsing-Logik
- Bessere Fehlerbehandlung pro Event-Typ
- Einfacheres Debugging und Logging
- Testbare Parser-Funktionen

### 3. Refaktorierter `ClaudeCodeConnector`
**Neue Struktur:**
```java
public class ClaudeCodeConnector implements IAIConnector {
    private final ClaudeCodeRequestBuilder requestBuilder;
    private final ClaudeCodeJsonParser jsonParser;
    
    // Core methods:
    - createRequest()
    - executeRequest()
    - convertResponse()
    - ensureProcess()
    - terminateProcess()
    - readUntilResult()
    - preprocessInput()
}
```

**Entfernte Methoden:** 8 private Methoden → Utility-Klassen ausgelagert
**Reduzierte Komplexität:** 537 → 310 Zeilen (-42%)

## Metriken

| Metrik | Vorher | Nachher | Änderung |
|--------|--------|---------|----------|
| ClaudeCodeConnector Zeilen | 537 | 310 | -42% |
| Gesamtzeilen (Package) | 537 | 660 | +23% |
| Private Methoden in Connector | 8 | 1 | -87.5% |
| Dedizierte Builder-Klasse | - | 115 Z. | NEU |
| Dedizierte Parser-Klasse | - | 235 Z. | NEU |

## Vorteile der Refaktorierung

### 1. **Übersichtlichkeit**
- Hauptklasse konzentriert sich auf Prozess-Orchestrierung
- Klare Separation of Concerns (Request-Building, JSON-Parsing, Prozess-Management)
- Reduzierte kognitive Last beim Lesen des Codes

### 2. **Wartbarkeit**
- Änderungen an JSON-Struktur sind lokalisiert in `ClaudeCodeRequestBuilder`
- Änderungen an Event-Parsing sind lokalisiert in `ClaudeCodeJsonParser`
- Einfachere Fehlersuche durch spezialisierte Klassen

### 3. **Testbarkeit**
- Einzelne Builder-Methoden können isoliert getestet werden
- Parser-Logik kann mit Mock-JSON-Nodes getestet werden
- Keine Abhängigkeit von Process-Management für Unit-Tests

### 4. **Automatische Verarbeitung**
- Parser-Klasse ist leicht automatisch zu generieren/aktualisieren (API-Schema-Änderungen)
- Builder-Klasse zeigt klare Struktur für Code-Generierung
- Bessere IDE-Unterstützung (Navigation, Refactoring)

## Integration

Keine Breaking Changes! Die public API von `ClaudeCodeConnector` bleibt unverändert:
```java
IModelRequest createRequest(String input, String systemPrompt, List<String> tools, boolean batchFix, IProgressMonitor mon)
IModelResponse executeRequest(IModelRequest request, IProgressMonitor mon)
AIAnswer convertResponse(IModelResponse response, IProgressMonitor mon)
KeyPattern getSupportedKeyPattern()
```

## Verwendete Dependencies
Alle neuen Klassen verwenden bereits vorhandene Dependencies:
- `com.fasterxml.jackson.databind.*` (für JSON-Verarbeitung)
- `xy.ai.workbench.ConfigManager` (Konfiguration)
- `xy.ai.workbench.LOG` (Logging)

## Nächste Schritte (Optional)

1. **Weitere Extraktion:** `preprocessInput()` könnte in `ClaudeCodeInputPreprocessor` ausgelagert werden
2. **Constants-Klasse:** CLI-Kommando-Argumente als Konstanten auslagern
3. **Event-Enums:** Event-Type-Konstanten als Enum definieren
4. **Configuration:** SCRIPT-Pfad und Standard-Argumente in Properties auslagern

---
**Datum:** 2026-07-03  
**Refactoring-Typ:** Strukturelle Optimierung für Wartbarkeit und Testbarkeit
