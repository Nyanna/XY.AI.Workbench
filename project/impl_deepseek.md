Ich benötige eine Anbindung von Deepseek auf Basis von `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/openai/OpenAIConnector.java`. Ich denke eine einfache Ableitung und Anpassung der Zielurl könnte reichen. Ebenfalls muss ein Mapping in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/AdaptingConnector.java` angelegt werden.

- Batch support wird nicht benötigt. Die Modellkonfiguration ist bereits implementiert.
- base_url (OpenAI)	https://api.deepseek.com

- Profile Combo ausblenden wenn leer
- Deepseek thinking falsche capabilities hard überschreiben none, low, high, max
- Für deepseek extra capability flag und Temp/TopP nur wenn thinking disabled
- temperature not in thinking
- top_p only when not thinking
- not safetyIdentifier