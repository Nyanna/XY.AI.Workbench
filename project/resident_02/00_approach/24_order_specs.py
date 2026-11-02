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