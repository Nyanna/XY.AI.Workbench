Eigentlich sollten die ID's die Eingabe ID spiegel aber die sind verschwunden("null")?
-`/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/edit_block.py`
```yaml
id: 9822-46
toolName: ast_edit_block
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
    old_text: "\tprivate final ConfigManager cfg;\n\tprivate final AdaptingConnector connector;"
    new_text: "\tprivate final ConfigManager cfg;\n\tprivate final AdaptingConnector connector;\n\t/** Discovers MCP tool names to populate a model's Capabilities.tools; caches results. */\n\tprivate final MCPClient mcpClient = new MCPClient();"
  - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
    old_text: "\t\tcfg.addInputModeObs(i -> updateInputStat(i));\n\t\tcfg.addEnabledToolsObs(t -> updateInputStat(InputMode.Tools), false);\n\t}"
    new_text: "\t\tcfg"
  reason: MCP-Tool-Discovery als Observer auf Model-Wechsel implementieren
```
```yaml
id: 9822-47
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
      id: null
      result: success
      new_id: AISessionManager.A7Wqyo
    errors:
    - path: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/AISessionManager.java
      id: null
      error: "Text found in node but rejected: the edit would merge/split lines or break syntax there."
      reason: guard_rejected
```