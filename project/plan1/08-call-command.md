# Step 08 – `/call`-Kommando (Phase 7)

**Übersicht:** `/home/user/xyan/xy.ai.workbench/project/plan1/00-overview.md`
**Ziel:** MCP-Templates erhalten ein angehängtes `/call`-Kommando und das `Tool:`-Präfix. Der
PromptHandler erkennt `/call`, erfasst den davor liegenden yaml-Block, typisiert ihn und übergibt ihn
via Prompt-Objekt an den MCP-Client. Der SessionProcessor wandelt den Tool Call in den korrekten
Message-Typ und ignoriert die `/call`-Zeile.
**Abhängigkeiten:** Steps 03 (CallCommand), 04 (Processor), 06 (PromptHandler), 07 (Prompt-API).

## Ist-Zustand
`connector/mcp/MCPControlClient.renderSchema(JsonNode tool)` liefert:
```java
sb.append("```yaml\n");
sb.append("tool: ").append(tool.path("name").asText()).append("\n");
// arguments: ... (oder 'arguments: {}')
sb.append("```");
return sb.toString();
```
`connector/mcp/MCPConnector.executeRequest` (case Prompt): `control.extractYamlBlock(param)` →
`parseYaml` → `client.findTool(name)` → `control.fillReason(tool, args)` → `client.callTool(name, args)`.
`EditorInterface.TOOLUSE = "Tool:"`. SessionProcessor `consumeToolCall` erwartet nach `Tool:` einen
`\`\`\`yaml`-Block mit `id`/`tool`/`arguments` und emittiert `callbacks.toolCall(new ToolCall(...))`.
`MCPClient.renderToolCall(String name, JsonNode arguments)` existiert bereits.

## Aktionen
1. `MCPControlClient.renderSchema` (bzw. der Rückgabepfad in `MCPConnector.executeRequest` case `Tool`):
   - Dem yaml-Template das Präfix `EditorInterface.TOOLUSE` (`"Tool:"`) auf eigener Zeile voranstellen.
   - Nach dem schließenden ` ``` ` eine Kommandozeile `EditorInterface.CMD_CALL` (`"/call"`) anhängen.
   Ergebnisform:
   ```
   Tool:
   ```yaml
   tool: <name>
   arguments:
     ...
   ```
   /call
   ```
2. `PromptHandler` (Step 06, Kommandoerkennung): Bei `/call` in der Zeile
   - rückwärts ab der Kommandozeile den vorausgehenden yaml-Block erfassen: Startgrenze Regex
     `^\`\`\`yaml$`, Endgrenze `^\`\`\`$` (die Zeile unmittelbar vor `/call`).
   - Block parsen/typisieren (`CallCommand`), in `Prompt` ablegen (`command` + `yamlBlock`).
   - Bei FullFile-Modus: Cursorzeile bzw. letzte Zeile gemäß Regeln aus Step 06.
3. Übergabe an MCP: Der MCP-Pfad (`MCPConnector.executeRequest` / `MCPClient.callTool`) erhält den
   getypten Call aus dem `Prompt` (statt erneutem String-Parsing). `fillReason` weiterhin anwenden.
4. `SessionProcessor` (Step 04): Der `Tool:`-Block wird via `consumeToolCall` in den korrekten
   Message-Typ gewandelt; die nachfolgende `/call`-Zeile ist ein REMOVE-Kommando (CallCommand) und wird
   ignoriert. Sicherstellen, dass `/call` einen offenen Block via `isMarkerLine` beendet (Step 04).

## Hinweise / Fallstricke
- Der `Tool:`-Block muss vom SessionProcessor unverändert als Tool Call erkannt werden (Symmetrie/
  Austauschbarkeit zwischen Connectoren). yaml-Schlüssel `tool`/`arguments`/optional `id` beibehalten.
- `/call` darf das Prefix-Caching nicht unterbrechen: Kommandozeile am Blockende, deterministische Form.
- Konsistenz mit bestehendem `MCPConnector` Prompt-Pfad (bisher `/tool` → Template, freier yaml-Block →
  Call). `/tool` bleibt für Template-Anforderung; `/call` löst den Call aus.

## Validierung
`ast_validate` für `connector/mcp/MCPControlClient.java`, `connector/mcp/MCPConnector.java`,
`connector/harness/PromptHandler.java`, `connector/harness/SessionProcessor.java`.
