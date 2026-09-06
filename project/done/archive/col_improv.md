Verbessere das Result in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/colgrep/__init__.py`. Leere Ergebniskomponenten sollen nicht zurückgegeben werden (false,Leerstring, null). Ebenso braucht "score" nicht verarbeitet werden und kann gefiltert werden.


```yaml
    - unit:
        name: raw_code_234
        qualified_name: 02_pathogenese_modell/data/hrsignatures/night_characterization.py::raw_code_234
        file: /home/user/xyan/XY.Archiv/02_pathogenese_modell/data/hrsignatures/night_characterization.py
        line: 234
        end_line: 284
        language: python
        unit_type: rawcode
        signature: "# ---------------------------------------------------------------------------"
        docstring: null
        parameters: []
        return_type: null
        extends: null
        parent_class: null
        calls: []
        called_by: []
        complexity: 1
        has_loops: false
        has_branches: false
        has_error_handling: false
        variables: []
        imports:
        - datetime
        - numpy
        - pandas
        - pathlib
        - scipy
        - sys
        code: |-
          # ---------------------------------------------------------------------------
          # Main
          # ---------------------------------------------------------------------------
      score: 0.43414623
    count: 10
```
SystemInit:  id=e7c9c42a-4846-47b1-9823-cf26db172347, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 9638-5
toolName: python-ast-outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/colgrep/__init__.py
  reason: "Struktur der Datei verstehen, um die Result-Formatierung zu finden"
```
/answer 9638-5 allow
ReasoningToken: 29
Control Request:
```yaml
id: 9638-6
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/colgrep/__init__.py
  reason: Vollständigen Code lesen um Result-Verarbeitung zu finden und anzupassen
```
/answer 9638-6 allow
Control Request:
```yaml
id: 9638-7
toolName: python-ast-insert
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/colgrep/__init__.py
  qualified_name: _find_index_root
  position: after
  code: |
    def _clean_result(value: Any) -> Any:
        """Recursively drop empty components (``False``, ``""``, ``None``) and the
        ``score`` key from colgrep JSON output.
        """
        if isinstance(value, dict):
            cleaned = {}
            for key, item in value.items():
                if key == 'score':
                    continue
                cleaned_item = _clean_result(item)
                if cleaned_item is False or cleaned_item == '' or cleaned_item is None:
                    continue
                cleaned[key] = cleaned_item
            return cleaned
        if isinstance(value, list):
            return [_clean_result(item) for item in value]
        return value
  reason: Hilfsfunktion zum Bereinigen leerer Ergebniskomponenten und Filtern von 'score' hinzufügen
```
/answer 9638-7 allow
Control Request:
```yaml
id: 9638-8
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/colgrep/__init__.py
  old_text: |2-
            payload = {'results': parsed, 'count': len(parsed)} if isinstance(parsed, list) else {'results': [parsed], 'count': 1}
            return ToolResult(structured_content=payload)
  new_text: |2-
            parsed = _clean_result(parsed)
            payload = {'results': parsed, 'count': len(parsed)} if isinstance(parsed, list) else {'results': [parsed], 'count': 1}
            return ToolResult(structured_content=payload)
  reason: Bereinigung der geparsten colgrep-Ergebnisse vor der Payload-Erstellung anwenden
```
/answer 9638-8 allow
ReasoningToken: 273
Control Request:
```yaml
id: 9638-9
toolName: python-ast-validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/colgrep/__init__.py
  reason: Syntaxprüfung nach den Änderungen
```
/answer 9638-9 allow
Erledigt. Ich habe eine rekursive Hilfsfunktion `_clean_result` hinzugefügt, die auf das geparste JSON von `colgrep` angewendet wird, bevor die Payload gebaut wird:

- Entfernt in jedem Dict den Key `score` unabhängig vom Wert.
- Entfernt in jedem Dict Keys, deren (bereits rekursiv bereinigter) Wert `False`, `""` oder `None` ist.
- Steigt dabei rekursiv in verschachtelte Dicts/Listen ab (z.B. das `unit`-Objekt innerhalb eines Ergebnisses).

Damit fällt z.B. `docstring: null`, `extends: null`, `parent_class: null`, `has_loops: false` etc. weg, und `score` wird generell nicht mehr zurückgegeben — ohne dass die restliche Struktur (Listen, verschachtelte Objekte) angetastet wird.

