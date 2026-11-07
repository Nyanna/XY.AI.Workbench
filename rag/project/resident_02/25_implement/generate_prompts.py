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
