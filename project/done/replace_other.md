Ändere die Funktionsweise von `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace/__init__.py`. Start und Endmarker sind exklusiv. Also es wird der Bereich nach start und vor end ersetzt. Passe docstrings und MCP für Agenten entsprechend an.
SystemInit:  id=adaafba0-e8be-44e8-b473-da538921d9d5, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: a9d4-1
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace/__init__.py
  reason: "Datei lesen, um die Marker-Logik für start/end zu verstehen und anzupassen"
```
/answer a9d4-1 allow
Control Request:
```yaml
id: a9d4-2
toolName: replace_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace/__init__.py
  old_text: "\"\"\"Replace tool – replaces the text matched by 'start', optionally extended through 'end' (both inclusive), with given content.\"\"\""
  new_text: "\"\"\"Replace tool – replaces the text matched by 'start', optionally extended up to 'end' (both exclusive), with given content.\"\"\""
  reason: Modul-Docstring an neue exklusive Start/End-Semantik anpassen
```
/answer a9d4-2 allow
Control Request:
```yaml
id: a9d4-3
toolName: replace_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace/__init__.py
  old_text: |2-
        """Replace the text matched by 'start', optionally extended through 'end', with content.

        Args:
            path: Absolute path to target file (must be a regular file).
            start: Unique substring marking the block's start (must occur exactly once).
                   If 'end' is omitted, this substring alone is what gets replaced.
            content: Replacement text. Repeat a marker inside content to keep it.
            end: Unique substring marking the block's end (must occur exactly once, after start).
                 If omitted, only the 'start' match itself is replaced.
            exact: If False (default), whitespace in start/end is matched tolerantly
                   (any whitespace run matches any other). If True, whitespace must match exactly.

        Returns:
            ReplaceResult with success status.

        Raises:
            ReplaceError: If path is not absolute, not found, or not a regular file.
            ReplaceError: If start or end markers are not found or appear more than once.
            ReplaceError: If end marker does not appear after start marker.
        """
  new_text: |2-
        """Replace the text between 'start' and 'end', with content. Both markers are excluded from the replacement and kept in the file.

        Args:
            path: Absolute path to target file (must be a regular file).
            start: Unique substring marking the block's start (must occur exactly once).
                   If 'end' is omitted, this substring alone is what gets replaced.
            content: Replacement text. Repeat a marker inside content to keep it.
            end: Unique substring marking the block's end (must occur exactly once, after start).
                 If omitted, only the 'start' match itself is replaced.
            exact: If False (default), whitespace in start/end is matched tolerantly
                   (any whitespace run matches any other). If True, whitespace must match exactly.

        Returns:
            ReplaceResult with success status.

        Raises:
            ReplaceError: If path is not absolute, not found, or not a regular file.
            ReplaceError: If start or end markers are not found or appear more than once.
            ReplaceError: If end marker does not start after start marker ends.
        """
  reason: Docstring der Funktion 'replace' an exklusive Start/End-Semantik anpassen
```
/answer a9d4-3 allow
Control Request:
```yaml
id: a9d4-4
toolName: replace_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace/__init__.py
  old_text: |2-
            if end_match.start < start_match.start:
                raise ReplaceError('End marker must appear after start marker.')
        result_text = text[:start_match.start] + content + text[end_match.end:]
  new_text: |2-
            if end_match.start < start_match.end:
                raise ReplaceError('End marker must start after start marker ends.')
        result_text = text[:start_match.end] + content + text[end_match.start:]
  reason: Ersetzungsbereich auf den exklusiven Bereich zwischen start.end und end.start umstellen
```
/answer a9d4-4 allow
Control Request:
```yaml
id: a9d4-5
toolName: replace_block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/replace/__init__.py
  old_text: |2-
        description = "Replace the text matched by 'start' with 'content'. If 'end' is also given, everything from 'start' through 'end' (both included) is replaced instead. Each marker must occur exactly once in the file; if given, 'end' must come after 'start'. Repeat a marker inside 'content' to keep it. By default whitespace in 'start'/'end' is matched tolerantly; set 'exact' to require exact whitespace matching."
        input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the target file.'}, 'start': {'type': 'string', 'description': "Unique substring marking the block's start (must occur exactly once). If 'end' is omitted, this substring alone is what gets replaced."}, 'end': {'type': 'string', 'description': "Optional unique substring marking the block's end (must occur exactly once, after 'start'). If omitted, only the 'start' match is replaced."}, 'content': {'type': 'string', 'description': "Text that replaces the matched block, including where 'start'/'end' were."}, 'exact': {'type': 'boolean', 'description': "If true, 'start'/'end' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.", 'default': False}}, 'required': ['path', 'start', 'content']}
  new_text: |2-
        description = "Replace the text matched by 'start' with 'content'. If 'end' is also given, everything strictly between 'start' and 'end' (both excluded and kept in the file) is replaced instead. Each marker must occur exactly once in the file; if given, 'end' must start after 'start' ends. Repeat a marker inside 'content' to keep it. By default whitespace in 'start'/'end' is matched tolerantly; set 'exact' to require exact whitespace matching."
        input_schema = {'type': 'object', 'properties': {'path': {'type': 'string', 'description': 'Absolute path to the target file.'}, 'start': {'type': 'string', 'description': "Unique substring marking the block's start (must occur exactly once). If 'end' is omitted, this substring alone is what gets replaced."}, 'end': {'type': 'string', 'description': "Optional unique substring marking the block's end (must occur exactly once, after 'start' ends). If given, only the text strictly between 'start' and 'end' is replaced; both markers themselves are kept. If omitted, only the 'start' match is replaced."}, 'content': {'type': 'string', 'description': "Text that replaces the matched block (the block between 'start' and 'end' when 'end' is given, otherwise the 'start' match itself)."}, 'exact': {'type': 'boolean', 'description': "If true, 'start'/'end' must match whitespace exactly. If false (default), whitespace runs match any amount/kind of whitespace.", 'default': False}}, 'required': ['path', 'start', 'content']}
  reason: MCP-Tool-Beschreibung und Input-Schema an exklusive Start/End-Semantik anpassen
```
/answer a9d4-5 allow
ReasoningToken: 387
Text:
 Jetzt die eigentliche Logik, Fehlermeldung und Tool-Beschreibungen anpassen:
Erledigt. Die Funktion `replace` sowie das MCP-Tool `ReplaceTool` behandeln `start` und `end` jetzt exklusiv: Bei Angabe von `end` wird nur der Text zwischen dem Ende von `start` und dem Anfang von `end` ersetzt, beide Marker selbst bleiben unverändert im Text erhalten. Ohne `end` bleibt das Verhalten unverändert (der `start`-Treffer selbst wird ersetzt). Docstrings, Fehlermeldung und MCP-Beschreibung/Input-Schema wurden entsprechend angepasst.