Verbessere das Result in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/colgrep/__init__.py`. Auch leere Arrays sollen im Result gefiltert werden. Zudem soll das Feld "language", "signature", "qualified_name", "unit_type", "complexity", "has_loops", "has_branches", "variables" entfernt werden. Das Feld "code" muss in der Länge (100) beschränkt werden.

```yaml
id: 27e1-4
result:
  structuredContent:
    results:
    - unit:
        name: raw_code_1
        qualified_name: hrviewer/src/xy/hrv/viewer/__main__.py::raw_code_1
        file: /home/user/xyan/XY.Archiv/hrviewer/src/xy/hrv/viewer/__main__.py
        line: 1
        end_line: 47
        language: python
        unit_type: rawcode
        signature: "\"\"\"CLI entry point: ``python -m xy.hrv.viewer`` / console script ``hrv``.\"\"\""
        parameters: []
        calls: []
        called_by: []
        complexity: 1
        variables: []
        imports:
        - argparse
        code: |-
          """CLI entry point: ``python -m xy.hrv.viewer`` / console script ``hrv``."""
          from __future__ import annotations

          import argparse

          from .app import HrViewer
          from .config import Config
```
SystemInit:  id=8787af86-5ac8-41de-b196-a1192a540dbe, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 27e1-5
toolName: python-ast-outline
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/colgrep/__init__.py
  reason: "Struktur der Datei verstehen, um die Result-Erstellung zu finden"
```
/answer 27e1-5 allow
ReasoningToken: 27
Control Request:
```yaml
id: 27e1-6
toolName: read-file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/colgrep/__init__.py
  reason: Details der _clean_result und Umgebung lesen
```
/answer 27e1-6 allow
Control Request:
```yaml
id: 27e1-7
toolName: python-ast-replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/colgrep/__init__.py
  qualified_name: _clean_result
  reason: "Result-Filterung erweitern: leere Arrays entfernen, unerwuenschte Felder droppen, code kürzen"
  old_text: |2-
        """Recursively drop empty components (``False``, ``""``, ``None``) and the
        ``score`` key from colgrep JSON output.
        """
        if isinstance(value, dict):
            cleaned = {}
            for key, item in value.items():
                if key == 'score':
                    continue
                cleaned_item = _clean_result(item)
                if cleaned_item is False or cleaned_item == '' or cleaned_item is None:
                    continue
                cleaned[key] = cleaned_item
            return cleaned
        if isinstance(value, list):
            return [_clean_result(item) for item in value]
        return value
  new_text: |2-
        """Recursively drop empty components (``False``, ``""``, ``None``, ``[]``) and
        unwanted keys (``score`` plus the fields listed in ``_DROPPED_KEYS``) from
        colgrep JSON output. The ``code`` field is truncated to ``_MAX_CODE_LEN``
        characters.
        """
        if isinstance(value, dict):
            cleaned = {}
            for key, item in value.items():
                if key == 'score' or key in _DROPPED_KEYS:
                    continue
                if key == 'code' and isinstance(item, str) and len(item) > _MAX_CODE_LEN:
                    item = item[:_MAX_CODE_LEN]
                cleaned_item = _clean_result(item)
                if cleaned_item is False or cleaned_item == '' or cleaned_item is None or cleaned_item == []:
                    continue
                cleaned[key] = cleaned_item
            return cleaned
        if isinstance(value, list):
            return [_clean_result(item) for item in value]
        return value
```
/answer 27e1-7 allow
ReasoningToken: 233
Control Request:
```yaml
id: 27e1-8
toolName: replace-block
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/colgrep/__init__.py
  old_text: _MAX_RESULTS = 50
  new_text: |-
    _MAX_RESULTS = 50
    _MAX_CODE_LEN = 100
    _DROPPED_KEYS = frozenset({'language', 'signature', 'qualified_name', 'unit_type', 'complexity', 'has_loops', 'has_branches', 'variables'})
  reason: Konstanten für gedroppte Felder und maximale code-Länge einführen
```
/answer 27e1-8 allow
Control Request:
```yaml
id: 27e1-9
toolName: python-ast-validate
arguments:
  paths:
  - /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/colgrep/__init__.py
  reason: Syntaxprüfung nach den Änderungen
```
/answer 27e1-9 allow
Text:
 Jetzt die Konstanten `_DROPPED_KEYS` und `_MAX_CODE_LEN` hinzufügen.
Änderungen sind erfolgreich und syntaktisch valide. Zusammenfassung:

- `_MAX_CODE_LEN = 100` und `_DROPPED_KEYS` (language, signature, qualified_name, unit_type, complexity, has_loops, has_branches, variables) hinzugefügt.
- `_clean_result` entfernt jetzt zusätzlich leere Arrays (`[]`) sowie die genannten Felder und kürzt `code` auf 100 Zeichen.
