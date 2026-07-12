Implementiere analog zu `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py` ein "list" Tool, das Verzeichnishierarchien auflistet.

* Eingabe-Parameter sind der absolute Verzeichnispfad und eine optionale Regexp zum Filtern der Ergebnisliste.
* Zurückgegeben wird eine flache Liste von Strings mit dem absoluten Dateinamen aller Einträge von Dateien im aktuellen Verzeichnis und in allen Unterverzeichnissen
* Die Liste ist Alphabetisch sortiert
#: Control Request:
#: {
#:   "id" : "58064936-8822-4d68-99a5-0abd73b09e19",
#:   "phase" : "request",
#:   "toolName" : "bash",
#:   "arguments" : {
#:     "cwd" : "/home/user/xyan/xy.ai.workbench/mcpc",
#:     "script" : "find . -path ./node_modules -prune -o -name '__init__.py' -print | grep tools"
#:   }
#: }
/allow 58064936-8822-4d68-99a5-0abd73b09e19
#: Control Request:
#: {
#:   "id" : "4ed2cd0e-1c10-4f8a-a50d-bc306d8c2b69",
#:   "phase" : "result",
#:   "toolName" : "bash",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "exit_code" : 0,
#:       "stdout" : "./src/xy/ai/mcpc/tools/mcp/__init__.py\n./src/xy/ai/mcpc/tools/change/__init__.py\n./src/xy/ai/mcpc/tools/replace_chars/__init__.py\n./src/xy/ai/mcpc/tools/skills/__init__.py\n./src/xy/ai/mcpc/tools/agent/__init__.py\n./src/xy/ai/mcpc/tools/insert/__init__.py\n./src/xy/ai/mcpc/tools/bash/__init__.py\n./src/xy/ai/mcpc/tools/replace_lines/__init__.py\n./src/xy/ai/mcpc/tools/__init__.py\n./src/xy/ai/mcpc/tools/openalex/__init__.py\n./src/xy/ai/mcpc/tools/markdown/__init__.py\n./src/xy/ai/mcpc/tools/ask-user/__init__.py\n./src/xy/ai/mcpc/tools/python/__init__.py\n./src/xy/ai/mcpc/tools/write/__init__.py\n./src/xy/ai/mcpc/tools/read/__init__.py\n"
#:     }
#:   }
#: }
/allow 4ed2cd0e-1c10-4f8a-a50d-bc306d8c2b69
#: Control Request:
#: {
#:   "id" : "e95053fd-3dff-4f8c-aaf0-c7ab12428f47",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py"
#:   }
#: }
/allow e95053fd-3dff-4f8c-aaf0-c7ab12428f47
#: Control Request:
#: {
#:   "id" : "9bf4e1b0-6f87-40b0-bf32-0148423c8795",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "\"\"\"Read tool – returns file contents, optionally sliced by line or unique marker.\n\nRange: start = min_line | start-marker | file start; end = max_line | end-marker\n| file end (all inclusive). Markers must be unique substrings. Per-session\nsha256 cache rejects unchanged re-reads (key ``_read_cache`` in session state).\n\"\"\"\n\nfrom __future__ import annotations\n\nimport hashlib\nfrom pathlib import Path\nfrom typing import Any\n\nfrom ...registry import ToolContext, ToolRegistry, ToolResult\n\n#: Key used inside ``Session.state`` to persist the per-session file cache.\n_CACHE_KEY = \"_read_cache\"\n\n\ndef _sha256(data: bytes) -> str:\n    return hashlib.sha256(data).hexdigest()\n\n\ndef register_read_tool(registry: ToolRegistry) -> None:\n    @registry.tool(\n        \"read\",\n        title=\"Read file\",\n        description=(\n            \"Read a file as text, optionally sliced to a range. Repeated \"\n            \"unchanged reads return an error.\"\n        ),\n        input_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                \"path\": {\n                    \"type\": \"string\",\n                    \"description\": \"Absolute file path.\",\n                },\n                \"min_line\": {\n                    \"type\": \"integer\",\n                    \"description\": \"Range start: line number, inclusive, 1-based. Excludes start.\",\n                    \"minimum\": 1,\n                },\n                \"max_line\": {\n                    \"type\": \"integer\",\n                    \"description\": \"Range end: line number, inclusive, 1-based. Excludes end.\",\n                    \"minimum\": 1,\n                },\n                \"start\": {\n                    \"type\": \"string\",\n                    \"description\": \"Range start: unique marker substring, inclusive. Excludes min_line.\",\n                },\n                \"end\": {\n                    \"type\": \"string\",\n                    \"description\": \"Range end: unique marker substring, inclusive. Excludes max_line.\",\n                },\n            },\n            \"required\": [\"path\"],\n        },\n        output_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                #\"path\": {\"type\": \"string\"},\n                #\"sha256\": {\"type\": \"string\"},\n                #\"total_lines\": {\"type\": \"integer\"},\n                #\"returned_lines\": {\"type\": \"integer\"},\n                \"content\": {\"type\": \"string\"},\n                #\"min_line\": {\"type\": \"integer\"},\n                #\"max_line\": {\"type\": \"integer\"},\n            },\n            \"required\": [\"content\"\n                         #, \"path\", \"sha256\", \"total_lines\", \"returned_lines\"\n                         ],\n        },\n        annotations={\"readOnlyHint\": True, \"openWorldHint\": False},\n    )\n    def read(ctx: ToolContext) -> ToolResult:\n        args: dict[str, Any] = ctx.arguments\n        path_str: str = args[\"path\"]\n        min_line: int | None = args.get(\"min_line\")\n        max_line: int | None = args.get(\"max_line\")\n        start_marker: str | None = args.get(\"start\")\n        end_marker: str | None = args.get(\"end\")\n\n        # --- mutual exclusivity validation ---\n        if min_line is not None and start_marker is not None:\n            return ToolResult(\n                structured_content={\n                    \"error\": \"``min_line`` and ``start`` are mutually exclusive.\"\n                },\n                is_error=True,\n            )\n        if max_line is not None and end_marker is not None:\n            return ToolResult(\n                structured_content={\n                    \"error\": \"``max_line`` and ``end`` are mutually exclusive.\"\n                },\n                is_error=True,\n            )\n\n        path = Path(path_str)\n        if not path.is_absolute():\n            return ToolResult(\n                structured_content={\"error\": f\"Path must be absolute: {path_str}\"},\n                is_error=True,\n            )\n        if not path.exists():\n            return ToolResult(\n                structured_content={\"error\": f\"File not found: {path_str}\"},\n                is_error=True,\n            )\n        if not path.is_file():\n            return ToolResult(\n                structured_content={\"error\": f\"Not a regular file: {path_str}\"},\n                is_error=True,\n            )\n\n        raw_bytes = path.read_bytes()\n        current_hash = _sha256(raw_bytes)\n\n        # --- session cache check ---\n        cache: dict[str, str] = ctx.session.state.setdefault(_CACHE_KEY, {})\n        key = \"|\".join(\n            str(part)\n            for part in (\n                path.resolve(),\n                min_line,\n                max_line,\n                start_marker,\n                end_marker,\n            )\n        )\n        if cache.get(key) == current_hash:\n            return ToolResult(\n                structured_content={\n                    \"error\": f\"File has not changed since the last read. Use your context data instead!\"\n                },\n                is_error=True,\n            )\n        cache[key] = current_hash\n\n        # --- decode ---\n        text = raw_bytes.decode(\"utf-8\", errors=\"replace\")\n        lines = text.splitlines(keepends=True)\n        total_lines = len(lines)\n\n        def line_start_offset(line_num: int) -> int:\n            n = max(0, min(line_num - 1, total_lines))\n            return sum(len(l) for l in lines[:n])\n\n        def line_end_offset(line_num: int) -> int:\n            n = max(0, min(line_num, total_lines))\n            return sum(len(l) for l in lines[:n])\n\n        # --- resolve start boundary ---\n        if start_marker is not None:\n            start_count = text.count(start_marker)\n            if start_count == 0:\n                return ToolResult(\n                    structured_content={\"error\": f\"Start marker not found in file: {start_marker!r}\"},\n                    is_error=True,\n                )\n            if start_count > 1:\n                return ToolResult(\n                    structured_content={\n                        \"error\": (\n                            f\"Start marker is ambiguous – found {start_count} occurrences \"\n                            f\"in file: {start_marker!r}\"\n                        )\n                    },\n                    is_error=True,\n                )\n            region_start = text.index(start_marker)\n        elif min_line is not None:\n            region_start = line_start_offset(min_line)\n        else:\n            region_start = 0\n\n        # --- resolve end boundary ---\n        if end_marker is not None:\n            end_count = text.count(end_marker)\n            if end_count == 0:\n                return ToolResult(\n                    structured_content={\"error\": f\"End marker not found in file: {end_marker!r}\"},\n                    is_error=True,\n                )\n            if end_count > 1:\n                return ToolResult(\n                    structured_content={\n                        \"error\": (\n                            f\"End marker is ambiguous – found {end_count} occurrences \"\n                            f\"in file: {end_marker!r}\"\n                        )\n                    },\n                    is_error=True,\n                )\n            region_end = text.index(end_marker) + len(end_marker)\n        elif max_line is not None:\n            region_end = line_end_offset(max_line)\n        else:\n            region_end = len(text)\n\n        # --- order validation ---\n        if region_end < region_start:\n            return ToolResult(\n                structured_content={\n                    \"error\": (\n                        f\"Resolved end position ({region_end}) must not lie before \"\n                        f\"the resolved start position ({region_start}).\"\n                    )\n                },\n                is_error=True,\n            )\n\n        sliced = text[region_start:region_end]\n\n        structured: dict[str, Any] = {\"content\": sliced}\n        if min_line is not None:\n            structured[\"min_line\"] = min_line\n        if max_line is not None:\n            structured[\"max_line\"] = max_line\n        if start_marker is not None:\n            structured[\"start_line\"] = text.count(\"\\n\", 0, region_start) + 1\n        if end_marker is not None:\n            structured[\"end_line\"] = text.count(\"\\n\", 0, region_end) + 1\n\n        return ToolResult(structured_content=structured)\n"
#:     }
#:   }
#: }
/allow 9bf4e1b0-6f87-40b0-bf32-0148423c8795
#: Control Request:
#: {
#:   "id" : "6c000f3b-e30a-44f7-9cf3-b80463684d8f",
#:   "phase" : "request",
#:   "toolName" : "bash",
#:   "arguments" : {
#:     "cwd" : "/home/user/xyan/xy.ai.workbench/mcpc",
#:     "script" : "ls src/xy/ai/mcpc/tools/read/ && cat src/xy/ai/mcpc/tools/__init__.py"
#:   }
#: }
/allow 6c000f3b-e30a-44f7-9cf3-b80463684d8f
#: Control Request:
#: {
#:   "id" : "7f24f35a-7717-4b1f-a8dc-9e387eba1a9f",
#:   "phase" : "result",
#:   "toolName" : "bash",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "exit_code" : 0,
#:       "stdout" : "__init__.py\n__pycache__\n\"\"\"File-system and shell tools for the MCPC server.\n\nAvailable tools\n---------------\n* ``read``          – read a file (with session-level content-hash caching)\n* ``write``         – overwrite or append to a file\n* ``insert``        – insert text at a character offset\n* ``replace-chars`` – replace a character range with new text\n* ``replace-lines`` – replace a line range with new text\n* ``change``        – replace a delimited block identified by start/end markers\n* ``bash``          – run a Bash script in a given working directory\n* ``python``        – run a Python script directly from context\n* ``markdown``      – AST-based Markdown editing via a remark (Node.js) script\n* ``ask-user``      – ask the user a clarifying question (back-channel)\n\nSkills (on-demand hint tools) are registered from the ``skills`` sub-package.\nBridges to external MCP servers (e.g. Exa) live in the ``mcp`` sub-package.\nOpenAlex scholarly-search tools live in the ``openalex`` sub-package.\n\nCall :func:`register_tools` to register all tools onto a\n:class:`~xy.ai.mcpc.registry.ToolRegistry` instance.\n\"\"\"\n\nfrom __future__ import annotations\n\nimport importlib\n\nfrom ..registry import ToolRegistry\nfrom .bash import register_bash_tool\nfrom .change import register_change_tool\nfrom .insert import register_insert_tool\nfrom .markdown import register_markdown_tool\nfrom .mcp import register_context7_tools, register_exa_tools, register_github_tools\nfrom .openalex import register_openalex_tools\nfrom .python import register_python_tool\nfrom .read import register_read_tool\nfrom xy.ai.mcpc.tools.replace_chars import register_replace_chars_tool\nfrom .replace_lines import register_replace_lines_tool\nfrom .skills import register_skills\nfrom .write import register_write_tool\n\n# ``ask-user`` uses a hyphenated directory name, which is not a valid Python\n# identifier, so it cannot be imported with a regular ``from .ask-user import``\n# statement. Use ``importlib`` instead.\nregister_ask_user_tool = importlib.import_module(\n    \"xy.ai.mcpc.tools.ask-user\"\n).register_ask_user_tool\n\n\ndef register_tools(registry: ToolRegistry) -> None:\n    \"\"\"Register all built-in file-system and shell tools onto *registry*.\"\"\"\n    register_read_tool(registry)\n    register_write_tool(registry)\n    register_insert_tool(registry)\n    register_change_tool(registry)\n    register_replace_chars_tool(registry)\n    register_replace_lines_tool(registry)\n    register_bash_tool(registry)\n    register_python_tool(registry)\n    register_markdown_tool(registry)\n    register_ask_user_tool(registry)\n    register_skills(registry)\n    register_exa_tools(registry)\n    register_github_tools(registry)\n    register_context7_tools(registry)\n    register_openalex_tools(registry)\n\n\n# Keep the old name available so existing call sites don't break.\nregister_builtin_tools = register_tools\n\n__all__ = [\n    \"register_tools\",\n    \"register_builtin_tools\",\n]\n"
#:     }
#:   }
#: }
/allow 7f24f35a-7717-4b1f-a8dc-9e387eba1a9f
#: Control Request:
#: {
#:   "id" : "5443c21b-e8cd-47d8-b4f1-7e4b1ea9efaf",
#:   "phase" : "request",
#:   "toolName" : "bash",
#:   "arguments" : {
#:     "cwd" : "/home/user/xyan/xy.ai.workbench/mcpc",
#:     "script" : "mkdir -p src/xy/ai/mcpc/tools/list"
#:   }
#: }
/allow 5443c21b-e8cd-47d8-b4f1-7e4b1ea9efaf
#: Control Request:
#: {
#:   "id" : "e689dde5-71df-46df-a1a7-21b00def5358",
#:   "phase" : "result",
#:   "toolName" : "bash",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "exit_code" : 0,
#:       "stdout" : ""
#:     }
#:   }
#: }
/allow e689dde5-71df-46df-a1a7-21b00def5358
#: Control Request:
#: {
#:   "id" : "08ef12c2-85be-4746-9340-29ff52e2ba32",
#:   "phase" : "request",
#:   "toolName" : "write",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/list/__init__.py",
#:     "mode" : "replace",
#:     "content" : "\"\"\"List tool – returns a flat, sorted list of absolute file paths below a directory.\n\nWalks the given absolute directory recursively and returns all file paths\n(files only, no directories) as an alphabetically sorted flat list of\nabsolute path strings. An optional regular expression can be supplied to\nfilter the resulting list (matched against each absolute file path).\n\"\"\"\n\nfrom __future__ import annotations\n\nimport re\nfrom pathlib import Path\nfrom typing import Any\n\nfrom ...registry import ToolContext, ToolRegistry, ToolResult\n\n\ndef register_list_tool(registry: ToolRegistry) -> None:\n    @registry.tool(\n        \"list\",\n        title=\"List directory\",\n        description=(\n            \"List all files below an absolute directory path, recursively, \"\n            \"as a flat, alphabetically sorted list of absolute file paths. \"\n            \"Optionally filter the result with a regular expression.\"\n        ),\n        input_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                \"path\": {\n                    \"type\": \"string\",\n                    \"description\": \"Absolute directory path.\",\n                },\n                \"pattern\": {\n                    \"type\": \"string\",\n                    \"description\": (\n                        \"Optional regular expression used to filter the resulting \"\n                        \"list of absolute file paths.\"\n                    ),\n                },\n            },\n            \"required\": [\"path\"],\n        },\n        output_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                \"entries\": {\n                    \"type\": \"array\",\n                    \"items\": {\"type\": \"string\"},\n                },\n            },\n            \"required\": [\"entries\"],\n        },\n        annotations={\"readOnlyHint\": True, \"openWorldHint\": False},\n    )\n    def list_dir(ctx: ToolContext) -> ToolResult:\n        args: dict[str, Any] = ctx.arguments\n        path_str: str = args[\"path\"]\n        pattern: str | None = args.get(\"pattern\")\n\n        path = Path(path_str)\n        if not path.is_absolute():\n            return ToolResult(\n                structured_content={\"error\": f\"Path must be absolute: {path_str}\"},\n                is_error=True,\n            )\n        if not path.exists():\n            return ToolResult(\n                structured_content={\"error\": f\"Directory not found: {path_str}\"},\n                is_error=True,\n            )\n        if not path.is_dir():\n            return ToolResult(\n                structured_content={\"error\": f\"Not a directory: {path_str}\"},\n                is_error=True,\n            )\n\n        regex: re.Pattern[str] | None = None\n        if pattern is not None:\n            try:\n                regex = re.compile(pattern)\n            except re.error as exc:\n                return ToolResult(\n                    structured_content={\"error\": f\"Invalid regular expression: {exc}\"},\n                    is_error=True,\n                )\n\n        entries: list[str] = []\n        for file_path in path.rglob(\"*\"):\n            if not file_path.is_file():\n                continue\n            abs_path = str(file_path.resolve())\n            if regex is not None and not regex.search(abs_path):\n                continue\n            entries.append(abs_path)\n\n        entries.sort()\n\n        return ToolResult(structured_content={\"entries\": entries})\n"
#:   }
#: }
/allow 08ef12c2-85be-4746-9340-29ff52e2ba32
#: Control Request:
#: {
#:   "id" : "40e52499-2476-4951-adc7-0c0b31f7fc05",
#:   "phase" : "request",
#:   "toolName" : "change",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py",
#:     "start" : "from .insert import register_insert_tool",
#:     "end" : "from .markdown import register_markdown_tool",
#:     "content" : "from .insert import register_insert_tool\nfrom .list import register_list_tool\nfrom .markdown import register_markdown_tool"
#:   }
#: }
/allow 40e52499-2476-4951-adc7-0c0b31f7fc05
#: Control Request:
#: {
#:   "id" : "f50532ca-ac7b-4e5e-99d6-6b29406a7d14",
#:   "phase" : "request",
#:   "toolName" : "change",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py",
#:     "start" : "    register_read_tool(registry)\n    register_write_tool(registry)",
#:     "end" : "    register_write_tool(registry)",
#:     "content" : "    register_read_tool(registry)\n    register_list_tool(registry)\n    register_write_tool(registry)"
#:   }
#: }
/allow f50532ca-ac7b-4e5e-99d6-6b29406a7d14
#: Control Request:
#: {
#:   "id" : "6f407f91-2da7-4328-8c0b-a5c453cfac24",
#:   "phase" : "request",
#:   "toolName" : "change",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/__init__.py",
#:     "start" : "* ``read``          – read a file (with session-level content-hash caching)",
#:     "end" : "* ``write``         – overwrite or append to a file",
#:     "content" : "* ``read``          – read a file (with session-level content-hash caching)\n* ``list``          – recursively list files below a directory\n* ``write``         – overwrite or append to a file"
#:   }
#: }
/allow 6f407f91-2da7-4328-8c0b-a5c453cfac24
#: Control Request:
#: {
#:   "id" : "aa64a0b2-b8e4-439d-9e21-7a5a286d2683",
#:   "phase" : "request",
#:   "toolName" : "bash",
#:   "arguments" : {
#:     "cwd" : "/home/user/xyan/xy.ai.workbench/mcpc",
#:     "script" : "find . -path ./node_modules -prune -o -iname '*test*' -path '*read*' -print"
#:   }
#: }
/allow aa64a0b2-b8e4-439d-9e21-7a5a286d2683
#: Control Request:
#: {
#:   "id" : "72c53a82-4ebf-4e31-8b56-96a1f260bf3e",
#:   "phase" : "result",
#:   "toolName" : "bash",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "exit_code" : 0,
#:       "stdout" : ""
#:     }
#:   }
#: }
/allow 72c53a82-4ebf-4e31-8b56-96a1f260bf3e
#: Control Request:
#: {
#:   "id" : "c7855025-f7ce-4173-a9e5-512419be8a3c",
#:   "phase" : "request",
#:   "toolName" : "bash",
#:   "arguments" : {
#:     "cwd" : "/home/user/xyan/xy.ai.workbench/mcpc",
#:     "script" : "find . -path ./node_modules -prune -o -iname '*test*' -print | head -50"
#:   }
#: }
/allow c7855025-f7ce-4173-a9e5-512419be8a3c
#: Control Request:
#: {
#:   "id" : "44fa04b9-9da9-4ecb-88cb-91026ed4ea6c",
#:   "phase" : "result",
#:   "toolName" : "bash",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "exit_code" : 0,
#:       "stdout" : "./.pytest_cache\n"
#:     }
#:   }
#: }
/allow 44fa04b9-9da9-4ecb-88cb-91026ed4ea6c
#: Control Request:
#: {
#:   "id" : "054c8eb8-12ce-462c-9875-823ff4e51ab9",
#:   "phase" : "request",
#:   "toolName" : "bash",
#:   "arguments" : {
#:     "cwd" : "/home/user/xyan/xy.ai.workbench/mcpc",
#:     "script" : "cd /home/user/xyan/xy.ai.workbench/mcpc && python -c \"\nimport sys\nsys.path.insert(0, 'src')\nfrom xy.ai.mcpc.tools.list import register_list_tool\nprint('import ok')\n\"\n"
#:   }
#: }
/allow 054c8eb8-12ce-462c-9875-823ff4e51ab9
#: Control Request:
#: {
#:   "id" : "b400e964-9aa4-4e16-aec4-39852e6402b9",
#:   "phase" : "result",
#:   "toolName" : "bash",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "exit_code" : 0,
#:       "stdout" : "import ok\n"
#:     }
#:   }
#: }
/allow b400e964-9aa4-4e16-aec4-39852e6402b9
#: Control Request:
#: {
#:   "id" : "9e7f960a-3571-4b2e-a6fd-82615994c62e",
#:   "phase" : "request",
#:   "toolName" : "bash",
#:   "arguments" : {
#:     "cwd" : "/home/user/xyan/xy.ai.workbench/mcpc",
#:     "script" : "cd /home/user/xyan/xy.ai.workbench/mcpc && python -c \"\nimport sys\nsys.path.insert(0, 'src')\nfrom xy.ai.mcpc.tools import register_tools\nprint('import ok')\n\" 2>&1 | tail -30\n"
#:   }
#: }
/allow 9e7f960a-3571-4b2e-a6fd-82615994c62e
#: Control Request:
#: {
#:   "id" : "119e2d55-6fa5-4452-8ded-533b0f27eb9c",
#:   "phase" : "result",
#:   "toolName" : "bash",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "exit_code" : 0,
#:       "stdout" : "import ok\n"
#:     }
#:   }
#: }
/allow 119e2d55-6fa5-4452-8ded-533b0f27eb9c
#: ReasoningToken: 22
#: ReasoningToken: 24
Beide Imports funktionieren fehlerfrei. Zusammenfassung:

- Neues Tool `list` unter `src/xy/ai/mcpc/tools/list/__init__.py` erstellt, analog zum `read`-Tool aufgebaut (gleiche Struktur mit `ToolContext`/`ToolResult`, Validierung von absolutem Pfad, Fehlerbehandlung).
- Es nimmt einen absoluten Verzeichnispfad (`path`, required) und ein optionales `pattern` (Regexp) entgegen.
- Rekursives Durchlaufen via `Path.rglob("*")`, dabei werden nur Dateien (`is_file()`) berücksichtigt, keine Verzeichniseinträge.
- Ergebnis: `entries` – flache Liste absoluter Pfade, alphabetisch sortiert, optional per Regexp gefiltert.
- Tool in `tools/__init__.py` registriert (Import + Aufruf in `register_tools`) und in der Modul-Dokumentation ergänzt.