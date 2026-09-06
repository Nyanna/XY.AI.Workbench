Erweitere das Read-Tool `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py` um die Fähigkeit und Parameter die Offsets auf Basis von Marker Strings zu ermitteln analog der Implementierung in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/change/__init__.py`
Das ganze dient der besseren Unterstützung von semantischen Block-Operationen.

* Dasselbe Fehlerverhalten wie das Change-Tool
#: Control Request:
 {
   "id" : "87dc3180-5828-42e0-bc28-4d9fba704510",
   "phase" : "result",
   "toolName" : "read",
   "result" : {
     "content" : [ ],
     "structuredContent" : {
       "error" : "Das ist ein Verzeichnis keine Datei"
     },
     "isError" : true
   }
 }
/allow 87dc3180-5828-42e0-bc28-4d9fba704510
#: Control Request:
#: {
#:   "id" : "6b21e140-7762-468e-850a-63bb156cc231",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py"
#:   }
#: }
/allow 6b21e140-7762-468e-850a-63bb156cc231
#: Control Request:
#: {
#:   "id" : "208355d3-e9f4-4cea-9956-8f147b52ad7e",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "\"\"\"Read tool – reads a file and returns its contents.\n\nFeatures\n--------\n* Optional line-range restriction (``min_line`` / ``max_line``, 1-based inclusive).\n* Content-hash caching per session: if the client requests the same file again\n  and the file on disk has not changed, an error is returned to avoid redundant\n  transfers.\n* The cache is stored under the ``_read_cache`` key in the session's ``state``\n  dict as ``{absolute_path: sha256_hex}``.\n\"\"\"\n\nfrom __future__ import annotations\n\nimport hashlib\nfrom pathlib import Path\nfrom typing import Any\n\nfrom ...registry import ToolContext, ToolRegistry, ToolResult\n\n#: Key used inside ``Session.state`` to persist the per-session file cache.\n_CACHE_KEY = \"_read_cache\"\n\n\ndef _sha256(data: bytes) -> str:\n    return hashlib.sha256(data).hexdigest()\n\n\ndef register_read_tool(registry: ToolRegistry) -> None:\n    @registry.tool(\n        \"read\",\n        title=\"Read file\",\n        description=(\n            \"Read the contents of a file and return them as text. \"\n            \"Optionally restrict the result to a line range (1-based, inclusive). \"\n            \"Results are cached per session by content hash; if the file has not \"\n            \"changed since the last read an error is returned indicating so.\"\n        ),\n        input_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                \"path\": {\n                    \"type\": \"string\",\n                    \"description\": \"Absolute path to the file to read.\",\n                },\n                \"min_line\": {\n                    \"type\": \"integer\",\n                    \"description\": \"First line to return (1-based, inclusive). Omit to start from the beginning.\",\n                    \"minimum\": 1,\n                },\n                \"max_line\": {\n                    \"type\": \"integer\",\n                    \"description\": \"Last line to return (1-based, inclusive). Omit to read to the end of the file.\",\n                    \"minimum\": 1,\n                },\n            },\n            \"required\": [\"path\"],\n        },\n        output_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                #\"path\": {\"type\": \"string\"},\n                #\"sha256\": {\"type\": \"string\"},\n                #\"total_lines\": {\"type\": \"integer\"},\n                #\"returned_lines\": {\"type\": \"integer\"},\n                \"content\": {\"type\": \"string\"},\n                #\"min_line\": {\"type\": \"integer\"},\n                #\"max_line\": {\"type\": \"integer\"},\n            },\n            \"required\": [\"content\"\n                         #, \"path\", \"sha256\", \"total_lines\", \"returned_lines\"\n                         ],\n        },\n        annotations={\"readOnlyHint\": True, \"openWorldHint\": False},\n    )\n    def read(ctx: ToolContext) -> ToolResult:\n        args: dict[str, Any] = ctx.arguments\n        path_str: str = args[\"path\"]\n        min_line: int | None = args.get(\"min_line\")\n        max_line: int | None = args.get(\"max_line\")\n\n        path = Path(path_str)\n        if not path.is_absolute():\n            return ToolResult(\n                structured_content={\"error\": f\"Path must be absolute: {path_str}\"},\n                is_error=True,\n            )\n        if not path.exists():\n            return ToolResult(\n                structured_content={\"error\": f\"File not found: {path_str}\"},\n                is_error=True,\n            )\n        if not path.is_file():\n            return ToolResult(\n                structured_content={\"error\": f\"Not a regular file: {path_str}\"},\n                is_error=True,\n            )\n\n        raw_bytes = path.read_bytes()\n        current_hash = _sha256(raw_bytes)\n\n        # --- session cache check ---\n        cache: dict[str, str] = ctx.session.state.setdefault(_CACHE_KEY, {})\n        key = str(path.resolve())\n        if cache.get(key) == current_hash:\n            return ToolResult(\n                structured_content={\n                    \"error\": f\"File has not changed since the last read (sha256={current_hash}): {path_str}\"\n                },\n                is_error=True,\n            )\n        cache[key] = current_hash\n\n        # --- decode and slice ---\n        text = raw_bytes.decode(\"utf-8\", errors=\"replace\")\n        lines = text.splitlines(keepends=True)\n        total_lines = len(lines)\n\n        lo = (min_line - 1) if min_line is not None else 0\n        hi = max_line if max_line is not None else total_lines\n        lo = max(0, lo)\n        hi = min(total_lines, hi)\n\n        sliced = \"\".join(lines[lo:hi])\n\n        structured: dict[str, Any] = {\n            #\"path\": key,\n            #\"sha256\": current_hash,\n            #\"total_lines\": total_lines,\n            #\"returned_lines\": hi - lo,\n            \"content\": sliced,\n        }\n        if min_line is not None:\n            structured[\"min_line\"] = min_line\n        if max_line is not None:\n            structured[\"max_line\"] = max_line\n\n        return ToolResult(structured_content=structured)\n"
#:     }
#:   }
#: }
/allow 208355d3-e9f4-4cea-9956-8f147b52ad7e
#: Control Request:
#: {
#:   "id" : "c1cb79b1-3dd9-48f2-9f78-6a62bdc1ef1d",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/change/__init__.py"
#:   }
#: }
/allow c1cb79b1-3dd9-48f2-9f78-6a62bdc1ef1d
#: Control Request:
#: {
#:   "id" : "783e5f0f-93b8-4574-bcc6-15d43e7b3908",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "\"\"\"Change tool – replaces a block inside a file delimited by start/end markers.\n\nModes\n-----\n``include_start``\n    The replaced region starts **at** the start marker (inclusive) and ends\n    **before** the end marker (exclusive).  The end marker is preserved in the\n    output.\n\n``include_end``\n    The replaced region starts **after** the start marker (exclusive) and ends\n    **at** the end of the end marker (inclusive).  The start marker is\n    preserved in the output.\n\n``full``\n    Both markers are included in the replaced region (both inclusive).\n\nError conditions\n----------------\n* Path is not absolute, does not exist or is not a regular file.\n* A marker string does not occur in the file at all.\n* A marker string occurs more than once (ambiguous); the exact count is\n  reported so the caller can supply a more specific marker.\n* The first occurrence of the end marker lies before (or at the same position\n  as) the first occurrence of the start marker.\n\"\"\"\n\nfrom __future__ import annotations\n\nfrom pathlib import Path\nfrom typing import Any\n\nfrom ...registry import ToolContext, ToolRegistry, ToolResult\n\n\ndef register_change_tool(registry: ToolRegistry) -> None:\n    @registry.tool(\n        \"change\",\n        title=\"Change file block\",\n        description=(\n            \"Replace a block of text inside a file that is delimited by a \"\n            \"start string and an end string.  The ``mode`` parameter controls \"\n            \"whether each marker is itself included in or excluded from the \"\n            \"replaced region:\\n\"\n            \"* ``include_start`` – replace from the start marker (inclusive) \"\n            \"to just before the end marker (exclusive); the end marker is kept.\\n\"\n            \"* ``include_end`` – replace from just after the start marker \"\n            \"(exclusive) to the end of the end marker (inclusive); the start \"\n            \"marker is kept.\\n\"\n            \"* ``full`` – replace the entire range including both markers.\\n\\n\"\n            \"Both markers must appear exactly once in the file and the end \"\n            \"marker must follow the start marker.  A precise error is returned \"\n            \"otherwise.\"\n        ),\n        input_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                \"path\": {\n                    \"type\": \"string\",\n                    \"description\": \"Absolute path to the target file.\",\n                },\n                \"mode\": {\n                    \"type\": \"string\",\n                    \"enum\": [\"include_start\", \"include_end\", \"full\"],\n                    \"description\": (\n                        \"``include_start`` – start marker inclusive, end marker exclusive. \"\n                        \"``include_end`` – start marker exclusive, end marker inclusive. \"\n                        \"``full`` – both markers inclusive.\"\n                    ),\n                },\n                \"start\": {\n                    \"type\": \"string\",\n                    \"description\": \"Exact string that marks the beginning of the block.\",\n                },\n                \"end\": {\n                    \"type\": \"string\",\n                    \"description\": \"Exact string that marks the end of the block.\",\n                },\n                \"content\": {\n                    \"type\": \"string\",\n                    \"description\": \"Replacement text that will be written in place of the matched block.\",\n                },\n            },\n            \"required\": [\"path\", \"mode\", \"start\", \"end\", \"content\"],\n        },\n        output_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                \"result\": {\n                    \"type\": \"string\",\n                    \"description\": \"``success`` on success.\",\n                },\n                \"error\": {\n                    \"type\": \"string\",\n                    \"description\": \"Human-readable error message (only present when is_error is true).\",\n                },\n            },\n            \"required\": [],\n        },\n        annotations={\"readOnlyHint\": False, \"idempotentHint\": False, \"openWorldHint\": False},\n    )\n    def change(ctx: ToolContext) -> ToolResult:\n        args: dict[str, Any] = ctx.arguments\n        path_str: str = args[\"path\"]\n        mode: str = args[\"mode\"]\n        start_marker: str = args[\"start\"]\n        end_marker: str = args[\"end\"]\n        new_content: str = args[\"content\"]\n\n        # --- path validation ---\n        path = Path(path_str)\n        if not path.is_absolute():\n            return ToolResult(\n                structured_content={\"error\": f\"Path must be absolute: {path_str}\"},\n                is_error=True,\n            )\n        if not path.exists():\n            return ToolResult(\n                structured_content={\"error\": f\"File not found: {path_str}\"},\n                is_error=True,\n            )\n        if not path.is_file():\n            return ToolResult(\n                structured_content={\"error\": f\"Not a regular file: {path_str}\"},\n                is_error=True,\n            )\n\n        text = path.read_text(encoding=\"utf-8\")\n\n        # --- locate and validate start marker ---\n        start_count = text.count(start_marker)\n        if start_count == 0:\n            return ToolResult(\n                structured_content={\"error\": f\"Start marker not found in file: {start_marker!r}\"},\n                is_error=True,\n            )\n        if start_count > 1:\n            return ToolResult(\n                structured_content={\n                    \"error\": (\n                        f\"Start marker is ambiguous – found {start_count} occurrences \"\n                        f\"in file: {start_marker!r}\"\n                    )\n                },\n                is_error=True,\n            )\n\n        # --- locate and validate end marker ---\n        end_count = text.count(end_marker)\n        if end_count == 0:\n            return ToolResult(\n                structured_content={\"error\": f\"End marker not found in file: {end_marker!r}\"},\n                is_error=True,\n            )\n        if end_count > 1:\n            return ToolResult(\n                structured_content={\n                    \"error\": (\n                        f\"End marker is ambiguous – found {end_count} occurrences \"\n                        f\"in file: {end_marker!r}\"\n                    )\n                },\n                is_error=True,\n            )\n\n        start_pos = text.index(start_marker)\n        end_pos = text.index(end_marker)\n\n        # --- order validation ---\n        if end_pos <= start_pos:\n            return ToolResult(\n                structured_content={\n                    \"error\": (\n                        f\"End marker must appear after start marker, but end marker \"\n                        f\"starts at position {end_pos} while start marker starts at \"\n                        f\"position {start_pos}.\"\n                    )\n                },\n                is_error=True,\n            )\n\n        # --- apply replacement based on mode ---\n        if mode == \"include_start\":\n            # replace from start_pos (inclusive) to end_pos (exclusive)\n            result_text = text[:start_pos] + new_content + text[end_pos:]\n        elif mode == \"include_end\":\n            # replace from start_pos + len(start) (exclusive) to end_pos + len(end) (inclusive)\n            result_text = text[:start_pos + len(start_marker)] + new_content + text[end_pos + len(end_marker):]\n        else:  # full\n            # replace from start_pos (inclusive) to end_pos + len(end) (inclusive)\n            result_text = text[:start_pos] + new_content + text[end_pos + len(end_marker):]\n\n        # --- write back ---\n        try:\n            path.write_text(result_text, encoding=\"utf-8\")\n        except OSError as exc:\n            return ToolResult(\n                structured_content={\"error\": f\"Write failed: {exc}\"},\n                is_error=True,\n            )\n\n        return ToolResult(structured_content={\"result\": \"success\"})\n"
#:     }
#:   }
#: }
/allow 783e5f0f-93b8-4574-bcc6-15d43e7b3908
#: Control Request:
#: {
#:   "id" : "63d6bbac-01d1-471a-bc7b-18ebaf764fa2",
#:   "phase" : "request",
#:   "toolName" : "write",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py",
#:     "mode" : "replace",
#:     "content" : "\"\"\"Read tool – reads a file and returns its contents.\n\nFeatures\n--------\n* Optional line-range restriction (``min_line`` / ``max_line``, 1-based inclusive).\n* Optional marker-based range restriction (``mode`` / ``start`` / ``end``),\n  analogous to the ``change`` tool's block-selection logic. This allows\n  semantic block operations: instead of specifying line numbers, the caller\n  supplies a start marker string and an end marker string, and a ``mode``\n  that controls whether each marker is itself included in the returned\n  region:\n\n  * ``include_start`` – region starts **at** the start marker (inclusive)\n    and ends **before** the end marker (exclusive).\n  * ``include_end`` – region starts **after** the start marker (exclusive)\n    and ends **at** the end of the end marker (inclusive).\n  * ``full`` – both markers are included in the returned region.\n\n  Both markers must appear exactly once in the file and the end marker must\n  follow the start marker. A precise error is returned otherwise (same\n  error behaviour as the ``change`` tool). Marker-based selection cannot be\n  combined with ``min_line`` / ``max_line``.\n* Content-hash caching per session: if the client requests the same file\n  again and the file on disk has not changed, an error is returned to avoid\n  redundant transfers.\n* The cache is stored under the ``_read_cache`` key in the session's\n  ``state`` dict as ``{absolute_path: sha256_hex}``.\n\"\"\"\n\nfrom __future__ import annotations\n\nimport hashlib\nfrom pathlib import Path\nfrom typing import Any\n\nfrom ...registry import ToolContext, ToolRegistry, ToolResult\n\n#: Key used inside ``Session.state`` to persist the per-session file cache.\n_CACHE_KEY = \"_read_cache\"\n\n\ndef _sha256(data: bytes) -> str:\n    return hashlib.sha256(data).hexdigest()\n\n\ndef register_read_tool(registry: ToolRegistry) -> None:\n    @registry.tool(\n        \"read\",\n        title=\"Read file\",\n        description=(\n            \"Read the contents of a file and return them as text. \"\n            \"Optionally restrict the result to a line range (1-based, inclusive) \"\n            \"using ``min_line`` / ``max_line``, or to a marker-delimited block \"\n            \"using ``mode`` / ``start`` / ``end`` (analogous to the ``change`` \"\n            \"tool). Results are cached per session by content hash; if the file \"\n            \"has not changed since the last read an error is returned \"\n            \"indicating so.\"\n        ),\n        input_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                \"path\": {\n                    \"type\": \"string\",\n                    \"description\": \"Absolute path to the file to read.\",\n                },\n                \"min_line\": {\n                    \"type\": \"integer\",\n                    \"description\": \"First line to return (1-based, inclusive). Omit to start from the beginning.\",\n                    \"minimum\": 1,\n                },\n                \"max_line\": {\n                    \"type\": \"integer\",\n                    \"description\": \"Last line to return (1-based, inclusive). Omit to read to the end of the file.\",\n                    \"minimum\": 1,\n                },\n                \"mode\": {\n                    \"type\": \"string\",\n                    \"enum\": [\"include_start\", \"include_end\", \"full\"],\n                    \"description\": (\n                        \"Selects the marker-based region to return (must be used \"\n                        \"together with ``start`` and ``end``, and cannot be combined \"\n                        \"with ``min_line`` / ``max_line``). \"\n                        \"``include_start`` – start marker inclusive, end marker exclusive. \"\n                        \"``include_end`` – start marker exclusive, end marker inclusive. \"\n                        \"``full`` – both markers inclusive.\"\n                    ),\n                },\n                \"start\": {\n                    \"type\": \"string\",\n                    \"description\": \"Exact string that marks the beginning of the block to return.\",\n                },\n                \"end\": {\n                    \"type\": \"string\",\n                    \"description\": \"Exact string that marks the end of the block to return.\",\n                },\n            },\n            \"required\": [\"path\"],\n        },\n        output_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                #\"path\": {\"type\": \"string\"},\n                #\"sha256\": {\"type\": \"string\"},\n                #\"total_lines\": {\"type\": \"integer\"},\n                #\"returned_lines\": {\"type\": \"integer\"},\n                \"content\": {\"type\": \"string\"},\n                #\"min_line\": {\"type\": \"integer\"},\n                #\"max_line\": {\"type\": \"integer\"},\n            },\n            \"required\": [\"content\"\n                         #, \"path\", \"sha256\", \"total_lines\", \"returned_lines\"\n                         ],\n        },\n        annotations={\"readOnlyHint\": True, \"openWorldHint\": False},\n    )\n    def read(ctx: ToolContext) -> ToolResult:\n        args: dict[str, Any] = ctx.arguments\n        path_str: str = args[\"path\"]\n        min_line: int | None = args.get(\"min_line\")\n        max_line: int | None = args.get(\"max_line\")\n        marker_mode: str | None = args.get(\"mode\")\n        start_marker: str | None = args.get(\"start\")\n        end_marker: str | None = args.get(\"end\")\n\n        # --- marker parameter validation ---\n        marker_params = (marker_mode, start_marker, end_marker)\n        markers_given = any(p is not None for p in marker_params)\n        if markers_given and not all(p is not None for p in marker_params):\n            return ToolResult(\n                structured_content={\n                    \"error\": \"``mode``, ``start`` and ``end`` must be provided together.\"\n                },\n                is_error=True,\n            )\n        if markers_given and (min_line is not None or max_line is not None):\n            return ToolResult(\n                structured_content={\n                    \"error\": (\n                        \"Marker-based selection (``mode``/``start``/``end``) cannot \"\n                        \"be combined with ``min_line``/``max_line``.\"\n                    )\n                },\n                is_error=True,\n            )\n\n        path = Path(path_str)\n        if not path.is_absolute():\n            return ToolResult(\n                structured_content={\"error\": f\"Path must be absolute: {path_str}\"},\n                is_error=True,\n            )\n        if not path.exists():\n            return ToolResult(\n                structured_content={\"error\": f\"File not found: {path_str}\"},\n                is_error=True,\n            )\n        if not path.is_file():\n            return ToolResult(\n                structured_content={\"error\": f\"Not a regular file: {path_str}\"},\n                is_error=True,\n            )\n\n        raw_bytes = path.read_bytes()\n        current_hash = _sha256(raw_bytes)\n\n        # --- session cache check ---\n        cache: dict[str, str] = ctx.session.state.setdefault(_CACHE_KEY, {})\n        key = str(path.resolve())\n        if cache.get(key) == current_hash:\n            return ToolResult(\n                structured_content={\n                    \"error\": f\"File has not changed since the last read (sha256={current_hash}): {path_str}\"\n                },\n                is_error=True,\n            )\n        cache[key] = current_hash\n\n        # --- decode ---\n        text = raw_bytes.decode(\"utf-8\", errors=\"replace\")\n\n        structured: dict[str, Any] = {}\n\n        if markers_given:\n            # --- locate and validate start marker ---\n            start_count = text.count(start_marker)\n            if start_count == 0:\n                return ToolResult(\n                    structured_content={\"error\": f\"Start marker not found in file: {start_marker!r}\"},\n                    is_error=True,\n                )\n            if start_count > 1:\n                return ToolResult(\n                    structured_content={\n                        \"error\": (\n                            f\"Start marker is ambiguous – found {start_count} occurrences \"\n                            f\"in file: {start_marker!r}\"\n                        )\n                    },\n                    is_error=True,\n                )\n\n            # --- locate and validate end marker ---\n            end_count = text.count(end_marker)\n            if end_count == 0:\n                return ToolResult(\n                    structured_content={\"error\": f\"End marker not found in file: {end_marker!r}\"},\n                    is_error=True,\n                )\n            if end_count > 1:\n                return ToolResult(\n                    structured_content={\n                        \"error\": (\n                            f\"End marker is ambiguous – found {end_count} occurrences \"\n                            f\"in file: {end_marker!r}\"\n                        )\n                    },\n                    is_error=True,\n                )\n\n            start_pos = text.index(start_marker)\n            end_pos = text.index(end_marker)\n\n            # --- order validation ---\n            if end_pos <= start_pos:\n                return ToolResult(\n                    structured_content={\n                        \"error\": (\n                            f\"End marker must appear after start marker, but end marker \"\n                            f\"starts at position {end_pos} while start marker starts at \"\n                            f\"position {start_pos}.\"\n                        )\n                    },\n                    is_error=True,\n                )\n\n            # --- resolve region based on mode ---\n            if marker_mode == \"include_start\":\n                region_start = start_pos\n                region_end = end_pos\n            elif marker_mode == \"include_end\":\n                region_start = start_pos + len(start_marker)\n                region_end = end_pos + len(end_marker)\n            else:  # full\n                region_start = start_pos\n                region_end = end_pos + len(end_marker)\n\n            sliced = text[region_start:region_end]\n            structured[\"content\"] = sliced\n            structured[\"start_line\"] = text.count(\"\\n\", 0, region_start) + 1\n            structured[\"end_line\"] = text.count(\"\\n\", 0, region_end) + 1\n        else:\n            lines = text.splitlines(keepends=True)\n            total_lines = len(lines)\n\n            lo = (min_line - 1) if min_line is not None else 0\n            hi = max_line if max_line is not None else total_lines\n            lo = max(0, lo)\n            hi = min(total_lines, hi)\n\n            sliced = \"\".join(lines[lo:hi])\n\n            structured[\"content\"] = sliced\n            if min_line is not None:\n                structured[\"min_line\"] = min_line\n            if max_line is not None:\n                structured[\"max_line\"] = max_line\n\n        return ToolResult(structured_content=structured)\n"
#:   }
#: }
/allow 63d6bbac-01d1-471a-bc7b-18ebaf764fa2
#: Control Request:
#: {
#:   "id" : "13546586-1ca7-446b-82de-bc9e0ea3e50c",
#:   "phase" : "result",
#:   "toolName" : "write",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "result" : "success"
#:     }
#:   }
#: }
/allow 13546586-1ca7-446b-82de-bc9e0ea3e50c
#: Control Request:
#: {
#:   "id" : "c36e1609-94b1-4e24-8d8a-3125b78f0f24",
#:   "phase" : "request",
#:   "toolName" : "bash",
#:   "arguments" : {
#:     "cwd" : "/home/user/xyan/xy.ai.workbench/mcpc",
#:     "script" : "python3 -m py_compile src/xy/ai/mcpc/tools/read/__init__.py && echo OK"
#:   }
#: }
/allow c36e1609-94b1-4e24-8d8a-3125b78f0f24
#: Control Request:
#: {
#:   "id" : "daf156f5-9464-45a0-95f1-f58afb8e1f99",
#:   "phase" : "result",
#:   "toolName" : "bash",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "exit_code" : 0,
#:       "stdout" : "OK\n"
#:     }
#:   }
#: }
/allow daf156f5-9464-45a0-95f1-f58afb8e1f99
#: Tool: mcp__mcpc__write
#: path: "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py"
#: mode: "replace"
#: content: "\"\"\"Read tool – reads a file and returns its contents.\n\nFeatures\n--------\n* Optional line-range restriction (``mi…
#: ReasoningToken: 1402
#: Tool: mcp__mcpc__bash
#: cwd: "/home/user/xyan/xy.ai.workbench/mcpc"
#: script: "python3 -m py_compile src/xy/ai/mcpc/tools/read/__init__.py && echo OK"
#: ReasoningToken: 22
Die Datei kompiliert fehlerfrei.

## Zusammenfassung

Ich habe das Read-Tool in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py` um marker-basierte Offset-Ermittlung erweitert, analog zum Change-Tool:

**Neue Parameter** (optional, müssen zusammen angegeben werden):
- `mode` (`include_start` | `include_end` | `full`)
- `start` (Start-Marker-String)
- `end` (End-Marker-String)

**Semantik** identisch zum Change-Tool:
- `include_start` – Region beginnt **bei** dem Start-Marker (inklusive), endet **vor** dem End-Marker (exklusive).
- `include_end` – Region beginnt **nach** dem Start-Marker (exklusive), endet **am Ende** des End-Markers (inklusive).
- `full` – beide Marker sind inklusive.

**Fehlerverhalten** identisch zum Change-Tool:
- Marker nicht gefunden → Fehler
- Marker mehrfach vorhanden (mehrdeutig) → Fehler mit Anzahl der Treffer
- End-Marker liegt vor/gleich Start-Marker → Fehler
- Zusätzlich: `mode`/`start`/`end` müssen gemeinsam angegeben werden, und können **nicht** mit `min_line`/`max_line` kombiniert werden (sonst Fehler).

**Output-Erweiterung**: Bei marker-basiertem Read werden `start_line` und `end_line` (1-based) der getroffenen Region zurückgegeben, damit der Aufrufer die Position für nachfolgende Operationen kennt.

Der bestehende Line-basierte Modus (`min_line`/`max_line`) sowie das Session-Cache-Verhalten (Hash-basierte Duplikatserkennung) bleiben unverändert erhalten.

