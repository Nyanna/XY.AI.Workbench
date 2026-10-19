Ich benötige ein Script:

- Für jede Datei in `/home/user/xyan/xy.ai.workbench/project/resident/plan` soll eine gleichnamige Datei in `/home/user/xyan/xy.ai.workbench/project/resident/plan/runs` angelegt werden.
- Für jede Datei die eine "SystemInit:  id=..." Zeile enthält.
- Aus der Ursprungsdatei soll alles ab dieser Zeile (inkl.) bis zum Dateiende in die Schwesterdatei verschoben werden. 
SystemInit:  id=052e9e38-3947-401d-98f7-36671244c450, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
ReasoningToken: 220
Control Request:
```yaml
id: 2a21-6b
toolName: ast_create
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/split_runs.py
    source: |
      #!/usr/bin/env python3
      import re
      import sys
      from pathlib import Path

      PLAN_DIR = Path("/home/user/xyan/xy.ai.workbench/project/resident/plan")
      RUNS_DIR = PLAN_DIR / "runs"
      MARKER = re.compile(r"SystemInit:\s+id=")


      def main() -> int:
          RUNS_DIR.mkdir(exist_ok=True)
          for src in sorted(p for p in PLAN_DIR.iterdir() if p.is_file()):
              lines = src.read_text(encoding="utf-8").splitlines(keepends=True)
              idx = next((i for i, l in enumerate(lines) if MARKER.search(l)), None)
              if idx is None:
                  continue
              dst = RUNS_DIR / src.name
              if dst.exists():
                  print(f"skip (exists): {dst}", file=sys.stderr)
                  continue
              dst.write_text("".join(lines[idx:]), encoding="utf-8")
              src.write_text("".join(lines[:idx]), encoding="utf-8")
              print(f"moved {len(lines) - idx} lines: {src.name}")
          return 0


      if __name__ == "__main__":
          sys.exit(main())
  reason: Create the requested split script
```
/answer 2a21-6b allow
Result Stats: id=052e9e38-3947-401d-98f7-36671244c450, total: 8678, in: 4, out: 1308, reason: 0, read: 6449, write: 7366, start: 1792439543722, end: 1792439547743, time: 3973, total: 10320, io: 0, control: 48, sent: 0, recv: 93
Ich habe das Script unter `/home/user/xyan/xy.ai.workbench/project/split_runs.py` angelegt. Ausgeführt oder getestet habe ich es noch nicht.

Start: `python3 /home/user/xyan/xy.ai.workbench/project/split_runs.py`

Das Script geht so vor:
- Es prüft jede Datei direkt in `plan/`. Unterordner wie `runs` werden nicht betrachtet.
- Die Zeile wird mit dem Regex `SystemInit:\s+id=` gesucht. Das passt auch bei einem oder mehreren Leerzeichen nach dem Doppelpunkt.
- Bei mehreren Treffern gilt die erste Zeile. Sie und alles bis zum Dateiende wandern in die gleichnamige Datei in `runs/`, und die Ursprungsdatei wird auf den Teil davor gekürzt.
- Dateien ohne diese Zeile bleiben unverändert.
- Existiert die Zieldatei in `runs/` schon, wird die Datei übersprungen und eine Warnung ausgegeben. So wird nichts überschrieben.

Weil die Ursprungsdateien verändert werden, solltest du vorher kurz `git status` prüfen, damit du die Änderungen bei Bedarf zurücksetzen kannst.
