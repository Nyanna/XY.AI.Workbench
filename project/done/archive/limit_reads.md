Implementiere in `/home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py` eine Kontrolle. Wenn Bereichsparameter bereits mehr als 70% der Datei selektieren, soll die Anfrage abgelehnt werden mit dem Hinweis die ganze Datei konditional einzulesen. 
SystemInit:  id=19a38aff-016f-421f-a6fb-ec204e1e7733, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5
Control Request:
```yaml
id: 24f3-1
toolName: read_file
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
  reason: Datei lesen um Bereichsparameter-Logik zu finden und Kontrolle zu implementieren
```
/answer 24f3-1 allow
ReasoningToken: 9
ReasoningToken: 718
Control Request:
```yaml
id: 24f3-2
toolName: edit_marks
arguments:
  path: /home/user/xyan/xy.ai.workbench/mcpc/src/xy/ai/mcpc/tools/read/__init__.py
  start: "    sliced = text[region_start:region_end]"
  end: '    is_full_file = min_line is None and max_line is None and (min_char is None) and (max_char is None) and (start is None) and (end is None)'
  content: |2-
        is_full_file = min_line is None and max_line is None and (min_char is None) and (max_char is None) and (start is None) and (end is None)
        if not is_full_file and len(text) and (region_end - region_start) > 0.7 * len(text):
            raise ReadError('The requested range selects more than 70% of the file. Read the whole file instead (omit the range parameters) and rely on the checksum-based conditional read to detect unchanged content.')
        sliced = text[region_start:region_end]
  reason: "Kontrolle einfügen, die Anfragen ablehnt, wenn der Bereich mehr als 70% der Datei selektiert"
```
/answer 24f3-2 allow
Die Kontrolle ist implementiert: Wenn ein Bereichsparameter (Zeilen, Zeichen-Offsets oder Marker) mehr als 70% der Dateigröße selektiert und es sich nicht um einen vollständigen Datei-Read handelt, wirft `read_file` einen `ReadError` mit dem Hinweis, stattdessen die ganze Datei ohne Bereichsparameter zu lesen und die checksum-basierte konditionale Erkennung unveränderten Inhalts zu nutzen.
