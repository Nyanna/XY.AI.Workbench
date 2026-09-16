# Step 09 – Cleanup & Gesamtvalidierung (Phase 8)

**Übersicht:** `/home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md`
**Ziel:** Redundanzen entfernen, Konsistenz herstellen, alles validieren.
**Abhängigkeiten:** Steps 01–08 abgeschlossen.

## Aktionen
1. Verstreute String-Literale entfernen: Alle verbliebenen `"/exit"`, `"/resume"`, `"/answer"`,
   `"/tool"`, `"Control Request:"` durch `EditorInterface.CMD_*` / `CONTROL_REQUEST` bzw.
   `CommandHandler` ersetzen (grep im gesamten `connector/`-Baum).
2. Prüfen, ob `CommandType` (claudecode) / `MCPCommandType` und die inneren `Command`/`MCPCommand`
   noch nötig sind oder auf die globale `Command`-Hierarchie konsolidiert werden können. Falls
   Konsolidierung: `CCRequest.Command`, `MCPCommand`, `MCPRequest`-Payload auf globale Typen umstellen.
   Sonst: Mapping-Schicht dokumentieren.
3. `AISessionManager` auf verbleibende tote Prompt-Logik prüfen (nach Auslagerung in `PromptHandler`).
4. `sessionProcessor.setEnabled` – prüfen, dass es keine direkten Aufrufe außerhalb des
   Kompatibilitätspfads mehr gibt (grep). Aktivierung ausschließlich über `Prompt.processorEnabled`
   / `process(inputs, enabled, cb)`.
5. Invarianten-Review: Symmetrie (Tool-Call-/Result-Blöcke connectorübergreifend durchleitbar),
   Determinismus, Prefix-Caching (stabile Block-Reihenfolge/Whitespace).

## Grep-Checkliste
Via grep MCP-Tool
```
grep -rE '"/(exit|resume|answer|tool|call)"' src/xy/ai/workbench/connector
grep -rE 'Control Request:' src/xy/ai/workbench
grep -rE 'createRequest\(' src/xy/ai/workbench            # keine Alt-Signatur mehr
grep -rE 'sessionProcessor\.setEnabled' src/xy/ai/workbench
grep -rE 'ClaudeSessionParameters\.fromConfig' src/xy/ai/workbench
```

## Finale Validierung (`ast_validate`)
- `EditorInterface.java`, `AISessionManager.java`, `Activator.java`, `ConfigManager.java`
- `connector/`: `IAIConnector.java`, `AdaptingConnector.java`, `Command.java`, `CommandHandler.java`,
  `SessionParameters.java`
- `connector/harness/`: `SessionProcessor.java`, `Prompt.java`, `FrozenConfig.java`, `PromptHandler.java`
- `connector/claudecode/`: `CCControlClient.java`, `CCConnector.java`, `CCRequest.java`,
  `CommandType.java`, `ClaudeSessionParameters.java`
- `connector/mcp/`: `MCPConnector.java`, `MCPControlClient.java`, `MCPClient.java`, `MCPRequest.java`
- `connector/claude/ClaudeConnector.java`, `connector/deepseek/DeepSeekConnector.java`,
  `connector/google/GeminiConnector.java`, `connector/openai/OpenAIConnector.java`
- `view/AISessionView.java` (Aufrufer)
