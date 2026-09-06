# Ask-User Tool

Erstelle parallel zum Read-Tool `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py` ein "ask-user"-Tool in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ask-user/__init__.py`.
Ziel ist es Agenten einen Rückkanal anzubieten, um den Nutzer Fragen zu stellen, die die Effizienz der Session erhöhen. Beispielweise ist es unnötig ganze Dateihierarchien zu durchsuchen, wenn es wahrscheinlich ist, das der Nutzer leichter an eine Antwort kommt oder sie bereits hat.

* Das Tool hat als Input nur die Frage an den Nutzer in der Sprache des Nutzers.
* Der Output ist die Antwort. 
* Die Implementierung gibt jedoch als Antwort stets zurück das der Nutzer nicht geantwortet hat und es dem Agenten selbst überlassen ist.

## Hintergrund

Das Permission-System erlaubt bereits dem Nutzer in Tool-Aufrude und Ausgaben einzugreifen. So das hier eine einfache Dummy-Implementierung ausreicht. Der Hauptnutzen dieser Abbildung ist die API und Integration als MCP-Tool.
#: Control Request:
#: {
#:   "id" : "5a55d54e-d6a5-4c66-9828-e95cb59a91c3",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py"
#:   }
#: }
/allow 5a55d54e-d6a5-4c66-9828-e95cb59a91c3
#: Control Request:
#: {
#:   "id" : "d565e2d2-4683-47db-817c-881105aedaef",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "\"\"\"Read tool – returns file contents, optionally sliced by line or unique marker.\n\nRange: start = min_line | start-marker | file start; end = max_line | end-marker\n| file end (all inclusive). Markers must be unique substrings. Per-session\nsha256 cache rejects unchanged re-reads (key ``_read_cache`` in session state).\n\"\"\"\n\nfrom __future__ import annotations\n\nimport hashlib\nfrom pathlib import Path\nfrom typing import Any\n\nfrom ...registry import ToolContext, ToolRegistry, ToolResult\n\n#: Key used inside ``Session.state`` to persist the per-session file cache.\n_CACHE_KEY = \"_read_cache\"\n\n\ndef _sha256(data: bytes) -> str:\n    return hashlib.sha256(data).hexdigest()\n\n\ndef register_read_tool(registry: ToolRegistry) -> None:\n    @registry.tool(\n        \"read\",\n        title=\"Read file\",\n        description=(\n            \"Read a file as text, optionally sliced to a range. Repeated \"\n            \"unchanged reads return an error.\"\n        ),\n        input_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                \"path\": {\n                    \"type\": \"string\",\n                    \"description\": \"Absolute file path.\",\n                },\n                \"min_line\": {\n                    \"type\": \"integer\",\n                    \"description\": \"Range start: line number, inclusive, 1-based. Excludes start.\",\n                    \"minimum\": 1,\n                },\n                \"max_line\": {\n                    \"type\": \"integer\",\n                    \"description\": \"Range end: line number, inclusive, 1-based. Excludes end.\",\n                    \"minimum\": 1,\n                },\n                \"start\": {\n                    \"type\": \"string\",\n                    \"description\": \"Range start: unique marker substring, inclusive. Excludes min_line.\",\n                },\n                \"end\": {\n                    \"type\": \"string\",\n                    \"description\": \"Range end: unique marker substring, inclusive. Excludes max_line.\",\n                },\n            },\n            \"required\": [\"path\"],\n        },\n        output_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                #\"path\": {\"type\": \"string\"},\n                #\"sha256\": {\"type\": \"string\"},\n                #\"total_lines\": {\"type\": \"integer\"},\n                #\"returned_lines\": {\"type\": \"integer\"},\n                \"content\": {\"type\": \"string\"},\n                #\"min_line\": {\"type\": \"integer\"},\n                #\"max_line\": {\"type\": \"integer\"},\n            },\n            \"required\": [\"content\"\n                         #, \"path\", \"sha256\", \"total_lines\", \"returned_lines\"\n                         ],\n        },\n        annotations={\"readOnlyHint\": True, \"openWorldHint\": False},\n    )\n    def read(ctx: ToolContext) -> ToolResult:\n        args: dict[str, Any] = ctx.arguments\n        path_str: str = args[\"path\"]\n        min_line: int | None = args.get(\"min_line\")\n        max_line: int | None = args.get(\"max_line\")\n        start_marker: str | None = args.get(\"start\")\n        end_marker: str | None = args.get(\"end\")\n\n        # --- mutual exclusivity validation ---\n        if min_line is not None and start_marker is not None:\n            return ToolResult(\n                structured_content={\n                    \"error\": \"``min_line`` and ``start`` are mutually exclusive.\"\n                },\n                is_error=True,\n            )\n        if max_line is not None and end_marker is not None:\n            return ToolResult(\n                structured_content={\n                    \"error\": \"``max_line`` and ``end`` are mutually exclusive.\"\n                },\n                is_error=True,\n            )\n\n        path = Path(path_str)\n        if not path.is_absolute():\n            return ToolResult(\n                structured_content={\"error\": f\"Path must be absolute: {path_str}\"},\n                is_error=True,\n            )\n        if not path.exists():\n            return ToolResult(\n                structured_content={\"error\": f\"File not found: {path_str}\"},\n                is_error=True,\n            )\n        if not path.is_file():\n            return ToolResult(\n                structured_content={\"error\": f\"Not a regular file: {path_str}\"},\n                is_error=True,\n            )\n\n        raw_bytes = path.read_bytes()\n        current_hash = _sha256(raw_bytes)\n\n        # --- session cache check ---\n        cache: dict[str, str] = ctx.session.state.setdefault(_CACHE_KEY, {})\n        key = str(path.resolve())\n        if cache.get(key) == current_hash:\n            return ToolResult(\n                structured_content={\n                    \"error\": f\"File has not changed since the last read (sha256={current_hash}): {path_str}\"\n                },\n                is_error=True,\n            )\n        cache[key] = current_hash\n\n        # --- decode ---\n        text = raw_bytes.decode(\"utf-8\", errors=\"replace\")\n        lines = text.splitlines(keepends=True)\n        total_lines = len(lines)\n\n        def line_start_offset(line_num: int) -> int:\n            n = max(0, min(line_num - 1, total_lines))\n            return sum(len(l) for l in lines[:n])\n\n        def line_end_offset(line_num: int) -> int:\n            n = max(0, min(line_num, total_lines))\n            return sum(len(l) for l in lines[:n])\n\n        # --- resolve start boundary ---\n        if start_marker is not None:\n            start_count = text.count(start_marker)\n            if start_count == 0:\n                return ToolResult(\n                    structured_content={\"error\": f\"Start marker not found in file: {start_marker!r}\"},\n                    is_error=True,\n                )\n            if start_count > 1:\n                return ToolResult(\n                    structured_content={\n                        \"error\": (\n                            f\"Start marker is ambiguous – found {start_count} occurrences \"\n                            f\"in file: {start_marker!r}\"\n                        )\n                    },\n                    is_error=True,\n                )\n            region_start = text.index(start_marker)\n        elif min_line is not None:\n            region_start = line_start_offset(min_line)\n        else:\n            region_start = 0\n\n        # --- resolve end boundary ---\n        if end_marker is not None:\n            end_count = text.count(end_marker)\n            if end_count == 0:\n                return ToolResult(\n                    structured_content={\"error\": f\"End marker not found in file: {end_marker!r}\"},\n                    is_error=True,\n                )\n            if end_count > 1:\n                return ToolResult(\n                    structured_content={\n                        \"error\": (\n                            f\"End marker is ambiguous – found {end_count} occurrences \"\n                            f\"in file: {end_marker!r}\"\n                        )\n                    },\n                    is_error=True,\n                )\n            region_end = text.index(end_marker) + len(end_marker)\n        elif max_line is not None:\n            region_end = line_end_offset(max_line)\n        else:\n            region_end = len(text)\n\n        # --- order validation ---\n        if region_end < region_start:\n            return ToolResult(\n                structured_content={\n                    \"error\": (\n                        f\"Resolved end position ({region_end}) must not lie before \"\n                        f\"the resolved start position ({region_start}).\"\n                    )\n                },\n                is_error=True,\n            )\n\n        sliced = text[region_start:region_end]\n\n        structured: dict[str, Any] = {\"content\": sliced}\n        if min_line is not None:\n            structured[\"min_line\"] = min_line\n        if max_line is not None:\n            structured[\"max_line\"] = max_line\n        if start_marker is not None:\n            structured[\"start_line\"] = text.count(\"\\n\", 0, region_start) + 1\n        if end_marker is not None:\n            structured[\"end_line\"] = text.count(\"\\n\", 0, region_end) + 1\n\n        return ToolResult(structured_content=structured)\n"
#:     }
#:   }
#: }
/allow d565e2d2-4683-47db-817c-881105aedaef
#: Control Request:
#: {
#:   "id" : "c66d3b26-e303-4099-8510-b39a57cfdafb",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py"
#:   }
#: }
/allow c66d3b26-e303-4099-8510-b39a57cfdafb
#: Control Request:
#: {
#:   "id" : "62ce34d1-40b2-4819-8bf3-90e7e2647fc8",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "\"\"\"File-system and shell tools for the MCPC server.\n\nAvailable tools\n---------------\n* ``read``          – read a file (with session-level content-hash caching)\n* ``write``         – overwrite or append to a file\n* ``insert``        – insert text at a character offset\n* ``replace-chars`` – replace a character range with new text\n* ``replace-lines`` – replace a line range with new text\n* ``change``        – replace a delimited block identified by start/end markers\n* ``bash``          – run a Bash script in a given working directory\n* ``python``        – run a Python script directly from context\n* ``markdown``      – AST-based Markdown editing via a remark (Node.js) script\n\nSkills (on-demand hint tools) are registered from the ``skills`` sub-package.\nBridges to external MCP servers (e.g. Exa) live in the ``mcp`` sub-package.\n\nCall :func:`register_tools` to register all tools onto a\n:class:`~xy.ai.mcpc.registry.ToolRegistry` instance.\n\"\"\"\n\nfrom __future__ import annotations\n\nfrom ..registry import ToolRegistry\nfrom .bash import register_bash_tool\nfrom .change import register_change_tool\nfrom .insert import register_insert_tool\nfrom .markdown import register_markdown_tool\nfrom .mcp import register_context7_tools, register_exa_tools, register_github_tools\nfrom .python import register_python_tool\nfrom .read import register_read_tool\nfrom xy.ai.mcpc.tools.replace_chars import register_replace_chars_tool\nfrom .replace_lines import register_replace_lines_tool\nfrom .skills import register_skills\nfrom .write import register_write_tool\n\n\ndef register_tools(registry: ToolRegistry) -> None:\n    \"\"\"Register all built-in file-system and shell tools onto *registry*.\"\"\"\n    register_read_tool(registry)\n    register_write_tool(registry)\n    register_insert_tool(registry)\n    register_change_tool(registry)\n    register_replace_chars_tool(registry)\n    register_replace_lines_tool(registry)\n    register_bash_tool(registry)\n    register_python_tool(registry)\n    register_markdown_tool(registry)\n    register_skills(registry)\n    register_exa_tools(registry)\n    register_github_tools(registry)\n    register_context7_tools(registry)\n\n\n# Keep the old name available so existing call sites don't break.\nregister_builtin_tools = register_tools\n\n__all__ = [\n    \"register_tools\",\n    \"register_builtin_tools\",\n]\n"
#:     }
#:   }
#: }
/allow 62ce34d1-40b2-4819-8bf3-90e7e2647fc8
#: Control Request:
#: {
#:   "id" : "57597515-3093-43b2-8f55-e5483584e7a2",
#:   "phase" : "request",
#:   "toolName" : "write",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/ask-user/__init__.py",
#:     "mode" : "replace",
#:     "content" : "\"\"\"Ask-user tool – lets an agent ask the human a clarifying question.\n\nThis tool exists to give agents a back-channel to the user so they can ask\nquestions that improve session efficiency instead of, e.g., exhaustively\nsearching whole directory hierarchies when the user might already know the\nanswer or can find it far more easily.\n\nThe permission system already allows the user to intercept tool calls and\ntheir outputs, so this implementation is intentionally a simple dummy: it\nalways reports that the user did not answer, leaving it up to the agent to\nproceed on its own (e.g. by falling back to exploration). The main value of\nthis module is exposing a well-defined API/MCP tool for the interaction, not\nan actual answering mechanism.\n\"\"\"\n\nfrom __future__ import annotations\n\nfrom typing import Any\n\nfrom ...registry import ToolContext, ToolRegistry, ToolResult\n\n#: Answer returned whenever the user has not (yet) responded.\n_NOT_ANSWERED = \"The user did not answer. Proceed on your own.\"\n\n\ndef register_ask_user_tool(registry: ToolRegistry) -> None:\n    @registry.tool(\n        \"ask-user\",\n        title=\"Ask user\",\n        description=(\n            \"Ask the user a clarifying question, in the user's language, to \"\n            \"improve session efficiency (e.g. instead of searching an entire \"\n            \"file hierarchy when the user likely knows the answer already). \"\n            \"The user may answer via the permission system; if not, this \"\n            \"tool reports that no answer was given and it is up to the agent \"\n            \"to proceed on its own.\"\n        ),\n        input_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                \"question\": {\n                    \"type\": \"string\",\n                    \"description\": \"The question to ask the user, in the user's language.\",\n                },\n            },\n            \"required\": [\"question\"],\n        },\n        output_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                \"answer\": {\"type\": \"string\"},\n            },\n            \"required\": [\"answer\"],\n        },\n        annotations={\"readOnlyHint\": True, \"openWorldHint\": False},\n    )\n    def ask_user(ctx: ToolContext) -> ToolResult:\n        args: dict[str, Any] = ctx.arguments\n        question: str = args[\"question\"]\n        if not question or not question.strip():\n            return ToolResult(\n                structured_content={\"error\": \"``question`` must not be empty.\"},\n                is_error=True,\n            )\n\n        return ToolResult(structured_content={\"answer\": _NOT_ANSWERED})\n"
#:   }
#: }
/allow 57597515-3093-43b2-8f55-e5483584e7a2
#: Control Request:
#: {
#:   "id" : "d9f5750f-502d-4cbd-aee0-48687162afed",
#:   "phase" : "result",
#:   "toolName" : "write",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "result" : "success"
#:     }
#:   }
#: }
/allow d9f5750f-502d-4cbd-aee0-48687162afed
#: Control Request:
#: {
#:   "id" : "d91ef823-62b3-4c14-a0b2-69109b8e9963",
#:   "phase" : "request",
#:   "toolName" : "change",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py",
#:     "mode" : "full",
#:     "start" : "from __future__ import annotations\n\nfrom ..registry import ToolRegistry\nfrom .bash import register_bash_tool",
#:     "end" : "from .read import register_read_tool",
#:     "content" : "from __future__ import annotations\n\nimport importlib\n\nfrom ..registry import ToolRegistry\nfrom .bash import register_bash_tool"
#:   }
#: }
/allow d91ef823-62b3-4c14-a0b2-69109b8e9963
#: Control Request:
#: {
#:   "id" : "685ebfba-fd8c-4195-92ef-c4d86b69a629",
#:   "phase" : "result",
#:   "toolName" : "change",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "result" : "success"
#:     }
#:   }
#: }
/allow 685ebfba-fd8c-4195-92ef-c4d86b69a629
#: Control Request:
#: {
#:   "id" : "933de64e-c7ea-4e97-b901-4ba99a63b012",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py"
#:   }
#: }
/allow 933de64e-c7ea-4e97-b901-4ba99a63b012
#: Control Request:
#: {
#:   "id" : "d5dbdf4b-6b44-4791-aaf6-f60defed12e1",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "\"\"\"File-system and shell tools for the MCPC server.\n\nAvailable tools\n---------------\n* ``read``          – read a file (with session-level content-hash caching)\n* ``write``         – overwrite or append to a file\n* ``insert``        – insert text at a character offset\n* ``replace-chars`` – replace a character range with new text\n* ``replace-lines`` – replace a line range with new text\n* ``change``        – replace a delimited block identified by start/end markers\n* ``bash``          – run a Bash script in a given working directory\n* ``python``        – run a Python script directly from context\n* ``markdown``      – AST-based Markdown editing via a remark (Node.js) script\n\nSkills (on-demand hint tools) are registered from the ``skills`` sub-package.\nBridges to external MCP servers (e.g. Exa) live in the ``mcp`` sub-package.\n\nCall :func:`register_tools` to register all tools onto a\n:class:`~xy.ai.mcpc.registry.ToolRegistry` instance.\n\"\"\"\n\nfrom __future__ import annotations\n\nimport importlib\n\nfrom ..registry import ToolRegistry\nfrom .bash import register_bash_tool\nfrom xy.ai.mcpc.tools.replace_chars import register_replace_chars_tool\nfrom .replace_lines import register_replace_lines_tool\nfrom .skills import register_skills\nfrom .write import register_write_tool\n\n\ndef register_tools(registry: ToolRegistry) -> None:\n    \"\"\"Register all built-in file-system and shell tools onto *registry*.\"\"\"\n    register_read_tool(registry)\n    register_write_tool(registry)\n    register_insert_tool(registry)\n    register_change_tool(registry)\n    register_replace_chars_tool(registry)\n    register_replace_lines_tool(registry)\n    register_bash_tool(registry)\n    register_python_tool(registry)\n    register_markdown_tool(registry)\n    register_skills(registry)\n    register_exa_tools(registry)\n    register_github_tools(registry)\n    register_context7_tools(registry)\n\n\n# Keep the old name available so existing call sites don't break.\nregister_builtin_tools = register_tools\n\n__all__ = [\n    \"register_tools\",\n    \"register_builtin_tools\",\n]\n"
#:     }
#:   }
#: }
/allow d5dbdf4b-6b44-4791-aaf6-f60defed12e1
#: Control Request:
#: {
#:   "id" : "1f836cb8-4ac5-42a9-8c6b-3b5ec411755e",
#:   "phase" : "request",
#:   "toolName" : "write",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py",
#:     "mode" : "replace",
#:     "content" : "\"\"\"File-system and shell tools for the MCPC server.\n\nAvailable tools\n---------------\n* ``read``          – read a file (with session-level content-hash caching)\n* ``write``         – overwrite or append to a file\n* ``insert``        – insert text at a character offset\n* ``replace-chars`` – replace a character range with new text\n* ``replace-lines`` – replace a line range with new text\n* ``change``        – replace a delimited block identified by start/end markers\n* ``bash``          – run a Bash script in a given working directory\n* ``python``        – run a Python script directly from context\n* ``markdown``      – AST-based Markdown editing via a remark (Node.js) script\n* ``ask-user``      – ask the user a clarifying question (back-channel)\n\nSkills (on-demand hint tools) are registered from the ``skills`` sub-package.\nBridges to external MCP servers (e.g. Exa) live in the ``mcp`` sub-package.\n\nCall :func:`register_tools` to register all tools onto a\n:class:`~xy.ai.mcpc.registry.ToolRegistry` instance.\n\"\"\"\n\nfrom __future__ import annotations\n\nimport importlib\n\nfrom ..registry import ToolRegistry\nfrom .bash import register_bash_tool\nfrom .change import register_change_tool\nfrom .insert import register_insert_tool\nfrom .markdown import register_markdown_tool\nfrom .mcp import register_context7_tools, register_exa_tools, register_github_tools\nfrom .python import register_python_tool\nfrom .read import register_read_tool\nfrom xy.ai.mcpc.tools.replace_chars import register_replace_chars_tool\nfrom .replace_lines import register_replace_lines_tool\nfrom .skills import register_skills\nfrom .write import register_write_tool\n\n# ``ask-user`` uses a hyphenated directory name, which is not a valid Python\n# identifier, so it cannot be imported with a regular ``from .ask-user import``\n# statement. Use ``importlib`` instead.\nregister_ask_user_tool = importlib.import_module(\n    \"xy.ai.mcpc.tools.ask-user\"\n).register_ask_user_tool\n\n\ndef register_tools(registry: ToolRegistry) -> None:\n    \"\"\"Register all built-in file-system and shell tools onto *registry*.\"\"\"\n    register_read_tool(registry)\n    register_write_tool(registry)\n    register_insert_tool(registry)\n    register_change_tool(registry)\n    register_replace_chars_tool(registry)\n    register_replace_lines_tool(registry)\n    register_bash_tool(registry)\n    register_python_tool(registry)\n    register_markdown_tool(registry)\n    register_ask_user_tool(registry)\n    register_skills(registry)\n    register_exa_tools(registry)\n    register_github_tools(registry)\n    register_context7_tools(registry)\n\n\n# Keep the old name available so existing call sites don't break.\nregister_builtin_tools = register_tools\n\n__all__ = [\n    \"register_tools\",\n    \"register_builtin_tools\",\n]\n"
#:   }
#: }
/allow 1f836cb8-4ac5-42a9-8c6b-3b5ec411755e
#: Control Request:
#: {
#:   "id" : "effb5db4-52f4-431e-8f25-86563a58cd40",
#:   "phase" : "result",
#:   "toolName" : "write",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "result" : "success"
#:     }
#:   }
#: }
/allow effb5db4-52f4-431e-8f25-86563a58cd40
#: Tool: mcp__mcpc__read
#: path: "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py"
#: ReasoningToken: 80
#: Tool: mcp__mcpc__write
#: path: "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py"
#: mode: "replace"
#: content: "\"\"\"File-system and shell tools for the MCPC server.\n\nAvailable tools\n---------------\n* ``read``          – read …
#: ReasoningToken: 59
Fertig. Zusammenfassung:

- **`tools/ask-user/__init__.py`**: neues Tool `ask-user` mit Input `question` (Frage in Nutzersprache) und Output `answer`. Die Dummy-Implementierung liefert immer eine feste Antwort, dass der Nutzer nicht geantwortet hat und der Agent selbst weitermachen soll – das eigentliche Eingreifen des Nutzers erfolgt über das bestehende Permission-System bei Tool-Aufrufen/-Outputs.
- **`tools/__init__.py`**: Registrierung ergänzt. Da `ask-user` (mit Bindestrich) kein gültiger Python-Bezeichner ist, wird das Modul per `importlib.import_module("xy.ai.mcpc.tools.ask-user")` geladen statt per normalem `from .ask-user import ...`. Dabei ist mir versehentlich beim ersten Patch-Versuch ein Teil der bestehenden Imports verloren gegangen – ich habe die Datei danach vollständig neu geschrieben und alle ursprünglichen Imports/Registrierungen wiederhergestellt plus `register_ask_user_tool(registry)` ergänzt.