Entferne den Modus und setze ihn fix auf "Full"-Verhalten. Die Parameter sind beliebig kombinierbar, eine start-line oder start-marker oder null und eine end-line oder end-marker oder null. 
#: Control Request:
#: {
#:   "id" : "ba8311bf-c5be-4b58-92bb-dc051a189baf",
#:   "phase" : "request",
#:   "toolName" : "write",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py",
#:     "mode" : "replace",
#:     "content" : "\"\"\"Read tool – reads a file and returns its contents.\n\nFeatures\n--------\n* Optional range restriction. The start of the returned region can be given\n  either as a line number (``min_line``, 1-based inclusive) or as a marker\n  string (``start``) that must occur exactly once in the file; if neither is\n  given, the region starts at the beginning of the file. The end of the\n  returned region works analogously via ``max_line`` / ``end``, defaulting to\n  the end of the file if neither is given. ``min_line``/``start`` and\n  ``max_line``/``end`` can be combined arbitrarily (e.g. a line-based start\n  with a marker-based end).\n\n  Marker-based boundaries always behave like the ``change`` tool's ``full``\n  mode: the marker itself is included in the returned region (``start`` marks\n  the first character of the region, ``end`` marks the last character of the\n  region).\n\n  Both markers must appear exactly once in the file and the resolved end\n  position must not lie before the resolved start position. A precise error\n  is returned otherwise (same error behaviour as the ``change`` tool).\n* Content-hash caching per session: if the client requests the same file\n  again and the file on disk has not changed, an error is returned to avoid\n  redundant transfers.\n* The cache is stored under the ``_read_cache`` key in the session's\n  ``state`` dict as ``{absolute_path: sha256_hex}``.\n\"\"\"\n\nfrom __future__ import annotations\n\nimport hashlib\nfrom pathlib import Path\nfrom typing import Any\n\nfrom ...registry import ToolContext, ToolRegistry, ToolResult\n\n#: Key used inside ``Session.state`` to persist the per-session file cache.\n_CACHE_KEY = \"_read_cache\"\n\n\ndef _sha256(data: bytes) -> str:\n    return hashlib.sha256(data).hexdigest()\n\n\ndef register_read_tool(registry: ToolRegistry) -> None:\n    @registry.tool(\n        \"read\",\n        title=\"Read file\",\n        description=(\n            \"Read the contents of a file and return them as text. \"\n            \"The start of the returned region is given either as a line \"\n            \"number (``min_line``, 1-based inclusive) or as a marker string \"\n            \"(``start``, included in the region) – or omitted to start at the \"\n            \"beginning of the file. The end of the region works analogously \"\n            \"via ``max_line`` / ``end`` (also inclusive), or omitted to read \"\n            \"to the end of the file. ``min_line``/``start`` and \"\n            \"``max_line``/``end`` can be combined arbitrarily. Results are \"\n            \"cached per session by content hash; if the file has not changed \"\n            \"since the last read an error is returned indicating so.\"\n        ),\n        input_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                \"path\": {\n                    \"type\": \"string\",\n                    \"description\": \"Absolute path to the file to read.\",\n                },\n                \"min_line\": {\n                    \"type\": \"integer\",\n                    \"description\": (\n                        \"First line to return (1-based, inclusive). Omit to start from \"\n                        \"the beginning. Mutually exclusive with ``start``.\"\n                    ),\n                    \"minimum\": 1,\n                },\n                \"max_line\": {\n                    \"type\": \"integer\",\n                    \"description\": (\n                        \"Last line to return (1-based, inclusive). Omit to read to the \"\n                        \"end of the file. Mutually exclusive with ``end``.\"\n                    ),\n                    \"minimum\": 1,\n                },\n                \"start\": {\n                    \"type\": \"string\",\n                    \"description\": (\n                        \"Marker string that must occur exactly once in the file; the \"\n                        \"region starts at this marker (inclusive). Mutually exclusive \"\n                        \"with ``min_line``.\"\n                    ),\n                },\n                \"end\": {\n                    \"type\": \"string\",\n                    \"description\": (\n                        \"Marker string that must occur exactly once in the file; the \"\n                        \"region ends at the end of this marker (inclusive). Mutually \"\n                        \"exclusive with ``max_line``.\"\n                    ),\n                },\n            },\n            \"required\": [\"path\"],\n        },\n        output_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                #\"path\": {\"type\": \"string\"},\n                #\"sha256\": {\"type\": \"string\"},\n                #\"total_lines\": {\"type\": \"integer\"},\n                #\"returned_lines\": {\"type\": \"integer\"},\n                \"content\": {\"type\": \"string\"},\n                #\"min_line\": {\"type\": \"integer\"},\n                #\"max_line\": {\"type\": \"integer\"},\n            },\n            \"required\": [\"content\"\n                         #, \"path\", \"sha256\", \"total_lines\", \"returned_lines\"\n                         ],\n        },\n        annotations={\"readOnlyHint\": True, \"openWorldHint\": False},\n    )\n    def read(ctx: ToolContext) -> ToolResult:\n        args: dict[str, Any] = ctx.arguments\n        path_str: str = args[\"path\"]\n        min_line: int | None = args.get(\"min_line\")\n        max_line: int | None = args.get(\"max_line\")\n        start_marker: str | None = args.get(\"start\")\n        end_marker: str | None = args.get(\"end\")\n\n        # --- mutual exclusivity validation ---\n        if min_line is not None and start_marker is not None:\n            return ToolResult(\n                structured_content={\n                    \"error\": \"``min_line`` and ``start`` are mutually exclusive.\"\n                },\n                is_error=True,\n            )\n        if max_line is not None and end_marker is not None:\n            return ToolResult(\n                structured_content={\n                    \"error\": \"``max_line`` and ``end`` are mutually exclusive.\"\n                },\n                is_error=True,\n            )\n\n        path = Path(path_str)\n        if not path.is_absolute():\n            return ToolResult(\n                structured_content={\"error\": f\"Path must be absolute: {path_str}\"},\n                is_error=True,\n            )\n        if not path.exists():\n            return ToolResult(\n                structured_content={\"error\": f\"File not found: {path_str}\"},\n                is_error=True,\n            )\n        if not path.is_file():\n            return ToolResult(\n                structured_content={\"error\": f\"Not a regular file: {path_str}\"},\n                is_error=True,\n            )\n\n        raw_bytes = path.read_bytes()\n        current_hash = _sha256(raw_bytes)\n\n        # --- session cache check ---\n        cache: dict[str, str] = ctx.session.state.setdefault(_CACHE_KEY, {})\n        key = str(path.resolve())\n        if cache.get(key) == current_hash:\n            return ToolResult(\n                structured_content={\n                    \"error\": f\"File has not changed since the last read (sha256={current_hash}): {path_str}\"\n                },\n                is_error=True,\n            )\n        cache[key] = current_hash\n\n        # --- decode ---\n        text = raw_bytes.decode(\"utf-8\", errors=\"replace\")\n        lines = text.splitlines(keepends=True)\n        total_lines = len(lines)\n\n        def line_start_offset(line_num: int) -> int:\n            n = max(0, min(line_num - 1, total_lines))\n            return sum(len(l) for l in lines[:n])\n\n        def line_end_offset(line_num: int) -> int:\n            n = max(0, min(line_num, total_lines))\n            return sum(len(l) for l in lines[:n])\n\n        # --- resolve start boundary ---\n        if start_marker is not None:\n            start_count = text.count(start_marker)\n            if start_count == 0:\n                return ToolResult(\n                    structured_content={\"error\": f\"Start marker not found in file: {start_marker!r}\"},\n                    is_error=True,\n                )\n            if start_count > 1:\n                return ToolResult(\n                    structured_content={\n                        \"error\": (\n                            f\"Start marker is ambiguous – found {start_count} occurrences \"\n                            f\"in file: {start_marker!r}\"\n                        )\n                    },\n                    is_error=True,\n                )\n            region_start = text.index(start_marker)\n        elif min_line is not None:\n            region_start = line_start_offset(min_line)\n        else:\n            region_start = 0\n\n        # --- resolve end boundary ---\n        if end_marker is not None:\n            end_count = text.count(end_marker)\n            if end_count == 0:\n                return ToolResult(\n                    structured_content={\"error\": f\"End marker not found in file: {end_marker!r}\"},\n                    is_error=True,\n                )\n            if end_count > 1:\n                return ToolResult(\n                    structured_content={\n                        \"error\": (\n                            f\"End marker is ambiguous – found {end_count} occurrences \"\n                            f\"in file: {end_marker!r}\"\n                        )\n                    },\n                    is_error=True,\n                )\n            region_end = text.index(end_marker) + len(end_marker)\n        elif max_line is not None:\n            region_end = line_end_offset(max_line)\n        else:\n            region_end = len(text)\n\n        # --- order validation ---\n        if region_end < region_start:\n            return ToolResult(\n                structured_content={\n                    \"error\": (\n                        f\"Resolved end position ({region_end}) must not lie before \"\n                        f\"the resolved start position ({region_start}).\"\n                    )\n                },\n                is_error=True,\n            )\n\n        sliced = text[region_start:region_end]\n\n        structured: dict[str, Any] = {\"content\": sliced}\n        if min_line is not None:\n            structured[\"min_line\"] = min_line\n        if max_line is not None:\n            structured[\"max_line\"] = max_line\n        if start_marker is not None:\n            structured[\"start_line\"] = text.count(\"\\n\", 0, region_start) + 1\n        if end_marker is not None:\n            structured[\"end_line\"] = text.count(\"\\n\", 0, region_end) + 1\n\n        return ToolResult(structured_content=structured)\n"
#:   }
#: }
/allow ba8311bf-c5be-4b58-92bb-dc051a189baf
#: Control Request:
#: {
#:   "id" : "704f3e7b-f88c-4fb2-b972-732894954f3d",
#:   "phase" : "result",
#:   "toolName" : "write",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "result" : "success"
#:     }
#:   }
#: }
/allow 704f3e7b-f88c-4fb2-b972-732894954f3d
#: Control Request:
#: {
#:   "id" : "1d0aeffd-dc2b-4239-abae-a7b50f7dc6c5",
#:   "phase" : "request",
#:   "toolName" : "bash",
#:   "arguments" : {
#:     "cwd" : "/home/user/xyan/xy.ai.workbench/mcpc",
#:     "script" : "python3 -m py_compile src/xy/ai/mcpc/tools/read/__init__.py && echo OK"
#:   }
#: }
/allow 1d0aeffd-dc2b-4239-abae-a7b50f7dc6c5
#: Control Request:
#: {
#:   "id" : "cc788357-4b72-4f85-aa21-06d43b61ab93",
#:   "phase" : "result",
#:   "toolName" : "bash",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "exit_code" : 0,
#:       "stdout" : "OK\n"
#:     }
#:   }
#: }
/allow cc788357-4b72-4f85-aa21-06d43b61ab93
#: Tool: mcp__mcpc__write
#: path: "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py"
#: mode: "replace"
#: content: "\"\"\"Read tool – reads a file and returns its contents.\n\nFeatures\n--------\n* Optional range restriction. The start…
#: ReasoningToken: 1475
#: Tool: mcp__mcpc__bash
#: cwd: "/home/user/xyan/xy.ai.workbench/mcpc"
#: script: "python3 -m py_compile src/xy/ai/mcpc/tools/read/__init__.py && echo OK"
Fertig – kompiliert fehlerfrei.

