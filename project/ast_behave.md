Ich würde gerne das AST, Spellcheck und Syntax Highlighting verhalten mittels Debounce verbessern.
Dazu müssen Document-Änderungen und Eingaben zentral gebündelt werden.
Ich würde vorschlagen alle drei Aspekte über einen Manager zu bündeln und zu verstecken, den Manager im Editor integrieren und alles Event-Handling durch diesen zu leiten. Quasi ein Document oder Editor Addon.
Prüfe und Implementiere den Ansatz.

* Editor: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AISessionEditor.java`
* AST: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/mdast/MarkdownDocument.java`
* Spellcheck: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/spellcheck/SpellCheckInstaller.java`
* Syntax Highlighting: `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/editor/AIRuleScanner.java`