Ich benötige eine Anbindung von Deepseek analog zu `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/openai/OpenAIConnector.java` im Zielpaket `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/deepseek`, auf Basis des SDK in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/openapi/deepseek`.

- Schema: `/home/user/xyan/xy.ai.workbench/libs/openapi/filters/deepseek.filtered.yaml`
- Es muss ein Mapping in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/AdaptingConnector.java` angelegt werden. Ein "Deepseek" Key Pattern ist bereits angelegt.

- Batch support wird nicht benötigt. Die Modellkonfiguration ist bereits implementiert.
- base_url (OpenAI)	https://api.deepseek.com
- Einen "safetyIdentifier" gibt es nicht, das muss analog `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claude/ClaudeConnector.java` gelöst werden
- Eine erweiterte Harness Unterstützung mit Tool-Loop erfolgt später.
/tool ast_list
```yaml
tool: ast_list
arguments:
  # Absolute paths of the files to list.
  paths:
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/openai/OpenAIConnector.java
    - /home/user/xyan/xy.ai.workbench/libs/openapi/filters/deepseek.filtered.yaml
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/AdaptingConnector.java
    - /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/claude/ClaudeConnector.java
```