## Änderungen

Der `mode`-Parameter wurde entfernt; Marker-Ränder verhalten sich jetzt fix wie das `full`-Verhalten des Change-Tools (Marker selbst ist Teil der zurückgegebenen Region).

**Neue Parameter-Logik** (beliebig kombinierbar):
- **Start der Region:** `min_line` **oder** `start` (Marker) **oder** keins von beidem (= Dateianfang)
- **Ende der Region:** `max_line` **oder** `end` (Marker) **oder** keins von beidem (= Dateiende)

Jede Kombination aus Zeilen- und Marker-Grenze ist erlaubt, z. B. `min_line` + `end`-Marker oder `start`-Marker + `max_line`.

**Validierung / Fehlerverhalten** (analog Change-Tool):
- `min_line` + `start` gleichzeitig → Fehler ("mutually exclusive")
- `max_line` + `end` gleichzeitig → Fehler ("mutually exclusive")
- Marker nicht gefunden → Fehler
- Marker mehrfach vorhanden (mehrdeutig) → Fehler mit Trefferanzahl
- Resultierendes Ende liegt vor resultierendem Start → Fehler

**Output:** `min_line`/`max_line` werden gespiegelt, falls als Parameter übergeben; zusätzlich werden `start_line`/`end_line` (1-based) berechnet und zurückgegeben, wenn die jeweilige Grenze über einen Marker aufgelöst wurde.
Komprimiere alle Beschreibungen des Tools und der Parameter. Ziel ist optimale und klare Gestaltung für Agents bei geringen Tokenverbrauch und fehlgeleiteter Salienz.
#: Control Request:
#: {
#:   "id" : "849eeb17-0cea-401b-9a32-5b4c45a2c580",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py",
#:     "max_line" : 100
#:   }
#: }
/allow 849eeb17-0cea-401b-9a32-5b4c45a2c580
#: Control Request:
#: {
#:   "id" : "fb70fe8b-4c46-4cc1-ac6c-9684b767dedb",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "\"\"\"Read tool – reads a file and returns its contents.\n\nFeatures\n--------\n* Optional range restriction. The start of the returned region can be given\n  either as a line number (``min_line``, 1-based inclusive) or as a marker\n  string (``start``) that must occur exactly once in the file; if neither is\n  given, the region starts at the beginning of the file. The end of the\n  returned region works analogously via ``max_line`` / ``end``, defaulting to\n  the end of the file if neither is given. ``min_line``/``start`` and\n  ``max_line``/``end`` can be combined arbitrarily (e.g. a line-based start\n  with a marker-based end).\n\n  Marker-based boundaries always behave like the ``change`` tool's ``full``\n  mode: the marker itself is included in the returned region (``start`` marks\n  the first character of the region, ``end`` marks the last character of the\n  region).\n\n  Both markers must appear exactly once in the file and the resolved end\n  position must not lie before the resolved start position. A precise error\n  is returned otherwise (same error behaviour as the ``change`` tool).\n* Content-hash caching per session: if the client requests the same file\n  again and the file on disk has not changed, an error is returned to avoid\n  redundant transfers.\n* The cache is stored under the ``_read_cache`` key in the session's\n  ``state`` dict as ``{absolute_path: sha256_hex}``.\n\"\"\"\n\nfrom __future__ import annotations\n\nimport hashlib\nfrom pathlib import Path\nfrom typing import Any\n\nfrom ...registry import ToolContext, ToolRegistry, ToolResult\n\n#: Key used inside ``Session.state`` to persist the per-session file cache.\n_CACHE_KEY = \"_read_cache\"\n\n\ndef _sha256(data: bytes) -> str:\n    return hashlib.sha256(data).hexdigest()\n\n\ndef register_read_tool(registry: ToolRegistry) -> None:\n    @registry.tool(\n        \"read\",\n        title=\"Read file\",\n        description=(\n            \"Read the contents of a file and return them as text. \"\n            \"The start of the returned region is given either as a line \"\n            \"number (``min_line``, 1-based inclusive) or as a marker string \"\n            \"(``start``, included in the region) – or omitted to start at the \"\n            \"beginning of the file. The end of the region works analogously \"\n            \"via ``max_line`` / ``end`` (also inclusive), or omitted to read \"\n            \"to the end of the file. ``min_line``/``start`` and \"\n            \"``max_line``/``end`` can be combined arbitrarily. Results are \"\n            \"cached per session by content hash; if the file has not changed \"\n            \"since the last read an error is returned indicating so.\"\n        ),\n        input_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                \"path\": {\n                    \"type\": \"string\",\n                    \"description\": \"Absolute path to the file to read.\",\n                },\n                \"min_line\": {\n                    \"type\": \"integer\",\n                    \"description\": (\n                        \"First line to return (1-based, inclusive). Omit to start from \"\n                        \"the beginning. Mutually exclusive with ``start``.\"\n                    ),\n                    \"minimum\": 1,\n                },\n                \"max_line\": {\n                    \"type\": \"integer\",\n                    \"description\": (\n                        \"Last line to return (1-based, inclusive). Omit to read to the \"\n                        \"end of the file. Mutually exclusive with ``end``.\"\n                    ),\n                    \"minimum\": 1,\n                },\n                \"start\": {\n                    \"type\": \"string\",\n                    \"description\": (\n                        \"Marker string that must occur exactly once in the file; the \"\n                        \"region starts at this marker (inclusive). Mutually exclusive \"\n                        \"with ``min_line``.\"\n                    ),\n                },\n                \"end\": {\n                    \"type\": \"string\",\n                    \"description\": (\n                        \"Marker string that must occur exactly once in the file; the \"\n                        \"region ends at the end of this marker (inclusive). Mutually \"\n                        \"exclusive with ``max_line``.\"\n                    ),\n                },\n            },\n",
#:       "max_line" : 100
#:     }
#:   }
#: }
/allow fb70fe8b-4c46-4cc1-ac6c-9684b767dedb
#: Control Request:
#: {
#:   "id" : "85f2cb01-1d79-4b66-b43e-2af0d63930d6",
#:   "phase" : "request",
#:   "toolName" : "change",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py",
#:     "mode" : "full",
#:     "start" : "\"\"\"Read tool – reads a file and returns its contents.",
#:     "end" : "``{absolute_path: sha256_hex}``.\n\"\"\"",
#:     "content" : "\"\"\"Read tool – returns file contents, optionally sliced by line or unique marker.\n\nRange: start = min_line | start-marker | file start; end = max_line | end-marker\n| file end (all inclusive). Markers must be unique substrings. Per-session\nsha256 cache rejects unchanged re-reads (key ``_read_cache`` in session state).\n\"\"\""
#:   }
#: }
/allow 85f2cb01-1d79-4b66-b43e-2af0d63930d6
#: Control Request:
#: {
#:   "id" : "b5d79bd5-552a-4aed-afd9-219d9c7388db",
#:   "phase" : "result",
#:   "toolName" : "change",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "result" : "success"
#:     }
#:   }
#: }
/allow b5d79bd5-552a-4aed-afd9-219d9c7388db
#: Control Request:
#: {
#:   "id" : "d1484cb5-0722-47cb-8b1b-9ccf229a3b55",
#:   "phase" : "request",
#:   "toolName" : "change",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py",
#:     "mode" : "full",
#:     "start" : "        description=(\n            \"Read the contents of a file and return them as text. \"\n            \"The start of the returned region is given either as a line \"\n            \"number (``min_line``, 1-based inclusive) or as a marker string \"\n            \"(``start``, included in the region) – or omitted to start at the \"\n            \"beginning of the file. The end of the region works analogously \"\n            \"via ``max_line`` / ``end`` (also inclusive), or omitted to read \"\n            \"to the end of the file. ``min_line``/``start`` and \"\n            \"``max_line``/``end`` can be combined arbitrarily. Results are \"\n            \"cached per session by content hash; if the file has not changed \"\n            \"since the last read an error is returned indicating so.\"\n        ),",
#:     "end" : "        input_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                \"path\": {\n                    \"type\": \"string\",\n                    \"description\": \"Absolute path to the file to read.\",\n                },\n                \"min_line\": {\n                    \"type\": \"integer\",\n                    \"description\": (\n                        \"First line to return (1-based, inclusive). Omit to start from \"\n                        \"the beginning. Mutually exclusive with ``start``.\"\n                    ),\n                    \"minimum\": 1,\n                },\n                \"max_line\": {\n                    \"type\": \"integer\",\n                    \"description\": (\n                        \"Last line to return (1-based, inclusive). Omit to read to the \"\n                        \"end of the file. Mutually exclusive with ``end``.\"\n                    ),\n                    \"minimum\": 1,\n                },\n                \"start\": {\n                    \"type\": \"string\",\n                    \"description\": (\n                        \"Marker string that must occur exactly once in the file; the \"\n                        \"region starts at this marker (inclusive). Mutually exclusive \"\n                        \"with ``min_line``.\"\n                    ),\n                },\n                \"end\": {\n                    \"type\": \"string\",\n                    \"description\": (\n                        \"Marker string that must occur exactly once in the file; the \"\n                        \"region ends at the end of this marker (inclusive). Mutually \"\n                        \"exclusive with ``max_line``.\"\n                    ),\n                },\n            },",
#:     "content" : "        description=(\n            \"Read a file as text, optionally sliced to a range. Repeated \"\n            \"unchanged reads return an error.\"\n        ),\n        input_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                \"path\": {\n                    \"type\": \"string\",\n                    \"description\": \"Absolute file path.\",\n                },\n                \"min_line\": {\n                    \"type\": \"integer\",\n                    \"description\": \"Range start: line number, inclusive, 1-based. Excludes start.\",\n                    \"minimum\": 1,\n                },\n                \"max_line\": {\n                    \"type\": \"integer\",\n                    \"description\": \"Range end: line number, inclusive, 1-based. Excludes end.\",\n                    \"minimum\": 1,\n                },\n                \"start\": {\n                    \"type\": \"string\",\n                    \"description\": \"Range start: unique marker substring, inclusive. Excludes min_line.\",\n                },\n                \"end\": {\n                    \"type\": \"string\",\n                    \"description\": \"Range end: unique marker substring, inclusive. Excludes max_line.\",\n                },\n            },"
#:   }
#: }
/allow d1484cb5-0722-47cb-8b1b-9ccf229a3b55
#: Control Request:
#: {
#:   "id" : "a76851b5-4be0-4029-b733-1422b88a0ed7",
#:   "phase" : "result",
#:   "toolName" : "change",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "result" : "success"
#:     }
#:   }
#: }
/allow a76851b5-4be0-4029-b733-1422b88a0ed7
#: Control Request:
#: {
#:   "id" : "70b4d491-eb96-44dd-8a50-6361072a314e",
#:   "phase" : "request",
#:   "toolName" : "bash",
#:   "arguments" : {
#:     "cwd" : "/home/user/xyan/xy.ai.workbench/mcpc",
#:     "script" : "python3 -m py_compile src/xy/ai/mcpc/tools/read/__init__.py && echo OK"
#:   }
#: }
/allow 70b4d491-eb96-44dd-8a50-6361072a314e
#: Control Request:
#: {
#:   "id" : "8a745483-72c7-436d-89cb-fdedc97d2318",
#:   "phase" : "result",
#:   "toolName" : "bash",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "exit_code" : 0,
#:       "stdout" : "OK\n"
#:     }
#:   }
#: }
/allow 8a745483-72c7-436d-89cb-fdedc97d2318
#: Tool: mcp__mcpc__change
#: path: "/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py"
#: mode: "full"
#: start: "        description=(\n            \"Read the contents of a file and return them as text. \"\n            \"The start o…
#: end: "        input_schema={\n            \"type\": \"object\",\n            \"properties\": {\n                \"path\": {\n…
#: content: "        description=(\n            \"Read a file as text, optionally sliced to a range. Repeated \"\n            \"unch…
#: Tool: mcp__mcpc__bash
#: cwd: "/home/user/xyan/xy.ai.workbench/mcpc"
#: script: "python3 -m py_compile src/xy/ai/mcpc/tools/read/__init__.py && echo OK"
Kompiliert fehlerfrei. Alle Beschreibungen wurden deutlich komprimiert:

**Modul-Docstring:** von ~25 Zeilen auf 4 kompakte Zeilen reduziert.

**Tool-Description:** von langem Fließtext auf einen Satz: *"Read a file as text, optionally sliced to a range. Repeated unchanged reads return an error."*

**Parameter-Descriptions:** einheitliches, knappes Muster ohne Redundanz zwischen den Feldern:
- `path`: "Absolute file path."
- `min_line`: "Range start: line number, inclusive, 1-based. Excludes start."
- `max_line`: "Range end: line number, inclusive, 1-based. Excludes end."
- `start`: "Range start: unique marker substring, inclusive. Excludes min_line."
- `end`: "Range end: unique marker substring, inclusive. Excludes max_line."

Die Ausschluss-Beziehung wird pro Parameter mit dem jeweils anderen Namen ("Excludes min_line"/"Excludes max_line") statt mit "mutually exclusive with X" formuliert – kürzer und ebenso eindeutig. Die Begriffe "start"/"end" (Range-Rolle) sind konsequent von den Parameternamen `start`/`end` (Marker) getrennt gehalten, um Fehlinterpretationen durch das Modell zu vermeiden. Funktionales Verhalten und Fehlerbehandlung im Code sind unverändert.