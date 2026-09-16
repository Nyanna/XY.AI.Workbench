In `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/deepseek/DeepSeekConnector.java` Zeile 326 kommt als Reasoning von Deepseek folgendes Zurück

```json
{"type":"reasoning","id":"9aec8cf2-41dd-4397-b22b-c413e34a2e03","status":"completed","content":[{"type":"reasoning_text","text":"The user wants me to call the tool on `/home/user/xyan/xy.ai.workbench/icons`."}],"summary":[],"encrypted_content":"e9d9093f-20d6-41a3-a39c-47a80689b483-0"}
```

Diese Struktur muss durch `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/SessionProcessor.java` formatiert ausgegeben und auch wieder symetrisch eingelesen werden können.
Mein Vorschlag ist:

- `EditorInterface.THINKING` für den Reasoning Text verwenden (`reasoning_text`)
- Einen neuen Prefix "Thinking Meta:" einführen und das JSON danach einzeilig anfügen. Der Value von `reasoning_text` wird durch einen Platzhalter ersetzt.
- Dies erlaubt eindeutige Diskriminierung in beide Richtung und ist leicht lesbar. Auch mehrzeiliges Thinking kann sicher abgegrenzt werden.
- Beim Einlesen triggert `EditorInterface.THINKING` dann bis zum neuen Prefix und das JSON kann wieder in seine Ursprungsform übersetzt werden.
- Die Schnittstelle zwischen Connector und Processor muss dabei generisch für alle Connectoren geeignet sein (Text, JSON-Message) 