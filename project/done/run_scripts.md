Ich benötige nachfolgend an die `batch_specs` ein weiteres Python Script das für jede Spec Datei in `/home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/specs` eine gleichlautende Prompt Markdown Datei in `/home/user/xyan/xy.ai.workbench/project/resident_02/25_implement/runs` erzeugt.
Die Prompt Datei soll analog auf Basis eines Templates erzeugt werden und einen Platzhalter für einen absoluten Verweis auf die Spec Datei zu haben. Ich erweitere das Template dann selbst und ich werde es dann selbst einem Agenten manuell zur Ausführung übergeben.

`/home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py`
`/home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/prompt.md`

Script erzeugen in `/home/user/xyan/xy.ai.workbench/project/resident_02/25_implement`
SystemInit:  id=f7d90c5a-5925-4d62-942f-48645f772561, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich lese zuerst die beiden Referenzdateien.
Control Request:
```yaml
id: 7673-15
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/prompt.md
  reason: "Referenz-Script und Template lesen, um den Stil analog zu übernehmen"
```
/answer 7673-15 allow
Control Request:
```yaml
id: 7673-16
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/specs
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/25_implement
    recursive: false
  reason: Vorhandene Spec-Dateien und Zielverzeichnis prüfen
```
/answer 7673-16 allow
Control Request:
```yaml
id: 7673-17
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/specs
      entries:
      - ".: (140 files)"
      - 001_erweiterbare_zusatzparameter.md
      warning: Result reduced because the total number of entries (140) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
    - path: /home/user/xyan/xy.ai.workbench/project/resident_02/25_implement
      entries: []
```
/answer 7673-17 allow
ReasoningToken: 93
Control Request:
```yaml
id: 7673-18
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/25_implement/generate_prompts.py
    mode: replace
    content: |
      #!/usr/bin/env python3
      """Create one implementation prompt per specification file from a template."""
      import argparse
      import sys
      from pathlib import Path

      from jinja2 import Environment, FileSystemLoader, StrictUndefined

      BASE_DIR = Path('/home/user/xyan/xy.ai.workbench/project/resident_02')
      IMPL_DIR = BASE_DIR / '25_implement'


      def main() -> int:
          parser = argparse.ArgumentParser(description=__doc__)
          parser.add_argument('--spec-dir', type=Path, default=BASE_DIR / '23_batch' / 'specs')
          parser.add_argument('--template', type=Path, default=IMPL_DIR / 'prompt.md')
          parser.add_argument('--out-dir', type=Path, default=IMPL_DIR / 'runs')
          parser.add_argument('--force', action='store_true', help='overwrite existing prompt files')
          args = parser.parse_args()

          specs = sorted(args.spec_dir.glob('*.md'))
          if not specs:
              print(f'No specification files in {args.spec_dir}', file=sys.stderr)
              return 1

          env = Environment(loader=FileSystemLoader(str(args.template.parent)),
                            undefined=StrictUndefined, keep_trailing_newline=True)
          template = env.get_template(args.template.name)
          args.out_dir.mkdir(parents=True, exist_ok=True)

          for spec in specs:
              target = args.out_dir / spec.name
              if target.exists() and not args.force:
                  print(f'[skip] {target.name}')
                  continue
              target.write_text(
                  template.render(spec_path=str(spec.resolve()), name=spec.stem),
                  encoding='utf-8')
              print(f'[ok  ] {target}')
          return 0


      if __name__ == '__main__':
          sys.exit(main())
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/25_implement/prompt.md
    mode: replace
    content: |
      Implement the module "{{ name }}".

      Specification: {{ spec_path }}
  reason: Generator-Script und Start-Template im Verzeichnis 25_implement anlegen
```
/answer 7673-18 allow
Result Stats: id=f7d90c5a-5925-4d62-942f-48645f772561, total: 10349, in: 6, out: 1963, reason: 0, read: 10076, write: 8380, start: 1793659820809, end: 1793659827558, time: 6684, total: 23905, io: 0, control: 65, sent: 0, recv: 91
Ich habe das Script und ein minimales Template angelegt. Ausgeführt oder getestet habe ich nichts.

- **Script:** `/home/user/xyan/xy.ai.workbench/project/resident_02/25_implement/generate_prompts.py`
- **Template:** `/home/user/xyan/xy.ai.workbench/project/resident_02/25_implement/prompt.md`

**Funktionsweise:**
- Das Script nimmt jede `*.md` aus `23_batch/specs` und rendert das Template mit Jinja2, wie in `batch_specs.py`. Das Ergebnis liegt unter dem gleichen Dateinamen in `25_implement/runs`.
- Das Template kennt zwei Platzhalter. `{{ spec_path }}` ist der absolute Pfad zur Spec-Datei, `{{ name }}` der Dateiname ohne Endung.
- Das Template ist bewusst nur ein Gerüst aus zwei Zeilen, das du selbst erweiterst.
- Bereits vorhandene Prompt-Dateien werden übersprungen. Mit `--force` werden sie überschrieben.
- Mit `--spec-dir`, `--template` und `--out-dir` kannst du die Pfade überschreiben.

Ausführen:
```
python3 /home/user/xyan/xy.ai.workbench/project/resident_02/25_implement/generate_prompts.py
```
