Implementiere einen Dry-Run Aspekt auf Basis eines Interceptor für alle schreibenden MCP-Tool (AST/Standalone).

- Dry Run wird über einen HTTP Parameter allein über HTTP aktiviert: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/http_transport.py`
- Der Parameter setzt den Interceptor in die Session `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/server/session.py`
- Der interceptor selbst wird dann in den Tool Backends und Engines geprüft und aufgerufen.
- Interceptor Beschreibung: `/home/user/xyan/xy.ai.workbench/project/dry-run-interceptor-prompt.md`
- AST Engine: `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/python/_engine.py`, `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ast/generic/_engine.py`

 - Der Interceptor muss folgende Methoden der AST Engines Abdecken: node_code,replace,insert,delete,append
 	- für delete muss nur die ID gespeichert werden, kein alter Inhalt
 - Schreibende AST Tools: create,delete,edit_block,edit_marks,insert,replace
 - Schreibende Standalone Tools: edit_block,edit_chars_edit_line_edit_lines_edit_marks,write
 - Toolsemantik beibehalten