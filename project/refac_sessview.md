`/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/AISessionView.java` is zu lang geworden. Überlege eine Aufteilung für eine Extraktion von Aspekten unter Berücksichtigung der geteilten Felder.

Eine Texteditor Referenz ("getLastTextEditor") soll im Prompt Objekt `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/Prompt.java` gespeichert werden. Das Prompt Objekt soll im jeweiligen `IModelRequest` gehalten werden. Das Prompt Objekt soll anschließend an `AIAnswer` übergeben werden, wo es in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/EditorInterface.java` in `replaceTag`, wenn vorhanden, als erster Hint für das Tag Replacement genutzt werden soll.
Im Fall eines Batch ist der Prompt später nicht mehr verfügbar.

Der `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/connector/harness/PromptHandler.java` soll in seine zwei Apekte aufgeteilt werden.
1. Koordination und Ausführung soll im PromptHandler bleiben.
2. Input Handling, Aggregation und Preprocessing (Kommandosubstitution) -> soll ausgelagert werden nach "PromptInputHandler"