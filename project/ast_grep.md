Erweitere `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/grep/__init__.py` um ein Postprocessing.
Wenn AST-Read aktiviert ist, soll eine Rückauflösung der Grep Ergebnisse zu AST-Node ID auf Basis der gefundenen Zeilennummern erfolgen.
Die Ergebnisse sollen dann nicht gruppiert werden von Dateien zu Matches, sondern von Dateien zu Node-ID zu Matches (neue Zwischenebene).

- Prüfung auf Tools: `with_ast = bool({'tools', 'ast_read'} & ctx.session.enabled_tools)`
- AST-Find kann bereits Zeilen zu Nodes auflösen: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/find.py`