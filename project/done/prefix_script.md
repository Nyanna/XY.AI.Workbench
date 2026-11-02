Ich benötige ein weiteres Python Script das nachfolgend auf `/home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py` die Fertigen Specs, dem Baum nach mit einer Prefixnummer umbenennt ("003_" nach Ordnungsnummer, padding mit führenden Nullen).
SystemInit:  id=88207fa2-3048-4561-95a3-68a0d9dc4f19, cwd=/home/user/xyan/xy.ai.workbench/project, model=claude-sonnet-5-5
Text:
 Ich lese zuerst das bestehende Script, um Struktur und Konventionen zu übernehmen.
ReasoningToken: 23
Control Request:
```yaml
id: 7673-14
toolName: read_file
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/project/resident_02/23_batch/batch_specs.py
  reason: "Bestehendes batch_specs.py lesen, um Ausgabestruktur (Spec-Dateien, Baum, Namenskonventionen) für das Umbenennungs-Script zu verstehen."
```
/answer 7673-14 allow
Result Stats: id=88207fa2-3048-4561-95a3-68a0d9dc4f19, total: 7158, in: 4, out: 2324, reason: 410, read: 1938, write: 4830, start: 1793654599654, end: 1793654615955, time: 16203, total: 19835, io: 0, control: 98, sent: 0, recv: 299
ReasoningToken: 410
Ich habe kein Schreib-Tool, deshalb steht das Script hier zum Speichern, z. B. als `resident_02/23_batch/rename_specs.py`.

```python
#!/usr/bin/env python3
"""Prefix finished specs with their zero-padded order number."""
import argparse
import re
import sys
from pathlib import Path

import yaml

BASE_DIR = Path('/home/user/xyan/xy.ai.workbench/project/resident_02/23_batch')
MIN_WIDTH = 3


def slugify(name: str) -> str:
    slug = re.sub(r'[^\w]+', '_', name.strip(), flags=re.UNICODE).strip('_')
    return slug.lower() or 'module'


def find_spec(spec_dir: Path, slug: str) -> Path | None:
    plain = spec_dir / f'{slug}.md'
    if plain.is_file():
        return plain
    pattern = re.compile(rf'^\d+_{re.escape(slug)}\.md$')
    matches = [p for p in spec_dir.iterdir() if p.is_file() and pattern.match(p.name)]
    return matches[0] if matches else None


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--modules', type=Path, default=BASE_DIR / 'modules.yaml')
    parser.add_argument('--spec-dir', type=Path, default=BASE_DIR / 'specs')
    parser.add_argument('--width', type=int, default=None,
                        help=f'padding width (default: max({MIN_WIDTH}, digits of highest order))')
    parser.add_argument('--dry-run', action='store_true')
    args = parser.parse_args()

    modules = yaml.safe_load(args.modules.read_text(encoding='utf-8'))
    modules.sort(key=lambda m: m.get('Order', 0))
    highest = max((int(m.get('Order', 0)) for m in modules), default=0)
    width = args.width or max(MIN_WIDTH, len(str(highest)))

    plan = []
    for module in modules:
        name = module['Name']
        source = find_spec(args.spec_dir, slugify(name))
        if source is None:
            print(f'[skip] {name}: no spec found')
            continue
        target = args.spec_dir / f'{int(module.get("Order", 0)):0{width}d}_{slugify(name)}.md'
        if source == target:
            print(f'[same] {target.name}')
            continue
        plan.append((name, source, target))

    targets = [t for _, _, t in plan]
    if len(set(targets)) != len(targets):
        print('[fail] duplicate target names (duplicate Order values?)', file=sys.stderr)
        return 1
    sources = {s for _, s, _ in plan}
    for _, _, target in plan:
        if target.exists() and target not in sources:
            print(f'[fail] target already exists: {target}', file=sys.stderr)
            return 1

    # two-phase rename avoids collisions when re-padding existing prefixes
    staged = []
    for name, source, target in plan:
        print(f'[ren ] {source.name} -> {target.name}')
        if args.dry_run:
            continue
        tmp = source.with_name(f'{source.name}.tmp_rename')
        source.rename(tmp)
        staged.append((tmp, target))
    for tmp, target in staged:
        tmp.rename(target)
    return 0


if __name__ == '__main__':
    sys.exit(main())
```

- **Reihenfolge:** Sie kommt aus dem Feld `Order` in `modules.yaml`, genau wie in `batch_specs.py`. Das Prefix ist diese Ordnungsnummer, zum Beispiel `003_modulname.md`.
- **Padding:** Mindestens 3 Stellen. Bei Order-Werten ab 1000 wächst es automatisch, oder du setzt es mit `--width`.
- **Mehrfaches Ausführen:** Das Script kann wiederholt laufen. Es erkennt schon geprefixte Dateien und passt sie bei geänderter Order oder Breite an.
- **Prüfung vorab:** Mit `--dry-run` siehst du nur die geplanten Umbenennungen. Bei doppelten Order-Werten oder bereits belegten Zielnamen bricht es ab, bevor etwas umbenannt wird.
- **Folge für `batch_specs.py`:** Dessen `spec_path` sucht nur `<slug>.md`. Nach dem Umbenennen gelten die Specs bei einem weiteren Lauf als fehlend und würden neu generiert. Wenn du `batch_specs.py` danach noch einmal laufen lassen willst, muss dort `spec_path` das Prefix mit berücksichtigen.
